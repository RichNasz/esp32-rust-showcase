<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-full-project-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# LP-Core GPIO Wakeup — ESP32-C6

> The HP core loads a small LP-core image, then deep-sleeps. The LP core watches GPIO 0 and wakes the HP core. A 20-second RTC timer is the backup. C6 has no capacitive touch.

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml`, `src/`, and the nested `lp-core/` crate are in this folder. A clean host compile is not a hardware wake test.

`specs/showcase-master-spec.md` still lists this example as **Specs only**. Humans own that catalog. The FunctionalSpec in this folder repeats that older note.

HP sleep, RTC, RMT, and `load_lp_code` use `esp-hal` feature **`unstable`**. LP-core APIs (`esp-lp-hal` 0.3.0, git tag `esp-hal-v1.2.1`) are unstable and may change with `esp-hal` minor releases. The HP crate stays on crates.io `esp-hal` 1.2.1 because this example has no radio. The git pin is only the nested LP crate.

## Overview

Each HP reset classifies the wake, blinks, **loads the LP image**, arms the backup timer, and enters deep sleep. Deep sleep does not return. The LP image is loaded before sleep: the LP core cannot fetch code from flash after the HP core powers flash down.

- **LP-core / GPIO 0 wake:** three blinks (~200 ms on / 200 ms off). The persistent GPIO counter increments by 1.
- **Timer wake (~20 s):** one slow blink (~500 ms). The GPIO counter does not increment.
- **First boot:** a log line, then sleep. The counter starts at 0 and resets on power-cycle.
- **XIAO BOOT (GPIO 9)** is not an RTC or LP GPIO. It cannot wake the chip. Use D0 / GPIO 0, or watch the timer blinks.

DevKitC uses the WS2812 on GPIO 8 (RMT, GRB, brightness cap 64/255): green for a GPIO wake, amber for a timer wake, black before sleep. XIAO uses GPIO 15, active LOW, with the same timings.

DevKitC BOOT is GPIO 0. Press it only after the chip has booted. Holding it across reset enters ROM download mode. A stuck low line wakes, blinks, and sleeps again. It does not spin on the HP core.

The handshake word is `0x4750_494F` at LP RAM `0x5000_2000` (C6 LP RAM starts at `0x5000_0000`). The LP image writes it after a debounced active-LOW edge. The HP core reads it before `LpCore::new`, which clears LP RAM. A timer wake ignores a stale word and does not increment the counter. If the word and the HP wakeup cause disagree, the firmware logs a warning and trusts the HP cause for the blink pattern.

LP-on sleep current is **above** the ~7–15 µA RTC-only floor and is **unmeasured**. Do not copy ESP32-S3 touch-current numbers.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- Optional on XIAO: a momentary button from D0 (GPIO 0) to GND

ESP-IDF is **not** required. There is no ULP assembly and no CMake.

## Supported Boards

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED | Wake pad | USB console |
| --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 WS2812 | GPIO 0 (BOOT, LP GPIO) | UART bridge, **left** USB-C |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15, active LOW | GPIO 0 (D0). BOOT is GPIO 9 and cannot wake | USB Serial/JTAG |

C6 RTC / LP GPIOs are **0–7 only**. GPIO 9 is not in that set.

## Per-board behavior

| Board | Serial monitor during deep sleep | LED feedback |
| --- | --- | --- |
| ESP32-C6-DevKitC-1-N8, **left** USB-C | Stays connected. Silence during sleep, logs resume on wake | Three green blinks (GPIO) or one amber blink (timer) |
| XIAO ESP32-C6 | Drops on the first sleep. `cargo run` exits. That is expected | Same timings, on/off. GPIO 9 does nothing |

Use the DevKitC UART-C port for counter-persistence tests. Treat XIAO as LED-only observation unless one cycle of log is enough.

## Power budget

HP active current is CPU-only (~25 mA). Sleep with the LP core running is above the RTC-only floor. The v1 success bar is a functional wake, not a current-meter reading.

| Mode | Typical current | Duration | Energy per cycle |
| --- | --- | --- | --- |
| Active (HP CPU, no RF) | ~25 mA | ~1.5 s GPIO-wake blinks; ~0.6 s timer blink | ~10 µAh / ~4 µAh |
| Deep sleep + LP core running | above ~15 µA; unmeasured | up to 20 s | — |
| **Total per duty cycle** | — | up to ~21.5 s | dominated by the active blink if LP sleep stays well under 1 mA |

**Sleep wakeup source:** LP-core GPIO 0, backup RTC timer
**Wakeup interval:** 20 seconds when the button is idle

## Build & Flash

From `examples/lp-core-wakeup/`:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

`build.rs` compiles the nested `lp-core/` crate with Cargo (no CMake) and embeds the ELF. Do not flash the LP ELF by itself.

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

> **Never edit generated source files directly.** To change behavior, update `specs/FunctionalSpec.md` or `specs/CodingSpec.md` and regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/lp-core-wakeup/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

DevKitC, left USB-C. Levels are `INFO` unless noted.

```
INFO First boot — GPIO count 0 (resets on power-cycle)
INFO WS2812 RMT init on GPIO 8 (GRB order, max brightness 64/255)
INFO LP image loaded (…………)
INFO Sleeping for 20 s — LP GPIO 0 or RTC timer
INFO Timer wake — GPIO count 0 unchanged
INFO LP-GPIO wake — GPIO count 1
```

A disagreement looks like:

```
WARN timer wake but handshake 0x4750494f still set; not counting a press
```

XIAO logs the same wake lines until the USB port drops on the first sleep.

## Key Concepts

- Nested `lp-core/` firmware on `esp-lp-hal` (`#![no_std]` `#![no_main]`), built by the HP `build.rs` and copied into LP RAM with `load_lp_code!`.
- Load and start the LP image, then `enable_wakeup()`, then HP `sleep_deep`. Flash is off during deep sleep, so loading afterwards is impossible.
- HP wakeup sources: LP core (GPIO 0, active LOW, pull-up armed on the HP side before the image runs) and a ~20 s RTC timer.
- Persistent GPIO counter: `#[esp_hal::ram(unstable(rtc_fast, persistent))]`, magic plus checksum. LP RAM is not that store. The reload clears it.
- Embassy time for the blinks. DevKitC WS2812 via RMT (800 kHz NRZ, GRB, cap 64/255). XIAO is a plain active-LOW GPIO.
- `esp_bootloader_esp_idf::esp_app_desc!()` so current `espflash` accepts the image. That crate is image metadata, not an ESP-IDF install.
- `esp-hal` feature `unstable` for sleep, RTC, RMT, and `load_lp_code`. Those APIs are unstable.

