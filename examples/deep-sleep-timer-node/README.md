<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Deep Sleep Timer Node

> RTC timer wakeup with boot-counter persistence and per-cycle LED heartbeat.

This example uses `esp-hal` sleep / RTC APIs gated on feature **`unstable`**. Those APIs may change with `esp-hal` minor releases.

## Overview

Baseline deep-sleep example. Demonstrates the wake-measure-persist-sleep loop that every battery-powered ESP32-C6 project depends on. An RTC timer fires on a short period; the firmware increments a boot counter stored in persistent RAM (`#[esp_hal::ram(persistent)]`), logs the count and wakeup cause over serial, blinks the onboard LED as a visible heartbeat, then returns to deep sleep.

On the DevKitC WS2812, each heartbeat blink uses a randomly selected colour from the project's standard 6-colour palette (cap 64/255).

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6

ESP-IDF is **not** required.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | Color: green = success, red = failure, off = sleep (WS2812) | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW | ~15 µA |

UART-bridge boards (DevKitC **left** USB-C) keep `espflash monitor` connected through deep sleep; native USB boards drop `/dev/cu.usbmodem*` and the monitor exits. That is expected, not a crash. Use the DevKitC UART-C port for counter-persistence tests; treat XIAO as LED-only observation unless you accept a single-cycle capture.

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

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/deep-sleep-timer-node/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

### First boot

```
sleep-node: First boot — counter resets to 0 on power-cycle
sleep-node: WS2812 RMT init on GPIO 8 (GRB order)
sleep-node: Sleeping
```

### Subsequent wakeups (UART-C port)

```
sleep-node: Wake #2 — timer wakeup
sleep-node: Sleeping
```

`#[esp_hal::ram(persistent)]` memory is retained across deep sleep but cleared on power-on. There is no sdkconfig RTC-memory option.

## Key Concepts

- **`#[esp_hal::ram(persistent)]`** — boot counter survives deep sleep; types must be safe to persist
- **RTC timer wakeup** — `esp-hal` sleep APIs (feature `unstable`)
- **WS2812 heartbeat** — random colour from the 6-colour palette at ≤64/255; GPIO boards blink once
- **USB vs UART console** — UART bridge stays up; USB Serial/JTAG drops on sleep
- **No sdkconfig** — board differences live in Cargo features + `src/board.rs`

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (DevKitC-C6)**

   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   ```

   Pass: exit 0, zero warnings.

2. **T-A2 — Zero-warning build (XIAO C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass: exit 0, zero warnings.

3. **T-A3 — Wokwi simulation (boot counter + serial log)** — additional tool, justified in TestSpec.md: simulates RTC timer wakeup cycles without physical hardware.

   Prerequisites: Wokwi CLI; `wokwi.toml` + `diagram.json` at the example root when generated.

   ```sh
   wokwi-cli --timeout 20000 --expect "Wake #3" .
   ```

   Pass: `Wake #3` appears in simulated serial output within 20 seconds.

### Manual Tests — hardware required

**T-M1 — Boot counter persistence through real deep sleep**

Why manual: persistent RAM depends on the hardware power domain staying live through a true deep-sleep state.

**Board:** ESP32-C6-DevKitC-1-N8 **left** USB-C (UART bridge) so the monitor stays connected.

1. `cargo run --release`
2. Observe several consecutive wakeups — confirm the counter increments with no gaps.
3. Power-cycle (disconnect USB, reconnect).
4. Confirm the counter resets to first boot.

Pass: counter increments monotonically; resets only on power-cycle; monitor stays connected across sleep.

---

**T-M2 — LED heartbeat (GPIO board)**

Why manual: visual on/off timing requires real hardware.

1. `cargo run --release --no-default-features --features board-xiao-c6`
2. Observe a single brief blink per wake; LED off during sleep.

Pass: single brief blink per cycle, off during sleep.

---

**T-M3 — WS2812 random-colour heartbeat**

Why manual: colour variation and 64/255 brightness require observing the physical LED.

1. `cargo run --release` on DevKitC-C6.
2. Observe 5+ consecutive wakeups.
3. Confirm colour is not identical every cycle; each blink is clearly visible; LED is off between blinks.

Pass: colour varies; blinks visible; LED off during sleep.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
