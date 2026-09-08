# <!-- FILL: Example Title — Short Tagline -->

## Overview

<!-- FILL: 2–4 sentences. What the example demonstrates, why it exists, who it is for. -->

## Target Hardware

v1 chip: **ESP32-C6** only (`esp32c6`). Other SoCs are not in v1.

### Board Compatibility Checklist

Tick every board this example must support. Default is DevKitC-C6.

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [ ] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add non-C6 boards. RTC / LP GPIOs on C6 are **0–7 only**; GPIO 9 (XIAO boot button) is **not** RTC-capable.

## Features

<!-- FILL: bullet list of observable behaviour. -->

## Wakeup Strategy

<!-- FILL: timer / RTC GPIO 0–7 / LP-core / none. If unused, write "Not applicable". -->

## Connectivity

<!-- FILL: none / BLE / Wi-Fi / ESP-NOW / MQTT. Radio examples git-pin the esp-hal monorepo stack (rust-toolchain.md). -->

## Security Requirements

<!-- FILL: none, or AES / esp-storage / TLS / OTA. Follow shared-specs/security-baseline.md. -->

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf`.
- `cargo clippy -- -D warnings` is clean.
- **`cargo run --release`** flashes via `espflash` and the example meets the behavioural checks below.
- Optional XIAO: `cargo run --release --no-default-features --features board-xiao-c6`.
- <!-- FILL: observable on-device criteria (LED, serial lines, current, …) -->
