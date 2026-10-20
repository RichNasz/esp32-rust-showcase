// AGENT-GENERATED — DO NOT EDIT BY HAND
//! One ESP-NOW payload layout for both roles. Magic lets the gateway ignore
//! unrelated frames. Boot count is little-endian. Eight bytes, under the
//! 250-byte ESP-NOW maximum.

/// Lab magic. Not a key.
pub const MAGIC: [u8; 4] = [0xC6, 0xE5, 0x4E, 0x57];

pub const LEN: usize = 8;

const _: () = assert!(LEN <= esp_radio::esp_now::ESP_NOW_MAX_DATA_LEN_V1);

#[cfg_attr(not(feature = "role-sensor"), allow(dead_code))]
pub fn encode(boot: u32) -> [u8; LEN] {
    let mut out = [0u8; LEN];
    out[..4].copy_from_slice(&MAGIC);
    out[4..].copy_from_slice(&boot.to_le_bytes());
    out
}

/// Boot count when the leading magic matches. Anything else is not our frame.
#[cfg_attr(not(feature = "role-gateway"), allow(dead_code))]
pub fn decode(data: &[u8]) -> Option<u32> {
    if data.len() < LEN || data[..4] != MAGIC {
        return None;
    }
    let mut bytes = [0u8; 4];
    bytes.copy_from_slice(&data[4..8]);
    Some(u32::from_le_bytes(bytes))
}
