// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Embed the lab CA as DER and export the lab clock. `linkall.x` stays last.

use std::env;
use std::fs;
use std::path::PathBuf;

/// Inside `certs/placeholder-ca.pem` (notBefore 2026-10-20 13:14:40 UTC).
const DEFAULT_NOW_UNIX: &str = "1792504800";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=WIFI_SSID");
    println!("cargo:rerun-if-env-changed=WIFI_PASSWORD");
    println!("cargo:rerun-if-env-changed=OTA_URL");
    println!("cargo:rerun-if-env-changed=OTA_CA_PATH");
    println!("cargo:rerun-if-env-changed=OTA_NOW_UNIX");

    let now = env::var("OTA_NOW_UNIX").unwrap_or_else(|_| DEFAULT_NOW_UNIX.to_string());
    if now.parse::<u64>().is_err() {
        panic!("OTA_NOW_UNIX must be a unix-seconds integer");
    }
    println!("cargo:rustc-env=OTA_NOW_UNIX={now}");

    let pem_path = ca_pem_path();
    println!("cargo:rerun-if-changed={}", pem_path.display());
    let pem = fs::read_to_string(&pem_path)
        .unwrap_or_else(|err| panic!("read CA PEM {}: {err}", pem_path.display()));
    if pem.contains("PRIVATE KEY") {
        panic!("OTA CA PEM must be a certificate, not a private key");
    }
    let der = pem_to_der(&pem);
    if der.len() < 64 {
        panic!("CA PEM did not contain a certificate");
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap_or_else(|err| panic!("{err}")));
    fs::write(out.join("ca.der"), &der).unwrap_or_else(|err| panic!("write ca.der: {err}"));

    // Emit last so linkall.x is the last linker script and the crate links.
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

fn ca_pem_path() -> PathBuf {
    let manifest = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|err| panic!("{err}")));
    match env::var("OTA_CA_PATH") {
        Ok(value) if !value.is_empty() => {
            let path = PathBuf::from(value);
            if path.is_absolute() {
                path
            } else {
                manifest.join(path)
            }
        }
        _ => manifest.join("certs/placeholder-ca.pem"),
    }
}

fn pem_to_der(pem: &str) -> Vec<u8> {
    let mut b64 = String::new();
    for line in pem.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("-----") {
            continue;
        }
        b64.push_str(line);
    }
    decode_b64(&b64)
}

fn decode_b64(input: &str) -> Vec<u8> {
    fn val(byte: u8) -> u8 {
        match byte {
            b'A'..=b'Z' => byte - b'A',
            b'a'..=b'z' => byte - b'a' + 26,
            b'0'..=b'9' => byte - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => 0xFF,
        }
    }

    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 4 * 3);
    let mut i = 0;
    while i + 4 <= bytes.len() {
        let n = [
            val(bytes[i]),
            val(bytes[i + 1]),
            val(bytes[i + 2]),
            val(bytes[i + 3]),
        ];
        if n.contains(&0xFF) {
            panic!("CA PEM is not valid base64");
        }
        out.push((n[0] << 2) | (n[1] >> 4));
        if bytes[i + 2] != b'=' {
            out.push((n[1] << 4) | (n[2] >> 2));
        }
        if bytes[i + 3] != b'=' {
            out.push((n[2] << 6) | n[3]);
        }
        i += 4;
    }
    out
}
