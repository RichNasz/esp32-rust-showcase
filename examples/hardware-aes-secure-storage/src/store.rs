// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Flash-backed key and blob in the dedicated `storage` partition (not IDF NVS).
//! Offsets match `partitions.csv` (base 0x110000).

use esp_storage::{FlashStorage, FlashStorageError};

use crate::crypto::{self, BLOCK, KEY_LEN};

/// Dedicated `storage` partition start (see `partitions.csv`).
pub const STORAGE_BASE: u32 = 0x110000;
const SECTOR: u32 = FlashStorage::SECTOR_SIZE;
const KEY_OFF: u32 = STORAGE_BASE;
const BLOB_OFF: u32 = STORAGE_BASE + SECTOR;

const KEY_MAGIC: u32 = 0xC6AE_5E01;
const BLOB_MAGIC: u32 = 0xC6AE_51B0;
const KEY_REC_LEN: usize = 40;
const BLOB_HDR: usize = 24;
const BLOB_CAP: usize = 128;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreError {
    Flash,
    Corrupt,
    TooLarge,
}

impl From<FlashStorageError> for StoreError {
    fn from(_: FlashStorageError) -> Self {
        StoreError::Flash
    }
}

#[repr(align(4))]
struct Rec([u8; BLOB_CAP]);

pub fn load_or_create_key(
    flash: &mut FlashStorage<'_>,
) -> Result<([u8; KEY_LEN], bool), StoreError> {
    if let Some(key) = load_key(flash)? {
        return Ok((key, false));
    }
    let mut key = [0u8; KEY_LEN];
    crypto::fill_random(&mut key);
    save_key(flash, &key)?;
    Ok((key, true))
}

fn load_key(flash: &mut FlashStorage<'_>) -> Result<Option<[u8; KEY_LEN]>, StoreError> {
    let mut rec = Rec([0; BLOB_CAP]);
    flash.read_nor(KEY_OFF, &mut rec.0[..KEY_REC_LEN])?;
    let magic = u32::from_le_bytes(rec.0[0..4].try_into().unwrap_or([0; 4]));
    if magic != KEY_MAGIC {
        return Ok(None);
    }
    let mut key = [0u8; KEY_LEN];
    key.copy_from_slice(&rec.0[4..36]);
    let stored = u32::from_le_bytes(rec.0[36..40].try_into().unwrap_or([0; 4]));
    if stored != key_checksum(&key) {
        return Err(StoreError::Corrupt);
    }
    Ok(Some(key))
}

fn save_key(flash: &mut FlashStorage<'_>, key: &[u8; KEY_LEN]) -> Result<(), StoreError> {
    let mut rec = Rec([0; BLOB_CAP]);
    rec.0[0..4].copy_from_slice(&KEY_MAGIC.to_le_bytes());
    rec.0[4..36].copy_from_slice(key);
    rec.0[36..40].copy_from_slice(&key_checksum(key).to_le_bytes());
    flash.erase(KEY_OFF, KEY_OFF + SECTOR)?;
    flash.write_nor(KEY_OFF, &rec.0[..KEY_REC_LEN])?;
    Ok(())
}

fn key_checksum(key: &[u8; KEY_LEN]) -> u32 {
    let mut acc = 0u32;
    for chunk in key.chunks_exact(4) {
        acc ^= u32::from_le_bytes(chunk.try_into().unwrap_or([0; 4]));
    }
    acc
}

pub fn write_blob(
    flash: &mut FlashStorage<'_>,
    iv: &[u8; BLOCK],
    ciphertext: &[u8],
) -> Result<(), StoreError> {
    if BLOB_HDR + ciphertext.len() > BLOB_CAP {
        return Err(StoreError::TooLarge);
    }
    let mut rec = Rec([0; BLOB_CAP]);
    rec.0[0..4].copy_from_slice(&BLOB_MAGIC.to_le_bytes());
    rec.0[4..20].copy_from_slice(iv);
    rec.0[20..24].copy_from_slice(&(ciphertext.len() as u32).to_le_bytes());
    rec.0[24..24 + ciphertext.len()].copy_from_slice(ciphertext);
    flash.erase(BLOB_OFF, BLOB_OFF + SECTOR)?;
    flash.write_nor(BLOB_OFF, &rec.0)?;
    Ok(())
}

pub fn read_blob(
    flash: &mut FlashStorage<'_>,
    iv: &mut [u8; BLOCK],
    ciphertext: &mut [u8],
) -> Result<usize, StoreError> {
    let mut rec = Rec([0; BLOB_CAP]);
    flash.read_nor(BLOB_OFF, &mut rec.0)?;
    let magic = u32::from_le_bytes(rec.0[0..4].try_into().unwrap_or([0; 4]));
    if magic != BLOB_MAGIC {
        return Err(StoreError::Corrupt);
    }
    iv.copy_from_slice(&rec.0[4..20]);
    let ct_len = u32::from_le_bytes(rec.0[20..24].try_into().unwrap_or([0; 4])) as usize;
    if ct_len == 0 || BLOB_HDR + ct_len > BLOB_CAP || ct_len > ciphertext.len() {
        return Err(StoreError::Corrupt);
    }
    ciphertext[..ct_len].copy_from_slice(&rec.0[24..24 + ct_len]);
    Ok(ct_len)
}
