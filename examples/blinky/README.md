<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Breathing LED — Multi-Board Demo

> **SDD baseline.** Demonstrates the full spec-to-firmware pipeline on rust-on-metal ESP32-C6. Drives a smooth breathing effect on the onboard status LED using GPIO PWM (simple LED) or the `esp-hal` RMT peripheral (WS2812 RGB), selected at compile time via mutually exclusive Cargo board features.

## Overview

Drives a smooth, continuous breathing effect on the onboard status LED of whichever C6 board is selected. Breathing period is 4 seconds (2 s fade up, 2 s fade down).

Two code paths are selected by Cargo board features (default `board-devkitc-c6`):

- **WS2812 path** (ESP32-C6-DevKitC-1-N8): RMT peripheral drives the 800 kHz NRZ protocol; brightness cycles through a colour sequence (red, green, blue, cyan, magenta, amber) with channel values capped at 64/255.
- **GPIO path** (XIAO ESP32-C6): simple GPIO LED, active LOW, hardware-timed fade — no busy-wait loops.

Pin numbers live in generated `src/board.rs`. Never hard-code GPIOs in `main.rs`.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6

ESP-IDF is **not** required.

## Supported Boards

Auto-discovered from `board-specs/` against the FunctionalSpec board-compatibility rules, filtered to ESP32-C6.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only.

## Per-Board Behavior

| Board | LED feedback |
| --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Color sequence at ≤64/255 (WS2812) |
| Seeed XIAO ESP32-C6 | On / off, active LOW |

The most consequential difference for a first-time user: DevKitC uses a **WS2812** on GPIO 8 (RMT required); XIAO uses a **simple active-LOW GPIO LED** on GPIO 15. Switch boards with Cargo features, not menuconfig. Flash DevKitC via the **left** USB-C UART-C port.

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
blinky: Board GPIO=15  polarity=active-LOW
blinky: Breathing period: 4000 ms
blinky: Fade up
blinky: Fade down
```

## Key Concepts

- **Standalone Cargo crate** — `#![no_std]` `#![no_main]`; Embassy entry via `esp-rtos`; no workspace at the repo root
- **Board features** — default `board-devkitc-c6`; optional `board-xiao-c6`; chip feature `esp32c6` only
- **`src/board.rs`** — GPIO / LED / polarity constants from `board-specs/`; `compile_error!` if both or neither board features are enabled
- **WS2812 via RMT** — GRB byte order, 800 kHz NRZ, brightness cap 64/255
- **Active-LOW GPIO LED** — XIAO C6 inverts drive so the breath still starts dark and brightens first

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

   Pass: image flashes; serial log shows LED driver init and breathing loop start.

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

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
