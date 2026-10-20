<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-full-project-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Breathing LED — ESP32-C6

> **SDD baseline.** Demonstrates the full spec-to-firmware pipeline on rust-on-metal ESP32-C6. Drives a smooth breathing effect on the onboard status LED using GPIO PWM (simple LED) or the `esp-hal` RMT peripheral (WS2812 RGB), selected at compile time via mutually exclusive Cargo board features.

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/main.rs` are in this folder (generated 2026-09-08 from the specs). The build commands below apply to this crate. This is not a record of a hardware test on 2026-10-20.

## Overview

Drives a smooth, continuous breathing effect on the onboard status LED of whichever C6 board is selected. Breathing period is 4 seconds (2 s fade up, 2 s fade down). The sequence always starts dark and brightens first.

Two code paths are selected by Cargo board features (default `board-devkitc-c6`):

- **WS2812 path** (ESP32-C6-DevKitC-1-N8): RMT peripheral drives the 800 kHz NRZ protocol; brightness cycles through a colour sequence (red, green, blue, cyan, magenta, amber) with channel values capped at 64/255.
- **GPIO path** (XIAO ESP32-C6): simple GPIO LED, active LOW, LEDC PWM with Embassy-timed duty — no busy-wait loops.

Pin numbers live in generated `src/board.rs`. Never hard-code GPIOs in `main.rs`.

This example enables `esp-hal` feature `unstable` because the RMT (WS2812) and LEDC APIs are unstable on the 1.2.1 pin.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6

ESP-IDF is **not** required. Do not install `espup` or the `esp` rustup channel.

## Build & Flash

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

> **Never edit generated source files directly.** To change board configuration, update `specs/FunctionalSpec.md` or `specs/CodingSpec.md` and regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/blinky/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

### ESP32-C6-DevKitC-1-N8 (WS2812 RGB, GPIO 8)

```
blinky: Blinky — WS2812 RGB LED, GPIO 8, breathing period 4000 ms
blinky: WS2812 RMT channel init on GPIO 8 (GRB order, max brightness 64/255)
blinky: Starting WS2812 breathing loop
blinky: Cycle color 1/6  r=64 g=0 b=0
```

The RGB LED breathes through a colour sequence — red, green, blue, cyan, magenta, amber — advancing at the start of each 4-second breath cycle.

### Seeed XIAO ESP32-C6 (active LOW, GPIO 15)

```
blinky: Blinky — GPIO LED (active-LOW), GPIO 15, breathing period 4000 ms
blinky: Board GPIO=15  polarity=active-LOW
blinky: Breathing period: 4000 ms
blinky: Starting GPIO LED breathing loop (LEDC PWM, Embassy-timed duty)
blinky: Fade up
blinky: Fade down
```

## Key Concepts

- **Standalone Cargo crate** — `#![no_std]` `#![no_main]`; Embassy entry via `esp-rtos`; no workspace at the repo root
- **Board features** — default `board-devkitc-c6`; optional `board-xiao-c6`; chip feature `esp32c6` only
- **`src/board.rs`** — GPIO / LED / polarity constants from `board-specs/`; `compile_error!` if both or neither board features are enabled
- **WS2812 via RMT** — GRB byte order, 800 kHz NRZ, brightness cap 64/255; `esp-hal` RMT is an **unstable** API
- **LEDC PWM** — XIAO C6 inverts duty so the breath still starts dark and brightens first; LEDC is an **unstable** API
- **Embassy time** — 4 s breath cadence with `embassy_time`; no busy-wait
- **Application descriptor** — `esp-bootloader-esp-idf` `esp_app_desc!()`; current `espflash` will not flash without it. This is image metadata, not ESP-IDF.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (DevKitC-C6, default features)**

   From `examples/blinky/`:

   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings.

2. **T-A2 — Zero-warning build (XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings.

3. **T-A3 — Flash via espflash runner**

   ```sh
   cargo run --release
   ```

   Pass: image flashes (no missing application descriptor); serial log shows LED driver init and breathing loop start.

### Manual Tests — hardware required

**T-M1 — Visual WS2812 breathing (ESP32-C6-DevKitC-1-N8)**

Why manual: WS2812 protocol errors produce no light or solid-on; a build cannot certify perceptual smoothness.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8, USB-C on the **left** UART-C port.

1. `cargo run --release`
2. Confirm serial output mentions WS2812 / RMT init on GPIO 8.
3. Confirm the RGB LED breathes through colours over ~4 seconds per cycle with no stuck-on or stuck-off.

Pass: LED breathes smoothly; serial log confirms RMT init.

---

**T-M2 — Visual GPIO breathing (Seeed XIAO ESP32-C6)**

Why manual: active-LOW polarity and fade quality require human observation.

Hardware required: Seeed XIAO ESP32-C6, USB-C cable.

1. `cargo run --release --no-default-features --features board-xiao-c6`
2. Observe the blue user LED on GPIO 15.
3. Confirm it starts dark, then brightens, and breathes smoothly over ~4 seconds.

Pass: correct polarity, smooth breathing.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)
