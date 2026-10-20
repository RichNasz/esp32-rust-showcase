// AGENT-GENERATED — DO NOT EDIT BY HAND
//! IEEE 802.15.4 raw data frames. No Wi-Fi, no Thread, no Zigbee.

use alloc::vec::Vec;
use embassy_time::Duration;
#[cfg(feature = "role-node")]
use embassy_time::Instant;
use embassy_time::Timer;
use esp_hal::peripherals::IEEE802154;
use esp_radio::ieee802154::{Config, Frame, Ieee802154};
use ieee802154::mac::{
    Address, FrameContent, FrameType, FrameVersion, Header, PanId, ShortAddress,
};

use crate::frame;
use crate::led::StatusLed;

const CHANNEL_RAW: &str = env!("IEEE802154_CHANNEL");
const PAN_RAW: &str = env!("IEEE802154_PAN_ID");

#[cfg(feature = "role-node")]
const ACK_WAIT: Duration = Duration::from_millis(200);
const POLL: Duration = Duration::from_millis(20);
#[cfg(feature = "role-node")]
const PERIOD: Duration = Duration::from_secs(2);

#[cfg(feature = "role-node")]
const LOCAL_SHORT: u16 = 0x2222;
#[cfg(feature = "role-node")]
const PEER_SHORT: u16 = 0x2323;

#[cfg(feature = "role-peer")]
const LOCAL_SHORT: u16 = 0x2323;
#[cfg(feature = "role-peer")]
const PEER_SHORT: u16 = 0x2222;

pub const CHANNEL: u8 = match parse_channel(CHANNEL_RAW) {
    Some(channel) => channel,
    None => panic!("IEEE802154_CHANNEL must be a decimal channel from 11 through 26"),
};

pub const PAN_ID: u16 = match parse_pan(PAN_RAW) {
    Some(pan) => pan,
    None => panic!("IEEE802154_PAN_ID must be a 16-bit hex value, for example 0x4242"),
};

