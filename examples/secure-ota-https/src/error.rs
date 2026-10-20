// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Transfer failures. Call [`StageError::log`] once at the wait loop.

use embassy_time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub enum StageError {
    WifiAssoc,
    WifiDhcp,
    WifiCreds,
    Dns,
    Untrusted,
    Handshake,
    Timeout,
    /// `0` means the URL, headers, or status line were rejected.
    Http(u16),
    Crc,
    Flash,
}

impl StageError {
    pub fn log(self) {
        match self {
            Self::WifiAssoc => log::warn!("Wi-Fi associate failed or timed out"),
            Self::WifiDhcp => log::warn!("DHCP timed out"),
            Self::WifiCreds => log::error!("Wi-Fi credentials rejected"),
            Self::Dns => log::warn!("DNS failed"),
            Self::Untrusted => log::warn!("TLS certificate rejected"),
            Self::Handshake => log::warn!("TLS handshake failed"),
            Self::Timeout => log::warn!("transfer timed out"),
            Self::Http(0) => log::warn!("HTTP response rejected"),
            Self::Http(code) => log::warn!("HTTP status {code}"),
            Self::Crc => log::warn!("image CRC, length, or magic rejected"),
            Self::Flash => log::error!("flash or otadata error"),
        }
    }
}

pub fn time_left(deadline: Instant) -> Result<Duration, StageError> {
    let now = Instant::now();
    if now >= deadline {
        Err(StageError::Timeout)
    } else {
        Ok(deadline - now)
    }
}
