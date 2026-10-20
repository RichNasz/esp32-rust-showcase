# <!-- FILL: Example Title — Short Tagline -->

## Overview

<!-- FILL: 2–4 sentences. What the example demonstrates, why it exists, who it is for. -->

## Target Hardware

Chip (SoC) and board are different axes. Name **one** SoC. Do not describe behaviour that needs another SoC’s peripherals (S3 touch, classic ESP32 ULP, Xtensa-only tooling). rust-on-metal is not `idf.py set-target`.

| Item | v1 value |
| --- | --- |
| **SoC** | ESP32-C6 only |
| **Chip Cargo feature** | `esp32c6` |
| **rustc target** | `riscv32imac-unknown-none-elf` |

Other SoCs are not in v1. Switching SoC later is a new spec + `rust-toolchain.md` row, not a feature flip.

### Board Compatibility Checklist

Tick every **C6** board this example must support. Pins come from `board-specs/`, not from this file. Default is DevKitC-C6.

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [ ] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add non-C6 boards. RTC / LP GPIOs on C6 are **0–7 only**; GPIO 9 (XIAO boot button) is **not** RTC-capable.

## Features

<!-- FILL: bullet list of observable behaviour. -->

## Wakeup Strategy

<!-- FILL: timer / RTC GPIO 0–7 / LP-core / none. If unused, write "Not applicable". -->

## Connectivity

<!-- FILL: none / BLE / Wi-Fi / ESP-NOW / MQTT / raw IEEE 802.15.4 / Thread / Matter over Wi-Fi. Radio examples copy the pin from rust-toolchain.md. Raw 802.15.4 and Thread do not also enable Wi-Fi. Matter over Wi-Fi enables wifi, ble, and coex, not ieee802154. -->

## Security Requirements

<!-- FILL: none, or AES / esp-storage, or TLS / CRC OTA. CRC OTA is not the AES skill. Follow shared-specs/security-baseline.md. -->

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf`.
- `cargo clippy -- -D warnings` is clean.
- **`cargo run --release`** flashes via `espflash` and the example meets the behavioural checks below.
- Optional XIAO: `cargo run --release --no-default-features --features board-xiao-c6`.
- <!-- FILL: observable on-device criteria (LED, serial lines, current, …) -->
