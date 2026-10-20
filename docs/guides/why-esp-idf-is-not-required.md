<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Why ESP-IDF is not required

If you already ship ESP32 firmware in C or C++, you almost certainly install **ESP-IDF** (Espressif IoT Development Framework): Python, CMake and Ninja, a GCC toolchain, `idf.py`, `sdkconfig` / `menuconfig`, and IDF components. You do not install that toolkit for this repository. ESP-IDF remains the right toolkit for C firmware. This page only explains the Rust path this repo actually uses.

This project is rust-on-metal on ESP32-C6. Firmware is **`no_std`**: Rust without the standard library, running on the chip through [`esp-hal`](https://github.com/esp-rs/esp-hal). You program it with rustup **stable**, Cargo, and [`espflash`](https://github.com/esp-rs/espflash). There is no ESP-IDF checkout, no `idf.py`, and no `sdkconfig`.

Install steps: [01 — Toolchain Setup](01-toolchain-setup.md). Pins: [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md).

## Two Rust-on-ESP32 ecosystems

There are two Rust-on-ESP32 ecosystems. Do not conflate them.

| Path | Crates | Needs ESP-IDF? | This repo |
| --- | --- | --- | --- |
| **`std` + ESP-IDF** | `esp-idf-sys`, `esp-idf-hal`, `esp-idf-svc` | **Yes.** Those crates are Rust bindings around the C ESP-IDF SDK | **Not used** |
| **`no_std` + `esp-hal`** | `esp-hal`, `esp-radio`, `esp-rtos`, `esp-lp-hal` | **No.** This is Espressif's official rust-on-metal HAL | **This is v1** |

Espressif's February 2025 [`esp-hal` 1.0 beta announcement](https://developer.espressif.com/blog/2025/02/rust-esp-hal-beta) marked the `esp-idf-*` crates **community-supported**. This repository does not use that path. Firmware talks to peripherals through Rust drivers (`esp-hal`, `esp-radio`, `esp-lp-hal`), not through IDF C drivers.

If you want `std` Rust wrapping ESP-IDF, install ESP-IDF and start from [The Rust on ESP Book](https://docs.espressif.com/projects/rust/book/). Keep this showcase on `esp-hal`.

## Why ESP-IDF is not installed here

### The HAL is bare-metal

[`esp-hal`](https://github.com/esp-rs/esp-hal) is a `no_std` HAL for Espressif devices. GPIO, UART, SPI, I2C, RMT, hardware AES, RTC sleep, and the C6 LP core are driven from Rust. There is no `idf.py`, no `sdkconfig`, and no IDF component registry.

### ESP32-C6 is RISC-V, so rustup stable is enough

The official [Toolchain Installation](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html) chapter splits the world in two:

- **RISC-V** (including the ESP32-C6): install rustup **stable** (or nightly) with `rust-src`, then `rustup target add riscv32imac-unknown-none-elf`. Those targets are Tier 2 in upstream Rust. If you are **not** targeting ESP32, ESP32-S2, or ESP32-S3 (Xtensa), you skip `espup`.
- **Xtensa** (ESP32, S2, S3): not an upstream LLVM or Rust target yet. That is what [`espup`](https://github.com/esp-rs/espup) and the `esp` rustup channel are for.

v1 of this showcase is **C6 only**. It does not use `espup` or the `esp` channel.

### The build is Cargo

Chip (`esp32c6`) and board (`board-devkitc-c6` / `board-xiao-c6`) are Cargo features. Pins live in generated `src/board.rs`. There is no `menuconfig`.

### Flash and monitor are `espflash`

`cargo run --release` uses `runner = "espflash flash --monitor"`. An example whose FunctionalSpec names a custom partition table appends `--partition-table partitions.csv`. [`espflash`](https://github.com/esp-rs/espflash) is the official `esp-rs` serial flasher, not `idf.py flash` or `idf.py monitor`.

### Radio is `esp-radio`, plus binary blobs

Wi-Fi, BLE, ESP-NOW, raw IEEE 802.15.4, Thread, and Matter over Wi-Fi use `esp-radio` on top of `esp-hal`, plus Espressif **binary blobs**. Copy the radio pin from [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md), including `esp-bootloader-esp-idf`. Raw 802.15.4 and Thread use IEEE 802.15.4 with Wi-Fi off: that driver does not coexist with Wi-Fi. Matter over Wi-Fi enables `wifi`, `ble`, and `coex`, and does not enable `ieee802154`. Thread is a two-node UDP lab link, not a border router. Matter over Wi-Fi uses test credentials. It is not Matter over Thread and not a certified product. That is not the ESP-IDF Wi-Fi or Bluetooth stack, not OpenThread from ESP-IDF, and not the ESP-IDF Matter stack. Radio APIs remain an **unstable beta**. Radio example READMEs say so. Do not treat them as frozen.

The Thread leader build needs Clang, CMake, and Ninja. A skip is not a leader pass. When the Matter image exceeds the 1 MB factory slot, the example README records the measured flash span and points `espflash` at one single-app `partitions.csv` that fits 4 MB flash.

## C-developer mapping

A **spec** is Markdown a person writes. The agent reads it and writes the Rust. A different SoC is a new spec, not a flag flip.

| ESP-IDF / C habit | This repository |
| --- | --- |
| `idf.py set-target esp32c6` | Cargo chip feature `esp32c6` — **v1 has no other chip**. A different SoC is a new spec, not a flag flip |
| `idf.py set-target esp32s3` (or any non-C6) | **Not in v1.** Do not swap chip features on an existing C6 example |
| `sdkconfig` / `menuconfig` | Cargo features + generated `src/board.rs` |
| `idf.py build` / `flash` / `monitor` | `cargo run --release` (`espflash`) |
| IDF components / `idf_component.yml` | crates (`esp-hal`, `esp-radio`, `esp-rtos`, …) |
| `nvs_flash_init` / IDF Wi-Fi/BT APIs | `esp-radio` (unstable beta) |

## Chip vs board

rust-on-metal firmware is **not** portable across ESP32, S3, and C6 via `idf.py set-target`. `esp-hal` selects the SoC with a Cargo chip feature. RTC GPIO maps, USB, LP-core versus ULP, and the rustc target are chip-gated. IDF `idf.py set-target` is not equivalent to swapping `esp-hal` chip features without a new spec.

| Axis | What it selects | v1 |
| --- | --- | --- |
| **Chip (SoC)** | `esp-hal` chip feature, rustc target, RTC/LP GPIO set, LP-core, radio / `unstable` | **`esp32c6` only** |
| **Board** | Pins, LED type, USB-UART versus native USB, antenna, LiPo — via `board-specs/` and generated `src/board.rs` | `board-devkitc-c6` (default), optional `board-xiao-c6` |

Changing **board** on the C6 is a Cargo feature and a regenerate. Changing **SoC** is a new FunctionalSpec, a `rust-toolchain.md` row, new board specs, and (for Xtensa) `espup`. Do not write `if S3 { … } else if C6 { … }` examples. The board catalog is the two ESP32-C6 boards only.

## Honest caveats

Skipping an ESP-IDF install is the right call for this repo. It is not a claim that ESP-IDF is unused in the wider ESP32 world.

- The crate `esp-bootloader-esp-idf` is **not** an ESP-IDF install. It only emits the ESP-IDF-compatible application descriptor (`esp_app_desc!()`) that current `espflash` and the chip's second-stage bootloader expect. Omitting it fails flash with `ESP-IDF App Descriptor missing`.
- Espressif chips still use an ESP-IDF-format **second-stage bootloader** already in flash on a typical DevKit module. See [Application Startup and Bootloader](https://docs.espressif.com/projects/rust/book/application-development/bootloader.html). You do not clone ESP-IDF to use the bootloader that is already on the module.
- Readers who want **`std` Rust wrapping ESP-IDF** install ESP-IDF and use the `esp-idf-*` crates. Point them at [The Rust on ESP Book](https://docs.espressif.com/projects/rust/book/) and keep this repo on `esp-hal`.
- Xtensa chips (ESP32, S2, S3) need `espup` even for `no_std`. They are **not in v1**.

You also do not get `menuconfig`, the IDF component registry, or IDF's C Wi-Fi and Bluetooth APIs on this path. Radio remains an unstable beta (`esp-radio` plus `esp-hal` feature `unstable`).

## Official sources

- [esp-hal](https://github.com/esp-rs/esp-hal) — `no_std` HAL
- [The Rust on ESP Book — Toolchain Installation](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html)
- [esp-hal 1.0.0 beta announcement](https://developer.espressif.com/blog/2025/02/rust-esp-hal-beta)
- [Application Startup and Bootloader](https://docs.espressif.com/projects/rust/book/application-development/bootloader.html)

Next: [01 — Toolchain Setup](01-toolchain-setup.md).
