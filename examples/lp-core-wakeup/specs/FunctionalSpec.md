# LP-Core GPIO Wakeup (ESP32-C6)

## Overview

Human-interface deep-sleep example for rust-on-metal ESP32-C6 (`no_std` `esp-hal`). The
HP core loads a small LP-core image into LP RAM, then enters deep sleep. The LP core
watches an RTC GPIO. An active-LOW edge makes the LP core wake the HP core. The HP core
blinks the onboard LED three times, logs the event, increments a persistent counter, loads
the LP image again, and sleeps. A backup RTC timer wakes the HP core if the button is
never pressed, so the device cannot sleep forever. Chip is ESP32-C6 only.

C6 has no capacitive-touch peripheral. This example replaces that class of demo. Do not
generate touch-pad calibration, `esp_sleep_enable_touchpad_wakeup`, or an ESP32-S3 target.

## Implementation status

**Specs only** as of 2026-10-20. These Functional, Coding, and Test specs are the current design (C6 LP-core GPIO 0, not capacitive touch). There is no `Cargo.toml` and no `src/` until `esp32-sdd-full-project-generator` runs. The catalog table in `specs/showcase-master-spec.md` is the source of truth across examples.

## Target Hardware

v1 chip: **ESP32-C6** only (`esp32c6`). Other SoCs are not in v1.

| Item | v1 value |
| --- | --- |
| **SoC** | ESP32-C6 only |
| **Chip Cargo feature** | `esp32c6` |
| **rustc target** | `riscv32imac-unknown-none-elf` (HP and LP images) |

### Board Compatibility Checklist

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [x] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add non-C6 boards. Adafruit HUZZAH32, Seeed XIAO ESP32S3, Seeed XIAO ESP32-C5, and
YEJMKJ ESP32-S3 boards are **not in v1**.

RTC / LP GPIOs on C6 are **0–7 only**. GPIO 9 (XIAO boot button) is **not** an RTC GPIO.
It cannot be an LP input, an HP deep-sleep wake pin, or an LP handshake line. DevKitC
BOOT is GPIO 0 and is the default pad. On XIAO, GPIO 0 is D0: use an external button
there, or observe timer-only wakes if no button is wired.

## Features

- Nested `lp-core/` crate (`esp-lp-hal`, feature `esp32c6`, `#![no_std]` `#![no_main]`).
  The HP `build.rs` builds that crate and passes the image to the HP firmware. No CMake.
- Before every HP deep sleep, the HP core loads that image into LP RAM and starts the LP
  core. The LP core cannot fetch code from flash after the HP core powers flash down.
  Loading after sleep entry is too late.
- LP firmware polls LP GPIO 0 (active LOW, pull-up configured from the HP side before the
  image runs). On an edge, it writes a handshake word into shared LP RAM and wakes the
  HP core. No heap, no Embassy, and no radio on the LP core.
- HP wakeup sources, both armed:
  - LP-core wake from GPIO 0.
  - Backup RTC timer of about **20 seconds**, so a missing button still produces a cycle.
- On an LP / GPIO wake: increment a persistent HP boot counter (magic + checksum,
  `#[esp_hal::ram(persistent)]` or the equivalent `#[ram(unstable(...))]` form). Log the
  handshake word and the count. Blink **three** times (~200 ms on / 200 ms off). The
  counter does not increment on a timer-only wake.
- On a timer wake: do not increment the GPIO counter. Log that the timer fired. Blink
  **once**, slowly (~500 ms on). Then load the LP image again and sleep.
- If the handshake word and the HP wakeup cause disagree, log a warning, trust the HP
  wakeup cause for the blink pattern, and still sleep. Do not hang.
- LED:
  - **DevKitC-C6:** WS2812 GPIO 8 via RMT, GRB, cap 64/255. Three green blinks on GPIO
    wake, one slower amber blink on timer wake, black before sleep. A GPIO level on
    GPIO 8 produces no light.
  - **XIAO ESP32-C6:** GPIO 15 active LOW. Same timings, binary on/off. No RMT.
- Pin numbers live only in generated `src/board.rs`. The LP image uses the GPIO number
  the board module defines for the wake pad (GPIO 0 on both boards). Do not hardcode a
  different pad for XIAO BOOT.
- Board selection is a Cargo feature. Default is DevKitC-C6. XIAO:
  `cargo run --release --no-default-features --features board-xiao-c6`.
- Serial INFO: first boot vs LP-GPIO wake vs timer wake, the persistent count, and sleep
  entry. DevKitC **left** USB-C UART-C stays connected through sleep. Native USB
  Serial/JTAG drops the port on sleep; XIAO observation is the LED unless one cycle of
  log is enough.
