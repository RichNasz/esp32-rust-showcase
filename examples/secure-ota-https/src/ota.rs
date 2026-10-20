// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Stream an image into the inactive OTA slot. CRC and magic are checked
//! before otadata changes. Flash writes run while the radio task is alive;
//! esp-storage wraps the ROM call in a critical section, and a Wi-Fi
//! interrupt whose handler lives in flash can still fault. The image is not
//! buffered whole to avoid that.

use embassy_net::Stack;
use embassy_time::{Duration, Instant};
use esp_bootloader_esp_idf::ota::OtaImageState;
use esp_bootloader_esp_idf::ota_updater::OtaUpdater;
use esp_bootloader_esp_idf::partitions::{
    AppPartitionSubType, FlashRegion, PartitionEntry, PartitionTable, PartitionType,
    PARTITION_TABLE_MAX_LEN,
};
use esp_storage::FlashStorage;

use crate::error::StageError;
use crate::https::{self, Prefix, Scratch};
use crate::led::{SlotColour, StatusLed};

const IMAGE_MAGIC: u8 = 0xE9;
const SECTOR: u32 = 4096;
const PROGRESS: u32 = 64 * 1024;
const TRANSFER_SECS: u64 = 60;

#[derive(Clone, Copy, Debug)]
pub enum SlotKind {
    Ota0,
    Ota1,
    Other,
}

impl SlotKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Ota0 => "ota_0",
            Self::Ota1 => "ota_1",
            Self::Other => "other",
        }
    }
}

pub fn colour(kind: SlotKind) -> SlotColour {
    match kind {
        SlotKind::Ota0 => SlotColour::Ota0,
        SlotKind::Ota1 => SlotColour::Ota1,
        SlotKind::Other => SlotColour::Other,
    }
}

pub struct SlotMap {
    pub booted: SlotKind,
    pub booted_offset: u32,
    pub ota0_offset: u32,
    pub ota1_offset: u32,
    pub slot_len: u32,
}

pub fn read_map(flash: &mut FlashStorage<'_>) -> Result<SlotMap, StageError> {
    let mut buffer = [0u8; PARTITION_TABLE_MAX_LEN];
    let table = read_table(flash, &mut buffer)?;
    let ota0 = find_app(&table, AppPartitionSubType::Ota0)?;
    let ota1 = find_app(&table, AppPartitionSubType::Ota1)?;
    let booted = table
        .booted_partition()
        .map_err(|_| StageError::Flash)?;
    let (booted_kind, booted_offset) = match booted {
        Some(entry) => (kind_of_entry(&entry), entry.offset()),
        None => (SlotKind::Other, 0),
    };
    Ok(SlotMap {
        booted: booted_kind,
        booted_offset,
        ota0_offset: ota0.offset(),
        ota1_offset: ota1.offset(),
        slot_len: ota0.len().min(ota1.len()),
    })
}

pub fn prepare(flash: &mut FlashStorage<'_>, map: &SlotMap) -> Result<(), StageError> {
    let mut buffer = [0u8; PARTITION_TABLE_MAX_LEN];
    let mut updater = OtaUpdater::new(flash, &mut buffer).map_err(|_| StageError::Flash)?;
    mark_running_valid(&mut updater);
    pin_empty_otadata(&mut updater, map.booted);
    Ok(())
}

pub async fn run(
    flash: &mut FlashStorage<'static>,
    map: &SlotMap,
    stack: Stack<'static>,
    scratch: &mut Scratch,
    led: &mut StatusLed,
    url: &str,
) -> Result<(), StageError> {
    let mut buffer = [0u8; PARTITION_TABLE_MAX_LEN];
    let mut updater = OtaUpdater::new(flash, &mut buffer).map_err(|_| StageError::Flash)?;
    if !selection_is_ota(&mut updater) {
        return Err(StageError::Flash);
    }
    let deadline = Instant::now() + Duration::from_secs(TRANSFER_SECS);
    let (mut session, prefix) = https::open(stack, scratch, url, deadline).await?;
    log::info!(
        "envelope length={} crc=0x{:08x}",
        prefix.image_len, prefix.crc
    );
    if prefix.image_len == 0 || prefix.image_len > map.slot_len {
        return Err(StageError::Crc);
    }
    let n0 = session.pull().await?;
    if n0 == 0 || session.chunk[0] != IMAGE_MAGIC {
        return Err(StageError::Crc);
    }
    let mut first = [0u8; 4096];
    first[..n0].copy_from_slice(&session.chunk[..n0]);
    let kind = write_slot(&mut updater, map, &mut session, &prefix, &first[..n0], led).await?;
    updater
        .activate_next_partition()
        .map_err(|_| StageError::Flash)?;
    updater
        .set_current_ota_state(OtaImageState::New)
        .map_err(|_| StageError::Flash)?;
    log::info!(
        "CRC ok; activated {} at offset {:#x}",
        kind.name(),
        offset_of(map, kind)
    );
    Ok(())
}

