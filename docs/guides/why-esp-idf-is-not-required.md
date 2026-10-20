<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Why ESP-IDF is not required

If you already ship ESP32 firmware in C, you almost certainly install **ESP-IDF** (Espressif IoT Development Framework): Python, CMake/Ninja, a GCC toolchain, `idf.py`, `sdkconfig` / `menuconfig`, and IDF components. **You do not install that toolkit for this repository.**

This project is **rust-on-metal**: `no_std` firmware on ESP32-C6 through Espressif’s official [`esp-hal`](https://github.com/esp-rs/esp-hal). The chip is programmed from rustup **stable** + Cargo + [`espflash`](https://github.com/esp-rs/espflash). There is no ESP-IDF checkout, no `idf.py`, and no `sdkconfig`.

Install steps: [01 — Toolchain Setup](01-toolchain-setup.md). Pins: [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md).

## Two Rust-on-ESP32 paths — only one is used here

| Path | Crates | Needs ESP-IDF? | This repo |
|---|---|---|---|
| **`std` wrapping ESP-IDF** | `esp-idf-sys`, `esp-idf-hal`, `esp-idf-svc` | **Yes** — those crates bind the C SDK | **Not used** |
| **`no_std` rust-on-metal** | `esp-hal`, `esp-radio`, `esp-rtos`, `esp-lp-hal` | **No** — the HAL talks to hardware directly | **This is v1** |

Espressif’s Feb 2025 [`esp-hal` 1.0 beta announcement](https://developer.espressif.com/blog/2025/02/rust-esp-hal-beta) marked the `esp-idf-*` crates **community-supported** and pointed people who want the vendor-backed SDK at `esp-hal`. The `esp-idf-hal` README says the same: `esp-hal` is `no_std`-only and **does not use ESP-IDF**.

If you specifically want `std` Rust on top of ESP-IDF, that is a different ecosystem. Start from [The Rust on ESP Book](https://docs.espressif.com/projects/rust/book/) — not from this showcase.

## Why the C toolkit is unused

### The HAL is bare-metal

[`esp-hal`](https://github.com/esp-rs/esp-hal) is a bare-metal (`no_std`) hardware abstraction layer. GPIO, UART, SPI, I2C, RMT, hardware AES, RTC sleep, and the C6 LP core are driven from Rust (`esp-hal`, `esp-lp-hal`). Firmware does not call into IDF C drivers, so there is nothing for CMake, `idf.py`, or the IDF component registry to build.

### ESP32-C6 is RISC-V — rustup stable already compiles for it

The official [Toolchain Installation](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html) chapter splits the world in two:

- **RISC-V** (ESP32-C and H series, including **C6**): `rustup toolchain install stable --component rust-src` and `rustup target add riscv32imac-unknown-none-elf`. Those targets are **Tier 2** in upstream `rustc`. If you are not targeting ESP32 / ESP32-S2 / ESP32-S3, **you are done**.
- **Xtensa** (ESP32, S2, S3): not an upstream LLVM/Rust target yet. That is what [`espup`](https://github.com/esp-rs/espup) and the `esp` rustup channel exist for.

This showcase’s hardware bar is **ESP32-C6 only**. v1 therefore does not use `espup` or channel `esp`.

### Build, board, and pins are Cargo — not `sdkconfig`

Chip feature `esp32c6` and board features `board-devkitc-c6` / `board-xiao-c6` live in `Cargo.toml`. GPIO numbers come from `board-specs/` via generated `src/board.rs`. There is no `menuconfig`, no `sdkconfig.defaults`, and no `Kconfig.projbuild`.

### Flash and serial monitor are `espflash`

[`espflash`](https://docs.espressif.com/projects/rust/book/getting-started/tooling/espflash.html) is the `esp-rs` serial flasher. Generated `.cargo/config.toml` sets `runner = "espflash flash --monitor"`, so `cargo run --release` builds, flashes, and opens a monitor. Do not use `idf.py build` / `flash` / `monitor` as the primary workflow.

### Radio is `esp-radio`, not the IDF Wi-Fi/BT stacks

Wi-Fi, BLE, and ESP-NOW go through [`esp-radio`](https://github.com/esp-rs/esp-hal) on top of `esp-hal`, plus Espressif **binary blobs** for the radio MAC. That is not ESP-IDF’s Wi-Fi or Bluetooth controller APIs. Those crates still need `esp-hal` feature `unstable`; `esp-radio` itself is an unstable beta — radio example READMEs say so.

## C-developer cheat sheet

| ESP-IDF / C habit | This repository |
|---|---|
| `idf.py set-target esp32c6` | Cargo chip feature `esp32c6` — **v1 has no other chip**. A different SoC is a new spec, not a flag flip |
| `idf.py set-target esp32s3` (or any non-C6) | **Not in v1.** Do not swap chip features on an existing C6 example |
| `sdkconfig` / `menuconfig` | Cargo features + generated `src/board.rs` |
| `idf.py build` / `flash` / `monitor` | `cargo run --release` (`espflash`) |
| IDF components / `idf_component.yml` | crates (`esp-hal`, `esp-radio`, `esp-rtos`, …) |
| `nvs_flash_init` / IDF Wi-Fi/BT APIs | `esp-radio` (unstable beta) |

## Chip vs board

ESP-IDF often lets you `idf.py set-target` and rebuild. **rust-on-metal does not work that way.** `esp-hal` selects the SoC with a Cargo **chip feature**. RTC GPIO maps, USB, LP-core vs ULP, capacitive touch, and the rustc target are chip-gated.

| Axis | What it selects | v1 |
|---|---|---|
| **Chip (SoC)** | `esp-hal` feature, rustc target, RTC/LP GPIO set, LP-core, radio/`unstable` | **`esp32c6` only** |
| **Board** | Pins, LED type, USB-UART vs native USB, antenna, LiPo — `board-specs/` → generated `src/board.rs` | `board-devkitc-c6` (default), optional `board-xiao-c6` |

Changing **board** on C6 is a Cargo feature + regenerate. Changing **SoC** is a new FunctionalSpec, a `rust-toolchain.md` row, new board-specs, and (for Xtensa) `espup`. Do not write `if S3 { … } else if C6 { … }` examples. `board-specs/` contains only the two ESP32-C6 boards.

## What looks like ESP-IDF but is not an IDF install

Two names confuse C developers. Neither means “install ESP-IDF.”

1. **`esp-bootloader-esp-idf` (Rust crate).** Every example depends on this crate and calls `esp_bootloader_esp_idf::esp_app_desc!()`. It only writes the ESP-IDF-*compatible application descriptor* that current `espflash` and the second-stage bootloader require. Omitting it fails flash with `ESP-IDF App Descriptor missing`. The crate is **not** ESP-IDF.
2. **Second-stage bootloader on the chip.** Espressif devices boot through a ROM first-stage loader, then a second-stage loader that understands the ESP image format. The *Rust on ESP Book* ([Application Startup and Bootloader](https://docs.espressif.com/projects/rust/book/application-development/bootloader.html)) notes that the supported second-stage loader is currently the ESP-IDF bootloader. Typical DevKit modules already have that loader in flash. You do **not** clone ESP-IDF to use it.

## What this path does not give you

Skipping ESP-IDF is the right call for this repo. It is not a claim that ESP-IDF is unused in the wider ESP32 world.

- You do not get `menuconfig`, the IDF component registry, or IDF’s C Wi-Fi/BT APIs.
- Xtensa chips (ESP32, S2, S3) still need `espup` even for `no_std`. They are **not in v1**.
- Radio remains an **unstable beta** (`esp-radio` + `esp-hal` feature `unstable`).

## Official sources

- [esp-hal](https://github.com/esp-rs/esp-hal) — bare-metal (`no_std`) HAL
- [The Rust on ESP Book — Toolchain Installation](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html)
- [esp-hal 1.0.0 beta announcement](https://developer.espressif.com/blog/2025/02/rust-esp-hal-beta) — `esp-idf-*` community-supported; official SDK is `esp-hal`
- [Application Startup and Bootloader](https://docs.espressif.com/projects/rust/book/application-development/bootloader.html)

Next: [01 — Toolchain Setup](01-toolchain-setup.md).
