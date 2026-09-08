// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Hardware AES-256-CBC via `esp_hal::aes`. The work-queue backend must stay
//! started for the lifetime of these calls.

use core::ptr::addr_of_mut;

use esp_hal::aes::cipher_modes::Cbc;
use esp_hal::aes::{AesContext, Operation};
use esp_hal::rng::Rng;
use esp_hal::time::Instant;

pub const BLOCK: usize = 16;
pub const KEY_LEN: usize = 32;
pub const BENCH_LEN: usize = 64 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CryptoError {
    Buffer,
    Length,
    Padding,
    Aes,
}

pub fn padded_len(n: usize) -> usize {
    let rem = n % BLOCK;
    n + (BLOCK - rem)
}

pub fn pkcs7_pad(src: &[u8], dst: &mut [u8]) -> Result<usize, CryptoError> {
    let out_len = padded_len(src.len());
    if dst.len() < out_len {
        return Err(CryptoError::Buffer);
    }
    dst[..src.len()].copy_from_slice(src);
    let pad = (out_len - src.len()) as u8;
    for byte in dst.iter_mut().take(out_len).skip(src.len()) {
        *byte = pad;
    }
    Ok(out_len)
}

pub fn pkcs7_unpad(buf: &[u8]) -> Result<&[u8], CryptoError> {
    if buf.is_empty() || !buf.len().is_multiple_of(BLOCK) {
        return Err(CryptoError::Padding);
    }
    let pad = buf[buf.len() - 1];
    let pad_n = pad as usize;
    if pad_n == 0 || pad_n > BLOCK || pad_n > buf.len() {
        return Err(CryptoError::Padding);
    }
    let start = buf.len() - pad_n;
    if buf[start..].iter().any(|&b| b != pad) {
        return Err(CryptoError::Padding);
    }
    Ok(&buf[..start])
}

pub fn fill_random(buf: &mut [u8]) {
    Rng::new().read(buf);
}

pub fn encrypt_cbc(
    key: &[u8; KEY_LEN],
    iv: [u8; BLOCK],
    input: &[u8],
    output: &mut [u8],
) -> Result<(), CryptoError> {
    transform(key, iv, Operation::Encrypt, input, output)
}

pub fn decrypt_cbc(
    key: &[u8; KEY_LEN],
    iv: [u8; BLOCK],
    input: &[u8],
    output: &mut [u8],
) -> Result<(), CryptoError> {
    transform(key, iv, Operation::Decrypt, input, output)
}

fn transform(
    key: &[u8; KEY_LEN],
    iv: [u8; BLOCK],
    op: Operation,
    input: &[u8],
    output: &mut [u8],
) -> Result<(), CryptoError> {
    if input.len() != output.len() || input.is_empty() || !input.len().is_multiple_of(BLOCK) {
        return Err(CryptoError::Length);
    }
    let mut ctx = AesContext::new(Cbc::new(iv), op, *key);
    let handle = ctx.process(input, output).map_err(|_| CryptoError::Aes)?;
    // CPU backend completes inline; the handle type's Status is in a private module.
    let _ = handle.wait_blocking();
    Ok(())
}

/// Encrypt 64 KiB and return (whole, milli) MB/s parts.
pub fn throughput_mbps_parts(key: &[u8; KEY_LEN]) -> Result<(u32, u32), CryptoError> {
    let (input, output) = bench_bufs();
    fill_random(input);
    let mut iv = [0u8; BLOCK];
    fill_random(&mut iv);
    let start = Instant::now();
    encrypt_cbc(key, iv, input, output)?;
    let us = start.elapsed().as_micros().max(1);
    let milli = (BENCH_LEN as u64 * 1000) / us;
    Ok(((milli / 1000) as u32, (milli % 1000) as u32))
}

fn bench_bufs() -> (&'static mut [u8; BENCH_LEN], &'static mut [u8; BENCH_LEN]) {
    #[repr(align(16))]
    struct Bench([u8; BENCH_LEN]);
    static mut IN: Bench = Bench([0; BENCH_LEN]);
    static mut OUT: Bench = Bench([0; BENCH_LEN]);
    // Single-core; called once from `main` after AES backend start.
    unsafe { (&mut (*addr_of_mut!(IN)).0, &mut (*addr_of_mut!(OUT)).0) }
}