async fn write_slot(
    updater: &mut OtaUpdater<'_, '_>,
    map: &SlotMap,
    session: &mut https::Session<'_>,
    prefix: &Prefix,
    first: &[u8],
    led: &mut StatusLed,
) -> Result<SlotKind, StageError> {
    let (mut region, sub) = updater.next_partition().map_err(|_| StageError::Flash)?;
    let Some(kind) = kind_of_sub(sub) else {
        return Err(StageError::Flash);
    };
    if prefix.image_len as usize > region.capacity() {
        return Err(StageError::Crc);
    }
    log::info!("Erasing inactive {}", kind.name());
    erase_image(&mut region, prefix.image_len, led, colour(map.booted)).await?;
    let mut tail = Tail {
        bytes: [0xFF; 4],
        len: 0,
        offset: 0,
    };
    let mut crc = IeeeCrc32::new();
    let mut got = write_chunk(&mut region, &mut tail, &mut crc, first)? as u32;
    let mut logged = 0u32;
    while session.remaining() > 0 {
        let n = session.pull().await?;
        if n == 0 {
            return Err(StageError::Crc);
        }
        got += write_chunk(&mut region, &mut tail, &mut crc, &session.chunk[..n])? as u32;
        if got / PROGRESS != logged / PROGRESS {
            log::info!("download {got}/{}", prefix.image_len);
            led.pulse_transfer(colour(map.booted)).await;
            logged = got;
        }
    }
    finish_tail(&mut region, &mut tail)?;
    if got != prefix.image_len || crc.finish() != prefix.crc {
        return Err(StageError::Crc);
    }
    Ok(kind)
}

fn read_table<'a>(
    flash: &mut FlashStorage<'_>,
    buffer: &'a mut [u8; PARTITION_TABLE_MAX_LEN],
) -> Result<PartitionTable<'a>, StageError> {
    esp_bootloader_esp_idf::partitions::read_partition_table(flash, buffer)
        .map_err(|_| StageError::Flash)
}

fn find_app(
    table: &PartitionTable<'_>,
    sub: AppPartitionSubType,
) -> Result<PartitionEntry, StageError> {
    table
        .find_partition(PartitionType::App(sub))
        .map_err(|_| StageError::Flash)?
        .ok_or(StageError::Flash)
}

fn kind_of_entry(entry: &PartitionEntry) -> SlotKind {
    match entry.partition_type() {
        PartitionType::App(AppPartitionSubType::Ota0) => SlotKind::Ota0,
        PartitionType::App(AppPartitionSubType::Ota1) => SlotKind::Ota1,
        _ => SlotKind::Other,
    }
}

fn kind_of_sub(sub: AppPartitionSubType) -> Option<SlotKind> {
    match sub {
        AppPartitionSubType::Ota0 => Some(SlotKind::Ota0),
        AppPartitionSubType::Ota1 => Some(SlotKind::Ota1),
        _ => None,
    }
}

fn offset_of(map: &SlotMap, kind: SlotKind) -> u32 {
    match kind {
        SlotKind::Ota0 => map.ota0_offset,
        SlotKind::Ota1 => map.ota1_offset,
        SlotKind::Other => 0,
    }
}

fn selection_is_ota(updater: &mut OtaUpdater<'_, '_>) -> bool {
    match updater.selected_partition() {
        Ok(AppPartitionSubType::Ota0 | AppPartitionSubType::Ota1) => true,
        // Factory::ota_app_number underflows. Never call next_partition on it.
        _ => false,
    }
}

fn mark_running_valid(updater: &mut OtaUpdater<'_, '_>) {
    let Ok(state) = updater.current_ota_state() else {
        return;
    };
    if state != OtaImageState::New && state != OtaImageState::PendingVerify {
        return;
    }
    if updater
        .set_current_ota_state(OtaImageState::Valid)
        .is_ok()
    {
        log::info!("Recorded image state Valid");
    } else {
        log::warn!("could not record image state Valid");
    }
}

