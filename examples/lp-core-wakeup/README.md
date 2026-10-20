<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# LP-Core GPIO Wakeup — ESP32-C6

> LP-core GPIO wakeup on RTC GPIO 0–7 (C6 has no capacitive touch).

## Implementation status

**Firmware generated.** Crate generated 2026-10-20 (C6 LP-core GPIO 0–7). Not capacitive touch. A host build is not a board pass.

A spec is Markdown a person writes; the agent generates the crate. See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

LP-core APIs and `load_lp_code` are unstable. HP sleep, RTC, RMT, and `load_lp_code` use `esp-hal` feature `unstable`. `esp-lp-hal` 0.3.0 is git-pinned at tag `esp-hal-v1.2.1` in the nested `lp-core/` crate only. The HP crate stays on crates.io `esp-hal` 1.2.1 because this example has no radio. Those APIs can change with the next `esp-hal` minor.

## Overview

The HP core loads a small LP-core image into LP RAM, then deep-sleeps. The LP core watches GPIO 0 (active LOW). An edge writes a handshake word and wakes the HP core. A backup RTC timer of about 20 seconds wakes the HP core if the button is never pressed. C6 has no capacitive touch. Do not treat this as a touch-pad demo.

Deep sleep does not return. Each HP reset classifies the wake, blinks, **loads the LP image again**, arms the backup timer, and sleeps. The image is loaded before sleep: the LP core cannot fetch code from flash after the HP core powers flash down.

- **LP-core / GPIO 0 wake:** three blinks (~200 ms on / 200 ms off). The persistent GPIO counter increments by 1.
- **Timer wake (~20 s):** one slow blink (~500 ms). The GPIO counter does not increment.
- **First boot:** a log line, then sleep. The counter starts at 0 and resets on power-cycle.
- **XIAO BOOT (GPIO 9)** is not an RTC or LP GPIO. It cannot wake the chip from deep sleep. Use D0 / GPIO 0, or watch the timer blinks.

C6 RTC / LP GPIOs are **0–7 only**. DevKitC BOOT is GPIO 0. Press it only after the chip has booted. Holding it across reset enters ROM download mode. A stuck low line wakes, blinks, and sleeps again. It does not spin on the HP core.

The handshake word is `0x4750_494F` at LP RAM `0x5000_2000` (C6 LP RAM starts at `0x5000_0000`). The LP image writes it after a debounced active-LOW edge. The HP core reads it before `LpCore::new`, which clears LP RAM. A timer wake ignores a stale word and does not increment the counter. If the word and the HP wakeup cause disagree, the firmware logs a warning and trusts the HP cause for the blink pattern.

DevKitC uses the WS2812 on GPIO 8 (RMT, GRB, 800 kHz NRZ, brightness cap 64/255): green for a GPIO wake, amber for a timer wake, black before sleep. A GPIO level on GPIO 8 produces no light. XIAO uses GPIO 15, active LOW, with the same timings and no colour.

LP-on sleep current is **above** the ~7–15 µA RTC-only floor and is **unmeasured**. Do not copy ESP32-S3 touch-current numbers.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- Optional on XIAO: a momentary button from D0 (GPIO 0) to GND
- Counter-persistence logs: DevKitC **left** USB-C (UART bridge). See [Toolchain setup](../../docs/guides/01-toolchain-setup.md).

ESP-IDF is not required. Do not install `espup` or the `esp` rustup channel. There is no `idf.py`, no `sdkconfig`, no ULP assembly, and no CMake.

## Supported Boards

The chip feature `esp32c6` is the SoC. Board features select pins. This is not ESP-IDF `idf.py set-target`.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected (left USB-C UART bridge) | WS2812: three green blinks (~200 ms) on GPIO wake, one slower amber blink (~500 ms) on timer wake, off before sleep | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW, same timings. No colour | ~15 µA |

The first-time difference that matters is the serial port. On DevKitC-C6, use the **left** USB-C UART bridge: the monitor stays connected and is silent during sleep. On XIAO, USB Serial/JTAG drops on the first sleep and `cargo run` exits; the port comes back on wake. That is expected, not a crash. Treat XIAO as LED-only observation unless one cycle of log is enough. The right USB-C port on DevKitC is the same USB Serial/JTAG drop.

GPIO 9 on XIAO cannot wake the chip from deep sleep. It is not an RTC or LP GPIO. Wire a button to D0 (GPIO 0), or use the timer blinks. The wake pad stays GPIO 0 on both boards.

The deep-sleep floor column is the board's RTC-only figure. This example leaves the LP core clocked, so sleep current is **above** that floor and is unmeasured.

## Build & Flash

From `examples/lp-core-wakeup/`:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

`build.rs` compiles the nested `lp-core/` crate with Cargo (no CMake) and embeds the ELF. Do not flash the LP ELF by itself. Open this example folder, not `lp-core/` alone, when you want the HP image.

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

> **Never edit generated source files directly.** Change behaviour in the spec files below, then regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder** (not the repository root) so rust-analyzer uses this crate. The nested `lp-core/` package is built by `build.rs`; it is not a second workspace root.

