---
name: esp32-lp-core-engineer
description: Generates ESP32-C6 LP-core firmware (esp-lp-hal) plus HP-core load/handshake. Nested lp-core crate, load_lp_code before HP deep sleep, GPIO 0–7 only (GPIO 9 is not RTC). Invoked by the full-project generator when FunctionalSpec requires LP-core wakeup or LP firmware.
version: "1.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, lp-core, esp-lp-hal, rtc, gpio, wakeup, handshake, rust, no_std, esp32-c6
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 LP-Core Engineer

## When to use
Any time a FunctionalSpec requires the ESP32-C6 low-power RISC-V coprocessor: GPIO
monitoring during HP deep sleep, LP-core wakeup of the HP core, shared LP-RAM handshake,
or LP peripherals (LP GPIO / LP I2C / LP UART). Invoked automatically as a sub-workflow by
`esp32-sdd-full-project-generator` when the spec names LP-core firmware or `lp-core-wakeup`.

v1 is ESP32-C6 only. Do not generate ULP-FSM / Xtensa ULP assembly, and do not target
ESP32-S2/S3 ULP.

## Locked extras

The nested LP-core crate is an **allowed extra** on the locked layout
(`specs/example-project-structure-spec.md` + validator CHECK-14):

```
examples/<kebab-name>/
├── src/main.rs              ← HP core (esp-hal)
├── src/board.rs
├── lp-core/                 ← LP-core firmware crate (esp-lp-hal)
│   ├── Cargo.toml
│   └── src/main.rs          ← #![no_std] #![no_main]; // AGENT-GENERATED on line 1
├── build.rs                 ← builds lp-core and exposes the image path
├── Cargo.toml
└── …
```

## GPIO rule (non-negotiable)

**ESP32-C6 RTC / LP GPIOs are 0–7 only.** GPIO 9 (XIAO boot button) is **not** an RTC GPIO
and cannot be used for LP-core input, HP deep-sleep wakeup, or LP handshake signalling.
DevKitC-C6 boot is GPIO 0 (RTC-capable). If the board's only physical button is GPIO 9,
use an external button on LP GPIO 0–7 or timer wakeup. Document the limitation; do not
invent a software workaround (`AIGenLessonsLearned.md`).

The HP core must configure those pins as LP pins (`into_lp`) **before** loading LP firmware.
`esp-lp-hal` GPIO drivers assume the HP core already muxed the pads.

## Workflow (follow exactly)
MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else.

1. Read the FunctionalSpec wakeup / LP-core section and the referenced board-spec. Confirm
   every LP GPIO is in 0–7. **FAIL the generation** (do not emit code) if the spec asks to
   wake or sample GPIO 9 from the LP core — send the author back to the FunctionalSpec.

2. **Nested `lp-core/` crate** (`esp-lp-hal`, feature `esp32c6`, `#![no_std]` `#![no_main]`):
   - Pin `esp-lp-hal` from `esp-rs/esp-hal` git tag `esp-hal-v1.2.1` (in-tree 0.3.0) so it
     matches the HAL line. Target `riscv32imac-unknown-none-elf`.
   - Entry via `esp_lp_hal::prelude::*` `#[entry]`. Keep the LP image small: poll or wait on
     LP GPIO, write a handshake word to shared LP RAM, then `esp_lp_hal::wake_hp_core()` when
     the spec requires waking HP.
   - Line 1 of `lp-core/src/main.rs`: `// AGENT-GENERATED — DO NOT EDIT BY HAND`.
   - No heap, no Embassy, no `esp-radio` on the LP core.

3. **HP core handshake** (`esp-hal` `unstable` — `load_lp_code` is unstable):
   - Stop the LP core if it is already running.
   - Configure LP GPIOs from `board.rs` (`Output` / `Input` → `into_lp`).
   - `load_lp_code!("<path-to-lp-image>")` then `run` with `LpCoreWakeupSource::HpCpu` (or
     the wakeup source the spec requires).
   - Shared memory: a known LP-RAM address (C6 LP RAM is at `0x5000_0000`; pick a handshake
     word the LP image actually writes). Document the address in a comment referencing the
     spec — do not collide with the LP image's `.bss` / stack. Loading LP code may clear LP
     RAM; load first, then treat handshake memory as valid only after `run`.

4. **Load the LP-core image before the HP core sleeps.** The LP core cannot fetch its image
   from flash after the HP core has powered down the flash. Order:
   1. Radio teardown (if any) — `esp32-ble-beacon-engineer` / radio owner.
   2. Load + start LP firmware.
   3. `lp_core.enable_wakeup()` (C6: the LP core enables its own HP wake).
   4. HP `sleep_deep` — `esp32-deep-sleep-engineer` owns this call.

5. **`build.rs` on the HP crate** compiles the nested `lp-core` package (`cargo build
   --release` in `lp-core/`, or `cc`-free Cargo) and prints `cargo:rustc-env=` (or a known
   relative path) so `load_lp_code!` can find the ELF/bin. Do not shell out to CMake.

6. **Cargo features:** HP crate still has `esp32c6` + board features. Enable `esp-hal`
   `unstable`. If the HP crate is otherwise non-radio, keep crates.io `esp-hal` 1.2.1 +
   `esp-rtos` 0.4.0; the nested LP crate's git pin of `esp-lp-hal` does not force git-pinning
   the HP `esp-hal` unless radio is also present.

7. Document in the example README: LP GPIO map (0–7), handshake word, “load before HP sleep”,
   and that GPIO 9 cannot wake C6. State that LP / `load_lp_code` APIs are `unstable`.
