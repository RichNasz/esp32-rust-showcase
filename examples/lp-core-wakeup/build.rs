// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Builds the nested LP-core crate and points the HP crate at its ELF.
//! Does not shell out to CMake.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=lp-core/Cargo.toml");
    println!("cargo:rerun-if-changed=lp-core/build.rs");
    println!("cargo:rerun-if-changed=lp-core/src/main.rs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let lp_manifest = manifest_dir.join("lp-core/Cargo.toml");
    let lp_target = manifest_dir.join("target/lp-image");
    let elf = lp_target.join("riscv32imac-unknown-none-elf/release/lp_core_firmware");

    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = scrubbed_cargo(&cargo)
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&lp_manifest)
        .arg("--target")
        .arg("riscv32imac-unknown-none-elf")
        .env("CARGO_TARGET_DIR", &lp_target)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .unwrap_or_else(|err| {
            eprintln!("failed to spawn cargo for the LP-core crate: {err}");
            std::process::exit(1);
        });
    if !status.success() {
        eprintln!("LP-core crate failed to build");
        std::process::exit(1);
    }
    if !elf.is_file() {
        eprintln!("LP-core ELF missing at {}", elf.display());
        std::process::exit(1);
    }

    let bytes = fs::read(&elf).unwrap_or_else(|err| {
        eprintln!("failed to read LP-core ELF: {err}");
        std::process::exit(1);
    });
    // Content hash, so a changed LP image forces the HP crate to re-expand load_lp_code!.
    println!("cargo:rustc-env=LP_CORE_ELF_SHA={}", fnv(&bytes));
    println!("cargo:rerun-if-changed={}", elf.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let loader = out_dir.join("load_lp.rs");
    let escaped = elf
        .display()
        .to_string()
        .replace('\\', "\\\\")
        .replace('"', "\\\"");
    fs::write(&loader, format!("load_lp_code!(\"{escaped}\")\n")).unwrap_or_else(|err| {
        eprintln!("failed to write load_lp.rs: {err}");
        std::process::exit(1);
    });

    // Last, so linkall.x is the last linker script and the HP crate links.
    println!("cargo:rustc-link-arg=-Tlinkall.x");
}

/// Drop the parent crate's Cargo/rustc environment. A nested build that inherits
/// `CARGO_ENCODED_RUSTFLAGS` would link the LP image with the HP `linkall.x` script.
fn scrubbed_cargo(cargo: &str) -> Command {
    let mut cmd = Command::new(cargo);
    cmd.env_clear();
    for (key, value) in env::vars() {
        if forward_env(&key) {
            cmd.env(key, value);
        }
    }
    cmd
}

fn forward_env(key: &str) -> bool {
    if key == "CARGO_HOME" || key == "CARGO" {
        return true;
    }
    if key == "RUSTFLAGS" || key == "RUSTDOCFLAGS" {
        return false;
    }
    if key.starts_with("CARGO_") || key.starts_with("RUSTC_") || key.starts_with("RUSTDOC") {
        return false;
    }
    true
}

fn fnv(bytes: &[u8]) -> String {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }
    format!("{hash:016x}")
}
