<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# BLE Beacon + Deep Sleep — Multi-Year Battery Life

> Duty-cycled BLE beacon: advertise a short window, tear the radio down, deep sleep.

This example enables `esp-hal` feature **`unstable`** (radio + sleep / RTC). **`esp-radio` is an unstable beta** (git-pinned with the `esp-hal` monorepo stack at tag `esp-hal-v1.2.1`; TrouBLE host as specified in `shared-specs/ble-patterns.md`). Those APIs may change with `esp-hal` minor releases.

## Overview

Wakes from deep sleep, advertises a BLE beacon with custom manufacturer data (boot counter, sequence, magic `DE AD BE EF`) plus complete local name `"ESP32-SDD"`, then stops advertising and returns to deep sleep. LED is on only while advertising is active (WS2812 solid blue at ≤64/255 on DevKitC; GPIO active level on XIAO).

**C6 wakeup:** RTC GPIOs are 0–7 only. DevKitC boot is GPIO 0 (RTC-capable) and can be used for on-demand advertisement. XIAO boot is GPIO 9 — **not** RTC-capable — so XIAO is timer-only unless you wire an external button to LP GPIO 0–7.

Tear down `esp-radio` before deep sleep.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- A BLE scanner (e.g. nRF Connect) for manual tests

ESP-IDF is **not** required. Never mix crates.io `esp-rtos = "0.4.0"` with git `esp-hal`.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6. BLE + deep-sleep timer + user LED required.

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

UART-bridge boards keep the serial monitor connected through deep sleep; native USB boards drop and reconnect — this is expected, not a crash. Use DevKitC **left** USB-C for multi-cycle serial logs.

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

**File → Open Folder… → select `examples/ble-beacon-deep-sleep/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

```
ble-beacon: BLE MAC: AA:BB:CC:DD:EE:FF
ble-beacon: Wake #3 — timer
ble-beacon: adv start  name=ESP32-SDD
ble-beacon: adv stop; radio teardown; sleeping
```

Scanner apps should list **`ESP32-SDD`**. Manufacturer data: company ID `0xFFFF`, magic `DE AD BE EF`, little-endian counters. ADV PDU ≤ 31 bytes.

## Key Concepts

- **`esp-radio` + TrouBLE** — non-connectable advertising; unstable beta
- **Git-pin** — `esp-hal` / `esp-radio` / `esp-rtos` from tag `esp-hal-v1.2.1`
- **Bounded advertising window** via `embassy_time`
- **Radio teardown before sleep**
- **Persistent counters** — `#[esp_hal::ram(persistent)]`
- **LED = advertising-active only** — dark during init and teardown
- **GPIO 9 is not RTC** on C6

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build**

   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   ```

   Pass: exit 0, zero warnings (radio git-pin resolves).

2. **T-A2 — XIAO feature build**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   ```

   Pass: exit 0.

### Manual Tests — hardware required

**T-M1 — BLE advertisement visible to scanner**

Why manual: OTA BLE content needs a second device acting as a scanner.

1. `cargo run --release` on DevKitC-C6.
2. Open nRF Connect → Scanner → start scanning.
3. Confirm a burst each wake; name `ESP32-SDD`; company ID `0xFFFF`; magic `DE AD BE EF`; `boot_count` increments.

Pass: advertisement every cycle; manufacturer data matches spec.

---

**T-M2 — Active window / LED timing**

Why manual: advertising-window duration is easiest to judge from the LED.

1. Observe LED on only while advertising; off during init, teardown, and sleep.
2. Serial timestamps: adv-start then adv-stop within the specified window.

Pass: LED dark except during advertising.

---

**T-M3 — Button wakeup (DevKitC GPIO 0)**

Why manual: physical RTC GPIO wakeup requires silicon.

1. During sleep, press DevKitC BOOT (GPIO 0).
2. Confirm advertisement and LED within 500 ms.

Pass: on-demand burst. XIAO GPIO 9 cannot wake from deep sleep.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
