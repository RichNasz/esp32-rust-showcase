// AGENT-GENERATED — DO NOT EDIT BY HAND
//! ESP-NOW on one fixed channel. No AP association and no embassy-net.
//! The sensor sends one datagram, then drops the controller so the radio
//! powers down before deep sleep. The gateway stays in receive.

use embassy_time::Duration;
#[cfg(feature = "role-sensor")]
use embassy_time::{Instant, with_timeout};
#[cfg(feature = "role-sensor")]
use esp_radio::esp_now::{EspNowWifiInterface, PeerInfo};
use esp_radio::wifi::{ControllerConfig, WifiController};

use crate::frame;
use crate::led::StatusLed;

/// Send-status wait. The sensor does not block past this.
#[cfg(feature = "role-sensor")]
const SEND_WAIT: Duration = Duration::from_millis(200);

const CHANNEL_STR: &str = env!("ESPNOW_CHANNEL");

/// Both images must use this channel. A mismatch looks like a dead gateway.
pub const CHANNEL: u8 = match parse_channel(CHANNEL_STR.as_bytes()) {
    Some(channel) => channel,
    None => panic!("ESPNOW_CHANNEL must be a decimal channel from 1 to 14"),
};

#[cfg(feature = "role-sensor")]
const GATEWAY_MAC_STR: &str = env!("ESPNOW_GATEWAY_MAC");

#[cfg(feature = "role-sensor")]
const GATEWAY_MAC: [u8; 6] = match parse_mac(GATEWAY_MAC_STR.as_bytes()) {
    Some(mac) => mac,
    None => panic!("ESPNOW_GATEWAY_MAC must be six hex octets, with or without colons"),
};

/// One unicast, then blink, then drop ESP-NOW and the Wi-Fi controller.
#[cfg(feature = "role-sensor")]
pub async fn sensor_once(
    wifi: esp_hal::peripherals::WIFI<'static>,
    led: &mut StatusLed,
    boot: u32,
    deadline: Instant,
) {
    log_mac("gateway MAC", GATEWAY_MAC);
    if gateway_unset(GATEWAY_MAC) {
        log::warn!("ESPNOW_GATEWAY_MAC is unset; skipping send");
        led.failure().await;
        led.off().await;
        return;
    }

    let controller = match WifiController::new(wifi, ControllerConfig::default()) {
        Ok(controller) => controller,
        Err(_) => {
            log::error!("Wi-Fi controller init failed");
            led.failure().await;
            led.off().await;
            return;
        }
    };
    let mut esp_now = controller.esp_now();
    let sent = send_one(&mut esp_now, boot, deadline).await;
    if sent {
        led.success().await;
    } else {
        led.failure().await;
    }
    led.off().await;
    drop(esp_now);
    drop(controller);
    log::info!("radio teardown");
    if Instant::now() > deadline {
        log::warn!("active window exceeded 3 s");
    }
}

#[cfg(feature = "role-sensor")]
async fn send_one(
    esp_now: &mut esp_radio::esp_now::EspNow,
    boot: u32,
    deadline: Instant,
) -> bool {
    if esp_now.set_channel(CHANNEL).is_err() {
        log::error!("ESP-NOW channel {CHANNEL} rejected");
        return false;
    }
    let peer = PeerInfo {
        interface: EspNowWifiInterface::Station,
        peer_address: GATEWAY_MAC,
        lmk: None,
        channel: Some(CHANNEL),
        encrypt: false,
    };
    if esp_now.add_peer(peer).is_err() {
        log::error!("ESP-NOW add_peer failed");
        return false;
    }
    let Some(left) = remaining(deadline) else {
        log::warn!("active window expired before send  boot={boot}");
        return false;
    };
    let payload = frame::encode(boot);
    let wait = if left < SEND_WAIT { left } else { SEND_WAIT };
    match with_timeout(wait, esp_now.send_async(&GATEWAY_MAC, &payload)).await {
        Ok(Ok(())) => {
            log::info!("send ok  boot={boot}");
            true
        }
        Ok(Err(_)) => {
            log::warn!("send failed  boot={boot}");
            false
        }
        Err(_) => {
            log::warn!("send timed out  boot={boot}");
            false
        }
    }
}

