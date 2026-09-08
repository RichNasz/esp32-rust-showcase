<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# LP-Core GPIO Wakeup

> HP core deep-sleeps; the ESP32-C6 LP core watches an RTC GPIO and wakes the chip. Replaces capacitive-touch-wakeup (C6 has no touch peripheral).

This example uses `esp-hal` sleep / RTC APIs gated on feature **`unstable`**, plus a nested `lp-core/` crate on `esp-lp-hal`. Those APIs may change with `esp-hal` minor releases.

## Overview

Human-interface deep-sleep example for rust-on-metal ESP32-C6. The HP core loads LP-core firmware into LP RAM, then enters deep sleep. The LP core monitors an RTC GPIO (C6: **GPIO 0–7 only**). On edge, it wakes the HP core. The HP core blinks the LED three times, logs the event, increments a persistent counter, and sleeps again.

A backup RTC timer wakeup prevents sleeping forever if the GPIO is never triggered. On timer wakeup the LED blinks once slowly and the LP image is loaded again before HP sleep.

**Load the LP-core image before the HP core sleeps.** The LP core cannot fetch its image from flash after the HP core has powered down the flash.

**GPIO 9 is not RTC.** XIAO ESP32-C6 boot button cannot wake the chip. DevKitC-C6 boot is GPIO 0 (RTC-capable) and is the default wakeup pad. On XIAO, use an external button on LP GPIO 0–7 or timer-only wakeup.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- Optional: external button on LP GPIO 0–7 for XIAO

ESP-IDF is **not** required.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6. Deep-sleep timer + user LED required. LP-core GPIO wakeup needs an RTC-capable pad (0–7).

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only. Capacitive-touch boards are not a C6 path.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | Color: green = success, red = failure, off = sleep (WS2812) | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW | ~15 µA |

UART-bridge boards keep the serial monitor connected through deep sleep; native USB boards drop and reconnect — this is expected, not a crash. Use DevKitC **left** USB-C for multi-cycle logs. XIAO boot GPIO 9 **cannot** wake from deep sleep.

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

The generator emits a nested `lp-core/` crate when required. `build.rs` embeds the LP image; `linkall.x` is the last linker script.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/lp-core-wakeup/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

```
lp-wakeup: LP image loaded
lp-wakeup: HP sleeping; LP watching GPIO 0
```

On GPIO wakeup:

```
lp-wakeup: LP GPIO wake  pad=0  count=4
lp-wakeup: LED 3×; sleeping
```

On timer backup:

```
lp-wakeup: Timer wake — no GPIO
lp-wakeup: LED 1× slow; sleeping
```

## Key Concepts

- **Nested `lp-core/` crate** — `esp-lp-hal`, `#![no_std]` `#![no_main]`
- **`load_lp_code` before HP deep sleep** — LP cannot fetch from flash after HP powers down flash
- **RTC GPIO 0–7 only** — GPIO 9 is not RTC
- **Persistent counter** — `#[esp_hal::ram(persistent)]` on the HP core
- **Backup timer wakeup** — avoids sleeping forever
- **WS2812 vs GPIO LED** via Cargo board features

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (HP + LP image embed)**

   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   ```

   Pass: exit 0, zero warnings. Confirms the LP image links into the HP crate.

2. **T-A2 — XIAO feature build**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   ```

   Pass: exit 0.

3. **T-A3 — Timer wakeup via Wokwi** — additional tool, justified in TestSpec.md: the backup timer path is testable in simulation; GPIO/LP wakeup needs silicon.

   ```sh
   wokwi-cli --timeout 30000 --expect "Timer wake" .
   ```

   Pass: `Timer wake` appears within 30 seconds when Wokwi files are generated.

### Manual Tests — hardware required

**T-M1 — LP-core GPIO wakeup from deep sleep**

Why manual: LP GPIO edge detection and HP wake handshake require real C6 silicon.

Hardware: ESP32-C6-DevKitC-1-N8, USB-C on the **left** UART-C port. Wake pad = BOOT / GPIO 0.

1. `cargo run --release`
2. Wait for "HP sleeping".
3. Press BOOT (GPIO 0).
4. Confirm wake promptly; serial shows LP GPIO wake; LED blinks 3×; counter increments.

Pass: GPIO wakes HP; 3 LED blinks; count increments.

---

**T-M2 — No false wakeups**

Why manual: ambient noise on the pad is a physical phenomenon.

1. Leave the pad untouched for several backup-timer periods.
2. Confirm timer-wake logs, not GPIO-wake, when idle.

Pass: no phantom GPIO wakes in idle environment.

---

**T-M3 — Timer wakeup (no GPIO)**

Why manual: confirms the backup timer while observing the LED pattern.

1. Do not press the button.
2. After the backup interval, confirm timer-wake log and a single slow LED blink.
3. Confirm the device loads LP firmware again and sleeps.

Pass: timer wakeup; single blink; sleep resumes.

XIAO: do not expect the BOOT button (GPIO 9) to wake the chip. Wire an external button to LP GPIO 0–7 or rely on T-M3.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
