<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# 01 — Toolchain Setup

Install the compiler and the flasher used by every example. Pins and the MSRV live in [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md). There is no ESP-IDF install, no `espup`, and no `esp` rustup channel. [Why ESP-IDF is not required](why-esp-idf-is-not-required.md).

`no_std` here means Rust without the standard library, running on the chip through `esp-hal`. ESP32-C6 is RISC-V, so rustup **stable** already has the target. Xtensa chips need `espup`; they are not in v1.

## 1. rustup stable

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustc --version   # MSRV 1.95.0 or newer
```

The minimum supported Rust version is **1.95.0**.

## 2. Components and target

```sh
rustup component add rust-src clippy rustfmt
rustup target add riscv32imac-unknown-none-elf
```

## 3. Flash and monitor

```sh
cargo install espflash
# optional Cargo integration:
cargo install cargo-espflash
```

Generated `.cargo/config.toml` sets `runner = "espflash flash --monitor"`, so `cargo run --release` flashes and opens a serial monitor. A custom partition table appends `--partition-table partitions.csv`.

## 4. Verify

```sh
rustc --version
rustup show
espflash --version
```

You should see the stable toolchain, target `riscv32imac-unknown-none-elf`, and a working `espflash`.

## Hardware

| Board | Role | USB |
| --- | --- | --- |
| **ESP32-C6-DevKitC-1-N8** | Required default | **Left** USB-C is the UART-C port (CH343P UART bridge). Use it for `espflash`. The right USB-C port is native USB Serial/JTAG |
| Seeed XIAO ESP32-C6 | Also supported | Native USB (USB Serial/JTAG). The port drops in deep sleep |

This page installs the compiler and flasher; the agent path is [guide 02](02-install-skills.md) and [using-with-ai-agents.md](using-with-ai-agents.md).