- HP crate is non-radio: crates.io `esp-hal` 1.2.1 and `esp-rtos` 0.4.0. The nested LP
  crate git-pins `esp-lp-hal` at tag `esp-hal-v1.2.1` (in-tree 0.3.0). That pin must not
  force the HP crate onto the git HAL.

## Wakeup Strategy

**LP-core GPIO 0** (active LOW) **plus RTC timer** (~20 s from HP sleep entry).

GPIO 9 is not used. Deep sleep does not return; the HP core resets on wake. Sleep, RTC,
RMT, and `load_lp_code` require `esp-hal` feature `unstable` on the HP crate. LP-core
APIs are unstable. The README must say so.

## Connectivity

None. No `esp-radio`.

## Security Requirements

None. No secrets, no radio, no flash data partition.

## Hardware Dependencies

- Board-spec: `board-specs/espressif/esp32-c6-devkitc-1-n8.md` (WS2812 GPIO 8; BOOT / LP
  GPIO 0; flash/monitor on the **left** USB-C UART-C port).
- Board-spec: `board-specs/seeed/xiao-esp32c6.md` (user LED GPIO 15 active LOW; BOOT GPIO 9
  is not LP-capable; external button on D0 / GPIO 0 if GPIO wake is to be demonstrated).
- Optional: a momentary button from GPIO 0 to GND on boards where the onboard button is
  not GPIO 0. DevKitC BOOT is already that button. Do not hold it across reset (download
  strap).

The generator auto-invokes:

- `esp32-lp-core-engineer` — nested LP crate, load-before-sleep, GPIO 0–7 rule
- `esp32-deep-sleep-engineer` — HP RTC timer, persistent RAM, `sleep_deep`
- `esp32-ws2812-led-engineer` — DevKitC WS2812 RMT path

## Power Budget

HP active current is CPU-only (~25 mA). Sleep with the LP core running is **above** the
RTC-only floor of ~7–15 µA. Do not copy ESP32-S3 touch-wakeup figures (~130 µA) or classic
ESP32 ULP figures. Do not claim a microamp number until it is measured on this board.
The success bar for v1 is functional wake, not a current meter reading.

| Mode | Typical current | Duration | Energy per cycle |
|---|---|---|---|
| Active (HP CPU, no RF) | ~25 mA | ~1.5 s GPIO-wake blinks; ~0.6 s timer blink | ~10 µAh / ~4 µAh |
| Deep sleep + LP core running | above ~15 µA; unmeasured | up to 20 s | — |
| **Total per duty cycle** | — | up to ~21.5 s | dominated by the active blink if LP sleep stays well under 1 mA |

**Sleep wakeup source:** LP-core GPIO 0, backup RTC timer
**Wakeup interval:** 20 seconds when the button is idle
**Observation:** UART-bridge (DevKitC left USB-C) keeps the monitor across sleep. Native USB Serial/JTAG drops the port.

The README must say the LP-on sleep current is unmeasured, and that it will not match a
timer-only deep sleep.

## Limitations

| This example is NOT | Why |
|---|---|
| Capacitive touch | ESP32-C6 has no touch peripheral. GPIO 9 cannot stand in for one |
| A zero-microamp wake source | The LP core stays clocked. Current is higher than RTC-timer-only sleep and is not specified numerically here |
| An LP sensor hub | The LP image only watches one GPIO and wakes the HP core. No LP I2C and no LP UART |
| XIAO onboard-BOOT wake | BOOT is GPIO 9. Wire D0, or use the timer |

Extension path: measure LP-on current and record it in this power table. A second LP
peripheral belongs in a new example.

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf` (default DevKitC-C6). The build must compile the nested `lp-core/` crate and link its image into the HP firmware.
- `cargo clippy --release -- -D warnings` is clean for the default features, including the LP crate if it is part of the workspace invocation the generator chooses. The HP package clippy must be clean.
- `cargo build --release --no-default-features --features board-xiao-c6` and the matching clippy invocation are clean.
- **`cargo run --release`** on DevKitC-C6 (left USB-C) shows first boot, then sleep. A BOOT press (GPIO 0) after the strap is released wakes the chip within about 100 ms of the press being seen by the LP core, blinks green three times, and increments the persistent count by 1. With no press, a timer wake about 20 seconds later blinks amber once and does not increment the count. The monitor stays up across sleep.
- The count survives repeated GPIO wakes and resets on power-cycle.
- XIAO: `cargo run --release --no-default-features --features board-xiao-c6`. Timer blinks are visible on GPIO 15. GPIO 9 does not wake the chip. An external button on D0 produces the three-blink path. USB monitor drop on sleep is expected.
- The LP image is loaded on every HP cycle before sleep, including after a timer wake.
- No hang if the button is held, and no use of GPIO 9 as an LP pin.
