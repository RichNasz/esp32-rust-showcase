# Breathing LED (ESP32-C6)

## Overview

Baseline SDD example. Demonstrates the full spec-to-firmware pipeline on rust-on-metal
ESP32-C6 (`no_std` `esp-hal`). Drives a smooth breathing effect on the onboard status LED
using the RMT peripheral (800 kHz NRZ, WS2812 RGB) or LEDC PWM (simple GPIO LED), selected
at compile time by mutually exclusive Cargo board features. Chip is ESP32-C6 only.

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/main.rs` are present (generated 2026-09-08). This line is not a hardware-test result. The catalog table in `specs/showcase-master-spec.md` is the source of truth across examples.

## Target Hardware

v1 chip: **ESP32-C6** only (`esp32c6`). Other SoCs are not in v1.

### Board Compatibility Checklist

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [x] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add boards for any SoC other than ESP32-C6.

RTC / LP GPIOs on C6 are **0–7 only**; GPIO 9 (XIAO boot button) is **not** RTC-capable.
This example does not sleep, so that constraint is informational only.

## Features

- Breathe the onboard status LED of whichever C6 board is selected, continuously.
- Breathing period: 4 seconds (2 s fade-up, 2 s fade-down). The sequence always starts
  dark and brightens first.
- **DevKitC-C6 (default, `board-devkitc-c6`)**: WS2812 RGB LED on GPIO 8, driven by the
  `esp-hal` RMT peripheral (not a GPIO level — a level produces no visible output).
  Brightness cycles through a fixed colour sequence (red, green, blue, cyan, magenta,
  amber). Each channel is capped at 64/255. Colour advances at the start of each new
  4-second breath cycle. GRB byte order, 800 kHz NRZ.
- **XIAO ESP32-C6 (`board-xiao-c6`)**: simple user LED on GPIO 15, **active LOW**. Drive
  with LEDC PWM if practical on the pinned `esp-hal`; otherwise Embassy-timed duty.
  No busy-wait loops. Invert duty so dark is still the start of the breath.
- Pin numbers live in generated `src/board.rs`. Application code must not hardcode GPIOs.
- Board selection is a Cargo feature, not menuconfig. Default features select DevKitC-C6.
  XIAO: `cargo run --release --no-default-features --features board-xiao-c6`.
- Serial log at INFO confirms LED path (WS2812/RMT vs GPIO/LEDC), GPIO, and breathing
  loop start (`log` + `esp-println`).

## Wakeup Strategy

Not applicable. This example runs continuously; it does not enter deep sleep.

## Connectivity

None. Non-radio crate pin: crates.io `esp-hal` 1.2.1 + `esp-rtos` 0.4.0.

## Security Requirements

None.

## Hardware Dependencies

- Board-spec: `board-specs/espressif/esp32-c6-devkitc-1-n8.md` (WS2812 RGB on GPIO 8;
  flash/monitor on the **left** USB-C UART-C port).
- Board-spec: `board-specs/seeed/xiao-esp32c6.md` (user LED GPIO 15, active LOW;
  USB Serial/JTAG console).
- No external components required.

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf` (default DevKitC-C6 features).
- `cargo clippy -- -D warnings` is clean for the default features.
- `cargo build --release --no-default-features --features board-xiao-c6` and the matching clippy invocation are clean.
- **`cargo run --release`** flashes via `espflash` on DevKitC-C6 (left USB-C / UART-C). The WS2812 breathes through the colour sequence over ~4 seconds per cycle; serial log confirms RMT init on GPIO 8.
- Optional XIAO: `cargo run --release --no-default-features --features board-xiao-c6`. The GPIO 15 LED starts dark, then breathes smoothly over ~4 seconds (active-LOW polarity).
