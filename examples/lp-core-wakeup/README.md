<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# LP-Core GPIO Wakeup — ESP32-C6

> The HP core loads LP-core firmware into LP RAM, then deep-sleeps. The LP core watches GPIO 0 and wakes the HP core. A 20-second RTC timer is the backup. C6 has no capacitive touch.

## Implementation status

**Specs only** as of 2026-10-20. The Functional, Coding, and Test specs in this folder are the current design. There is no `Cargo.toml`, no `src/`, and no nested `lp-core/` crate. `cargo build` and `cargo run` fail in this directory until `esp32-sdd-full-project-generator` runs. This README describes the spec. It is not a record of an LP-core wake on hardware.

HP sleep, RTC, RMT, and `load_lp_code` use `esp-hal` feature **`unstable`**. LP-core APIs (`esp-lp-hal` 0.3.0, git tag `esp-hal-v1.2.1`) are unstable and may change with `esp-hal` minor releases. The HP crate stays on crates.io `esp-hal` 1.2.1 because this example has no radio. The git pin is only the nested LP crate.

## Overview

Each HP reset classifies the wake, blinks, **loads the LP image**, arms the backup timer, and enters deep sleep. Deep sleep does not return. The LP image must be loaded before sleep: the LP core cannot fetch code from flash after the HP core powers flash down.

- **LP-core / GPIO 0 wake:** three blinks (~200 ms), persistent counter increments by 1.
- **Timer wake (~20 s):** one slow blink (~500 ms). The GPIO counter does not increment.
- **XIAO BOOT (GPIO 9)** is not an RTC or LP GPIO. It cannot wake the chip. Use D0 / GPIO 0, or watch the timer blinks.

DevKitC BOOT is GPIO 0. Press it only after the chip has booted. Holding it across reset enters ROM download mode.

LP-on sleep current is **above** the ~7–15 µA RTC-only floor and is **unmeasured** in this spec. Do not copy ESP32-S3 touch-current numbers.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- Optional on XIAO: a momentary button from D0 (GPIO 0) to GND

ESP-IDF is **not** required. There is no ULP assembly and no CMake.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only. ESP32-S3 capacitive touch is not a stand-in for this example.

RTC / LP GPIOs on C6 are **0–7 only**.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Wake button |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | WS2812, cap 64/255. Three green blinks = GPIO wake. One slower amber blink = timer | Onboard BOOT = GPIO 0 (LP-capable) |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | GPIO 15, active LOW. Same timings, binary on/off | Onboard BOOT = GPIO 9, **does not wake**. External button on D0 / GPIO 0 |

UART-bridge boards (DevKitC **left** USB-C) keep `espflash monitor` connected through deep sleep. Native USB boards drop the port. That is expected, not a crash. Use the DevKitC UART-C port for the counter test. Treat XIAO as LED-only unless one cycle of log is enough.

Sleep with the LP core running draws more than timer-only deep sleep. The number is not in the spec until someone measures it.

## After the crate is generated

These commands are the intended workflow. **They fail today** because this folder has no `Cargo.toml`.

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

The HP `build.rs` compiles the nested `lp-core/` crate and links that image. No `CMakeLists.txt`.

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

> **Never edit generated source files directly.** To change behaviour, update the specs and regenerate.

### Opening in VS Code / Cursor

After generation this folder is a standalone Cargo package. Open **only this folder**.

**File → Open Folder… → select `examples/lp-core-wakeup/`**

Until then the folder is specs plus this README.

## Key Concepts

- **Load before HP sleep** — LP RAM is programmed while flash is still available. `sleep_deep` does not return.
- **Handshake word** — the LP image writes one word in LP RAM (`0x5000_0000` region) before waking the HP core. A timer wake must not treat a stale word as a new press. The persistent counter lives in HP RTC RAM (`#[esp_hal::ram(persistent)]`), not in LP RAM, because the image is reloaded.
- **Two wakeup sources** — LP-core GPIO 0, plus a ~20 s RTC timer so a missing button still cycles.
- **No touch driver** — do not generate `esp_sleep_enable_touchpad_wakeup` or an S3 target.
- **Application descriptor** — `esp_app_desc!()` on the HP image. Current `espflash` will not flash without it.

## Limitations

| This example is NOT | Why |
|---|---|
| Capacitive touch | ESP32-C6 has no touch peripheral. GPIO 9 cannot stand in for one |
| A zero-microamp wake source | The LP core stays clocked. Current is higher than RTC-timer-only sleep and is not specified numerically here |
| An LP sensor hub | The LP image only watches one GPIO and wakes the HP core |
| XIAO onboard-BOOT wake | BOOT is GPIO 9. Wire D0, or use the timer |

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

Run these only after the crate exists. From `examples/lp-core-wakeup/`:

1. **T-A1 — DevKitC-C6** — `cargo build --release` and `cargo clippy --release -- -D warnings` exit 0. The build log shows the nested `lp-core/` crate compiling. No `CMakeLists.txt` appears.
2. **T-A2 — XIAO ESP32-C6** — the same two commands with `--no-default-features --features board-xiao-c6`. The wake pad remains GPIO 0, not GPIO 9.

### Manual Tests — hardware required

Why manual: LP-core wake and persistent RAM need the real RTC domain.

**T-M1 — DevKitC-C6, left USB-C.** `cargo run --release`. Do not hold BOOT while resetting. Wait ~20 s and confirm one slow amber blink and no counter increment. Then press BOOT briefly and confirm three green blinks and a counter step of 1. Power-cycle and confirm the counter resets. The monitor stays connected across sleep.

**T-M2 — XIAO.** `cargo run --release --no-default-features --features board-xiao-c6`. The USB monitor exits on the first sleep. That is expected. Pressing onboard BOOT (GPIO 9) during sleep does not produce the three-blink pattern. A slow blink continues about every 20 seconds. A button on D0 produces three blinks.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)
