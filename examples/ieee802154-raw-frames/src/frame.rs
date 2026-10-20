// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Lab payload shared by the node and the peer. Not a network-layer frame.

#[cfg(feature = "role-node")]
use alloc::vec::Vec;

/// Magic that marks a frame from this example. Unrelated 802.15.4 traffic is dropped.
pub const MAGIC: &[u8] = b"C6RAW";

const SEQ_INDEX: usize = 5;
const PAYLOAD_LEN: usize = 6;

/// Magic plus one sequence byte.
#[cfg(feature = "role-node")]
pub fn payload_bytes(seq: u8) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(PAYLOAD_LEN);
    bytes.extend_from_slice(MAGIC);
    bytes.push(seq);
    bytes
}

pub fn is_ours(payload: &[u8]) -> bool {
    payload.len() == PAYLOAD_LEN && payload.starts_with(MAGIC)
}

pub fn seq_of(payload: &[u8]) -> Option<u8> {
    if is_ours(payload) {
        Some(payload[SEQ_INDEX])
    } else {
        None
    }
}