**File → Open Folder… → select `examples/lp-core-wakeup/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target + `espflash` runner). Do not open the top-level repository folder as your active workspace while developing this example.

## Expected behavior

DevKitC, left USB-C. Levels are `INFO` unless noted. The parenthetical on `LP image loaded` is the start of the embedded image hash, not a fixed string of dots.

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

XIAO logs the same wake lines until the USB port drops on the first sleep. These lines describe the spec. They are not a recorded board pass.

## Key Concepts

- Nested `lp-core/` firmware on `esp-lp-hal` (`#![no_std]` `#![no_main]`), built by the HP `build.rs` and copied into LP RAM with `load_lp_code`. LP-core APIs and `load_lp_code` are unstable.
- Load and start the LP image, then `enable_wakeup()`, then HP `sleep_deep`. Flash is off during deep sleep, so loading afterwards is impossible.
- HP wakeup sources: LP core (GPIO 0, active LOW, pull-up armed on the HP side before the image runs) and a ~20 s RTC timer. GPIO 9 is not a wake source.
- Persistent GPIO counter: `#[esp_hal::ram(unstable(rtc_fast, persistent))]` (or the equivalent persistent form), magic plus checksum. LP RAM is not that store. The reload clears the handshake word.
- Embassy time for the blinks. DevKitC WS2812 via RMT (800 kHz NRZ, GRB, cap 64/255). XIAO is a plain active-LOW GPIO. `esp-hal` feature `unstable` covers sleep, RTC, RMT, and `load_lp_code`.
- HP crate is non-radio: crates.io `esp-hal` 1.2.1 and `esp-rtos` 0.4.0. The git pin is only the nested LP crate (`esp-lp-hal` 0.3.0 at `esp-hal-v1.2.1`).
- `esp_bootloader_esp_idf::esp_app_desc!()` so current `espflash` accepts the image. That crate is image metadata, not an ESP-IDF install.

## Power budget

HP active current is CPU-only (~25 mA). Sleep with the LP core running is **above** the RTC-only floor of ~7–15 µA. Do not copy ESP32-S3 touch-wakeup figures or classic ESP32 ULP figures. Do not claim a microamp number until it is measured on this board. The v1 success bar is a functional wake, not a current-meter reading.

| Mode | Typical current | Duration | Energy per cycle |
| --- | --- | --- | --- |
| Active (HP CPU, no RF) | ~25 mA | ~1.5 s GPIO-wake blinks; ~0.6 s timer blink | ~10 µAh / ~4 µAh |
| Deep sleep + LP core running | above ~15 µA; unmeasured | up to 20 s | — |
| **Total per duty cycle** | — | up to ~21.5 s | dominated by the active blink if LP sleep stays well under 1 mA |

**Sleep wakeup source:** LP-core GPIO 0, backup RTC timer
**Wakeup interval:** 20 seconds when the button is idle

UART-bridge (DevKitC left USB-C) keeps the monitor across sleep. Native USB Serial/JTAG drops the port.

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

From `examples/lp-core-wakeup/`:

1. **T-A1 — Zero-warning DevKitC build.** `cargo build --release` and `cargo clippy --release -- -D warnings` exit 0 with no warnings. The build log shows the nested `lp-core/` crate compiling. No `CMakeLists.txt` is created. The wake pad is GPIO 0.
2. **T-A2 — Zero-warning XIAO build.** The wake pad stays GPIO 0, not GPIO 9.

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Both commands exit 0 with no warnings.

### Manual — hardware required

1. **T-M1 — GPIO wake and timer wake (DevKitC-1-N8, left USB-C).** Why automation is impractical: LP-core wake, persistent RAM, and deep sleep need the real RTC domain and a person watching the LED. There is no practical host substitute.
   - `cargo run --release`
   - Confirm a first-boot log, then sleep. Do not hold BOOT while resetting.
   - Wait ~20 s. Confirm one slow amber blink, a timer log line, and that the GPIO counter does not increment.
   - After the device sleeps again, press BOOT (GPIO 0) briefly. Confirm a wake well under a second, three green blinks, and a counter increment of 1.
   - Repeat the press twice more. The counter is monotonic.
   - Power-cycle. The next boot is a first boot and the counter reset.
   - The monitor stayed connected through the sleep intervals. Blinks are visible in room light (cap 64/255).
2. **T-M2 — GPIO 9 does not wake XIAO; timer and D0 do.** Why automation is impractical: only a person can see which button was pressed, and USB Serial/JTAG drops on sleep so the LED is the instrument.
   - `cargo run --release --no-default-features --features board-xiao-c6`
   - Expect the USB monitor to exit on the first sleep. That is not a failure.
   - Press onboard BOOT (GPIO 9) several times during sleep. The user LED does not run the three-blink pattern.
   - A single slow blink about every 20 seconds is the timer path (active LOW).
   - If a button is wired to D0, press it during sleep. Confirm three blinks.

## Spec Files

Edit these, then ask the agent `Activate esp32-sdd-full-project-generator` (typed to the agent, not a shell command). See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md) — observable behavior.
- [specs/CodingSpec.md](specs/CodingSpec.md) — architecture and constraints, with no code.
- [specs/TestSpec.md](specs/TestSpec.md) — how to check.