/// Log the station MAC and stay in receive. Does not sleep.
#[cfg(feature = "role-gateway")]
pub async fn gateway_forever(wifi: esp_hal::peripherals::WIFI<'static>, led: &mut StatusLed) -> ! {
    let controller = match WifiController::new(wifi, ControllerConfig::default()) {
        Ok(controller) => controller,
        Err(_) => {
            log::error!("Wi-Fi controller init failed");
            halt().await
        }
    };
    let mac = esp_radio::wifi::Interface::station().mac_address();
    log_mac("station MAC", mac);
    log::info!("set ESPNOW_GATEWAY_MAC on the sensor to that address");

    let mut esp_now = controller.esp_now();
    if esp_now.set_channel(CHANNEL).is_err() {
        log::error!("ESP-NOW channel {CHANNEL} rejected");
        halt().await
    }

    loop {
        let packet = esp_now.receive_async().await;
        let src = packet.info.src_address;
        match frame::decode(packet.data()) {
            Some(boot) => {
                log::info!("rx src={} boot={boot}", MacAddr(src));
                led.accept().await;
            }
            None => {
                log::warn!(
                    "drop frame from {} len={} (magic mismatch)",
                    MacAddr(src),
                    packet.data().len()
                );
            }
        }
    }
}

struct MacAddr([u8; 6]);

impl core::fmt::Display for MacAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

pub fn log_mac(label: &str, mac: [u8; 6]) {
    log::info!("{label} {}", MacAddr(mac));
}

#[cfg(feature = "role-sensor")]
fn remaining(deadline: Instant) -> Option<Duration> {
    let now = Instant::now();
    (now < deadline).then(|| deadline - now)
}

#[cfg(feature = "role-sensor")]
const fn gateway_unset(mac: [u8; 6]) -> bool {
    mac[0] == 0 && mac[1] == 0 && mac[2] == 0 && mac[3] == 0 && mac[4] == 0 && mac[5] == 0
}

const fn parse_channel(bytes: &[u8]) -> Option<u8> {
    if bytes.is_empty() || bytes.len() > 2 {
        return None;
    }
    let mut value: u16 = 0;
    let mut index = 0;
    while index < bytes.len() {
        let digit = bytes[index];
        if digit < b'0' || digit > b'9' {
            return None;
        }
        value = value * 10 + (digit - b'0') as u16;
        index += 1;
    }
    if value >= 1 && value <= 14 {
        Some(value as u8)
    } else {
        None
    }
}

#[cfg(feature = "role-sensor")]
const fn parse_mac(bytes: &[u8]) -> Option<[u8; 6]> {
    if bytes.len() == 17 {
        parse_mac_separated(bytes)
    } else if bytes.len() == 12 {
        parse_mac_raw(bytes)
    } else {
        None
    }
}

#[cfg(feature = "role-sensor")]
const fn parse_mac_separated(bytes: &[u8]) -> Option<[u8; 6]> {
    let mut mac = [0u8; 6];
    let mut index = 0;
    while index < 6 {
        let at = index * 3;
        let Some(hi) = hex_val(bytes[at]) else {
            return None;
        };
        let Some(lo) = hex_val(bytes[at + 1]) else {
            return None;
        };
        if index < 5 && bytes[at + 2] != b':' {
            return None;
        }
        mac[index] = (hi << 4) | lo;
        index += 1;
    }
    Some(mac)
}

#[cfg(feature = "role-sensor")]
const fn parse_mac_raw(bytes: &[u8]) -> Option<[u8; 6]> {
    let mut mac = [0u8; 6];
    let mut index = 0;
    while index < 6 {
        let Some(hi) = hex_val(bytes[index * 2]) else {
            return None;
        };
        let Some(lo) = hex_val(bytes[index * 2 + 1]) else {
            return None;
        };
        mac[index] = (hi << 4) | lo;
        index += 1;
    }
    Some(mac)
}

#[cfg(feature = "role-sensor")]
const fn hex_val(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(feature = "role-gateway")]
async fn halt() -> ! {
    use embassy_time::Timer;
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