## Limitations

| This example is NOT | Why |
| --- | --- |
| Capacitive touch | ESP32-C6 has no touch peripheral. GPIO 9 cannot stand in for one |
| A zero-microamp wake source | The LP core stays clocked. Current is higher than RTC-timer-only sleep and is not specified numerically here |
| An LP sensor hub | The LP image only watches one GPIO and wakes the HP core. No LP I2C and no LP UART |
| XIAO onboard-BOOT wake | BOOT is GPIO 9. Wire D0, or use the timer |

Extension path: measure LP-on current and record it in the power table. A second LP peripheral belongs in a new example.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **Zero-warning DevKitC build.** From `examples/lp-core-wakeup/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Both commands must exit 0 with no warnings. The build log shows the nested LP crate compiling. No `CMakeLists.txt` is created.

2. **Zero-warning XIAO build.** The wake pad stays GPIO 0, not GPIO 9.

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Both commands must exit 0 with no warnings.

### Manual — hardware required

These checks need the real RTC domain, the button, and a person watching the LED. There is no practical host substitute.

1. **GPIO wake and timer wake (DevKitC-1-N8, left USB-C).** Why manual: LP-core wake, persistent RAM, and deep sleep need the chip.
   - `cargo run --release`
   - Confirm a first-boot log, then sleep. Do not hold BOOT while resetting.
   - Wait ~20 s. Confirm one slow amber blink, a timer log line, and that the GPIO counter does not increment.
   - After the device sleeps again, press BOOT (GPIO 0) briefly. Confirm a wake well under a second, three green blinks, and a counter increment of 1.
   - Repeat the press twice more. The counter is monotonic.
   - Power-cycle. The next boot is a first boot and the counter reset.
   - The monitor stayed connected through the sleep intervals.
   - Blinks are visible in room light (cap 64/255).

2. **GPIO 9 does not wake XIAO; timer and D0 do.** Why manual: only a person can see which button was pressed, and USB Serial/JTAG drops on sleep so the LED is the instrument.
   - `cargo run --release --no-default-features --features board-xiao-c6`
   - Expect the USB monitor to exit on the first sleep. That is not a failure.
   - Press onboard BOOT (GPIO 9) several times during sleep. The user LED does not run the three-blink pattern.
   - A single slow blink about every 20 seconds is the timer path (active LOW).
   - If a button is wired to D0, press it during sleep. Confirm three blinks.

## Spec Files

- [FunctionalSpec.md](specs/FunctionalSpec.md)
- [CodingSpec.md](specs/CodingSpec.md)
- [TestSpec.md](specs/TestSpec.md)