const fn hex_val(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

const fn parse_u16_dec(bytes: &[u8]) -> Option<u16> {
    if bytes.is_empty() {
        return None;
    }
    let mut value: u32 = 0;
    let mut index = 0;
    while index < bytes.len() {
        let digit = bytes[index];
        if digit < b'0' || digit > b'9' {
            return None;
        }
        value = value * 10 + (digit - b'0') as u32;
        if value > u16::MAX as u32 {
            return None;
        }
        index += 1;
    }
    Some(value as u16)
}

const fn parse_channel(text: &str) -> Option<u8> {
    let Some(value) = parse_u16_dec(text.as_bytes()) else {
        return None;
    };
    if value >= 11 && value <= 26 {
        Some(value as u8)
    } else {
        None
    }
}

const fn parse_pan(text: &str) -> Option<u16> {
    let bytes = text.as_bytes();
    let mut start = 0;
    if bytes.len() >= 2 && bytes[0] == b'0' && (bytes[1] == b'x' || bytes[1] == b'X') {
        start = 2;
    }
    let len = bytes.len() - start;
    if len == 0 || len > 4 {
        return None;
    }
    let mut value: u32 = 0;
    let mut index = start;
    while index < bytes.len() {
        let Some(digit) = hex_val(bytes[index]) else {
            return None;
        };
        value = (value << 4) | digit as u32;
        index += 1;
    }
    Some(value as u16)
}

fn data_frame(seq: u8, dest: u16, ack_request: bool, payload: Vec<u8>) -> Frame {
    Frame {
        header: Header {
            frame_type: FrameType::Data,
            frame_pending: false,
            ack_request,
            pan_id_compress: false,
            seq_no_suppress: false,
            ie_present: false,
            version: FrameVersion::Ieee802154_2003,
            seq,
            destination: Some(Address::Short(PanId(PAN_ID), ShortAddress(dest))),
            source: Some(Address::Short(PanId(PAN_ID), ShortAddress(LOCAL_SHORT))),
            auxiliary_security_header: None,
        },
        content: FrameContent::Data,
        payload,
        footer: [0u8; 2],
    }
}

fn bring_up(radio: IEEE802154<'static>) -> Ieee802154<'static> {
    let mut ieee = Ieee802154::new(radio);
    ieee.set_config(Config {
        channel: CHANNEL,
        pan_id: Some(PAN_ID),
        short_addr: Some(LOCAL_SHORT),
        rx_when_idle: true,
        auto_ack_rx: true,
        auto_ack_tx: true,
        promiscuous: false,
        ..Default::default()
    });
    ieee.start_receive();
    log::info!(
        "802.15.4 up — channel {CHANNEL} pan {PAN_ID:#06x} short {LOCAL_SHORT:#06x} peer {PEER_SHORT:#06x}"
    );
    ieee
}

#[cfg(feature = "role-node")]
fn log_received(role: &str, ieee: &mut Ieee802154<'_>) {
    while let Some(item) = ieee.received() {
        match item {
            Ok(received) => match frame::seq_of(&received.frame.payload) {
                Some(seq) => log::info!(
                    "{role} frame seq {seq} rssi {} lqi {}",
                    received.rssi,
                    received.lqi
                ),
                None => log::warn!("{role} dropped frame"),
            },
            Err(err) => log::warn!("{role} decode: {err}"),
        }
    }
}

#[cfg(feature = "role-node")]
pub async fn node_forever(radio: IEEE802154<'static>, led: &mut StatusLed) -> ! {
    let mut ieee = bring_up(radio);
    let mut seq = 0u8;
    loop {
        let frame = data_frame(seq, PEER_SHORT, true, frame::payload_bytes(seq));
        let sent = match ieee.transmit(&frame, true) {
            Ok(()) => {
                log::info!("node tx seq {seq}");
                true
            }
            Err(err) => {
                log::error!("node tx seq {seq} failed: {err}");
                false
            }
        };
        let acked = if sent {
            wait_ack(&mut ieee).await
        } else {
            false
        };
        if acked {
            log::info!("node seq {seq} ack");
            led.success().await;
        } else {
            log::info!("node seq {seq} no ack");
            led.failure().await;
        }
        log_received("node", &mut ieee);
        seq = seq.wrapping_add(1);
        Timer::after(PERIOD).await;
    }
}

#[cfg(feature = "role-node")]
async fn wait_ack(ieee: &mut Ieee802154<'_>) -> bool {
    let deadline = Instant::now() + ACK_WAIT;
    loop {
        log_received("node", ieee);
        if ieee.get_ack_frame().is_some() {
            return true;
        }
        if Instant::now() >= deadline {
            return false;
        }
        Timer::after(POLL).await;
    }
}

#[cfg(feature = "role-peer")]
pub async fn peer_forever(radio: IEEE802154<'static>, led: &mut StatusLed) -> ! {
    let mut ieee = bring_up(radio);
    let mut echo_seq = 0u8;
    loop {
        match ieee.received() {
            Some(Ok(received)) => match frame::seq_of(&received.frame.payload) {
                Some(seq) => {
                    log::info!(
                        "peer rx seq {seq} rssi {} lqi {}",
                        received.rssi,
                        received.lqi
                    );
                    let echo = data_frame(echo_seq, PEER_SHORT, false, received.frame.payload);
                    match ieee.transmit(&echo, true) {
                        Ok(()) => log::info!("peer echo seq {seq}"),
                        Err(err) => log::error!("peer echo failed: {err}"),
                    }
                    echo_seq = echo_seq.wrapping_add(1);
                    led.accept().await;
                }
                None => log::warn!("peer dropped frame"),
            },
            Some(Err(err)) => log::warn!("peer decode: {err}"),
            None => Timer::after(POLL).await,
        }
    }
}
