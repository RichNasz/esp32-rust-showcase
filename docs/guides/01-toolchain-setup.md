<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# 01 — Toolchain Setup

Install the rust-on-metal toolchain used by every example in this repository. There is **no ESP-IDF install**. Pins live in [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md).

## 1. rustup stable

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustc --version   # MSRV 1.95.0 or newer
```

Do **not** install the `esp` rustup channel or `espup`. RISC-V ESP32-C6 `no_std` uses rustup **stable**.

## 2. Component and target

```sh
rustup component add rust-src clippy rustfmt
rustup target add riscv32imac-unknown-none-elf
```

## 3. Flash / monitor

```sh
cargo install espflash
# optional Cargo integration:
cargo install cargo-espflash
```

The generated `.cargo/config.toml` sets `runner = "espflash flash --monitor"`, so `cargo run --release` flashes and opens a serial monitor.

## 4. Verify

```sh
rustc --version
rustup show
espflash --version
```

You should see the stable toolchain, target `riscv32imac-unknown-none-elf`, and a working `espflash`.

## Hardware

| Board | Role | USB |
|---|---|---|
| **ESP32-C6-DevKitC-1-N8** | Required default | **Left** USB-C = UART bridge (`espflash`); right USB-C = native USB Serial/JTAG |
| Seeed XIAO ESP32-C6 | Also supported | Native USB Serial/JTAG (port drops in deep sleep) |

Next: [02 — Install Skills](02-install-skills.md).