fn pin_empty_otadata(updater: &mut OtaUpdater<'_, '_>, booted: SlotKind) {
    let selected = match updater.selected_partition() {
        Ok(selected) => selected,
        Err(_) => {
            log::warn!("otadata unreadable; not changing boot selection");
            return;
        }
    };
    if selected != AppPartitionSubType::Factory {
        return;
    }
    match booted {
        SlotKind::Ota0 => {
            if set_slot(updater, AppPartitionSubType::Ota0) {
                log::info!("initialized otadata; boot selection stays ota_0");
            } else {
                log::warn!("could not initialize otadata");
            }
        }
        SlotKind::Ota1 => {
            // Seq 1 then seq 2. A direct Ota1 write from erased otadata stores
            // sequence 0, which the bootloader does not treat as a selection.
            let ok = set_slot(updater, AppPartitionSubType::Ota0)
                && set_slot(updater, AppPartitionSubType::Ota1);
            if ok {
                log::info!("initialized otadata; boot selection stays ota_1");
            } else {
                log::warn!("otadata empty while booted from ota_1; not resetting");
            }
        }
        SlotKind::Other => {
            log::warn!("otadata empty and booted slot is unknown; not changing otadata");
        }
    }
}

fn set_slot(updater: &mut OtaUpdater<'_, '_>, slot: AppPartitionSubType) -> bool {
    match updater.ota_data() {
        Ok(mut ota) => ota.set_current_app_partition(slot).is_ok(),
        Err(_) => false,
    }
}

async fn erase_image(
    region: &mut FlashRegion<'_, '_>,
    image_len: u32,
    led: &mut StatusLed,
    colour: SlotColour,
) -> Result<(), StageError> {
    let end = image_len.div_ceil(SECTOR) * SECTOR;
    if end == 0 || end as usize > region.capacity() {
        return Err(StageError::Flash);
    }
    let mut at = 0u32;
    let mut marked = 0u32;
    while at < end {
        let next = (at + SECTOR).min(end);
        region.erase(at, next).map_err(|_| StageError::Flash)?;
        at = next;
        if at - marked >= PROGRESS || at == end {
            led.pulse_transfer(colour).await;
            marked = at;
        }
    }
    Ok(())
}

struct Tail {
    bytes: [u8; 4],
    len: usize,
    offset: u32,
}

fn write_chunk(
    region: &mut FlashRegion<'_, '_>,
    tail: &mut Tail,
    crc: &mut IeeeCrc32,
    data: &[u8],
) -> Result<usize, StageError> {
    crc.update(data);
    write_bytes(region, tail, data)?;
    Ok(data.len())
}

/// Copy into an aligned buffer. ROM flash writes reject a 4-byte-unaligned pointer.
/// Padding written by [`finish_tail`] is not part of `data` and is not hashed.
fn write_bytes(
    region: &mut FlashRegion<'_, '_>,
    tail: &mut Tail,
    mut data: &[u8],
) -> Result<(), StageError> {
    if tail.len > 0 {
        let take = (4 - tail.len).min(data.len());
        tail.bytes[tail.len..tail.len + take].copy_from_slice(&data[..take]);
        tail.len += take;
        data = &data[take..];
        if tail.len == 4 {
            region
                .write(tail.offset, &tail.bytes)
                .map_err(|_| StageError::Flash)?;
            tail.offset += 4;
            tail.len = 0;
        }
    }
    let aligned = data.len() - (data.len() % 4);
    write_copied(region, &mut tail.offset, &data[..aligned])?;
    let rest = &data[aligned..];
    if !rest.is_empty() {
        tail.bytes[..rest.len()].copy_from_slice(rest);
        tail.len = rest.len();
    }
    Ok(())
}

fn write_copied(
    region: &mut FlashRegion<'_, '_>,
    offset: &mut u32,
    data: &[u8],
) -> Result<(), StageError> {
    let mut start = 0;
    while start < data.len() {
        let n = (data.len() - start).min(256);
        let n = n - (n % 4);
        let mut buf = [0u8; 256];
        buf[..n].copy_from_slice(&data[start..start + n]);
        region
            .write(*offset, &buf[..n])
            .map_err(|_| StageError::Flash)?;
        *offset += n as u32;
        start += n;
    }
    Ok(())
}

fn finish_tail(region: &mut FlashRegion<'_, '_>, tail: &mut Tail) -> Result<(), StageError> {
    if tail.len == 0 {
        return Ok(());
    }
    for byte in &mut tail.bytes[tail.len..] {
        *byte = 0xFF;
    }
    region
        .write(tail.offset, &tail.bytes)
        .map_err(|_| StageError::Flash)?;
    tail.offset += 4;
    tail.len = 0;
    Ok(())
}

struct IeeeCrc32 {
    state: u32,
}

impl IeeeCrc32 {
    const fn new() -> Self {
        Self {
            state: 0xFFFF_FFFF,
        }
    }

    fn update(&mut self, data: &[u8]) {
        let mut crc = self.state;
        for &byte in data {
            crc ^= u32::from(byte);
            for _ in 0..8 {
                let mask = (crc & 1).wrapping_neg();
                crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
            }
        }
        self.state = crc;
    }

    const fn finish(self) -> u32 {
        !self.state
    }
}

