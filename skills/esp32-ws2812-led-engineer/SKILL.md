---
name: esp32-ws2812-led-engineer
description: Generates the canonical WS2812B driver block for ESP32-C6 rust-on-metal examples using the esp-hal RMT peripheral. Enforces GRB colour order, 800 kHz NRZ timing, brightness cap 64/255, and board-spec GPIO via src/board.rs. Handles DevKitC (WS2812) vs XIAO C6 (simple GPIO) via Cargo board features.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, ws2812, rgb, led, rmt, neopixel, addressable-led, multi-board, esp-hal, rust
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 WS2812 LED Engineer

## When to use
Any time a FunctionalSpec lists a board with a WS2812 RGB LED (e.g. Espressif
ESP32-C6-DevKitC-1-N8 GPIO 8). Invoked automatically as a sub-workflow by
`esp32-sdd-full-project-generator` when a WS2812 LED is identified in any referenced
board-spec. Can also be run standalone to add WS2812 support to an existing example.

v1 boards: DevKitC-C6 (WS2812 on GPIO 8, primary, feature `board-devkitc-c6`) and
XIAO ESP32-C6 (simple active-LOW GPIO LED on GPIO 15, secondary, feature `board-xiao-c6`).
Do not introduce `sdkconfig` or menuconfig — board differences live in generated `src/board.rs`.

## Workflow (follow exactly)

1. **Read board specs** — for each board listed in the FunctionalSpec that has a WS2812
   LED, read its `board-specs/<vendor>/<board>.md`. Extract: GPIO number, LED type
   (WS2812 / SK6812 vs simple GPIO), polarity, and console type. Confirm the LED type is
   WS2812/WS2812B (single-wire NRZ, GRB byte order) — this skill is not suitable for
   APA102 (SPI) or other addressable LED protocols.

2. **Identify primary/secondary board assignments via Cargo features** — v1 has one chip
   feature (`esp32c6`) and two mutually exclusive board features. The WS2812 board
   (DevKitC-C6) is the default (`board-devkitc-c6`). The simple-GPIO board (XIAO C6) is
   selected with `--no-default-features --features board-xiao-c6`. Generated `src/board.rs`
   exposes LED GPIO, LED kind (WS2812 vs GPIO), and polarity. `compile_error!` if both
   board features or neither are enabled. Never hardcode pin numbers in `main.rs`.

3. **Drive WS2812 with `esp-hal` RMT** — do not bit-bang; do not use a GPIO level for a
   WS2812 (a GPIO level produces no visible output). Encode 800 kHz NRZ:

   | Symbol | Typical width | Notes |
   | --- | --- | --- |
   | 0-bit high | ~300 ns | T0H |
   | 0-bit low | ~900 ns | T0L |
   | 1-bit high | ~900 ns | T1H |
   | 1-bit low | ~300 ns | T1L |
   | Reset / latch | ≥ 50 µs low | Prefer encoding the reset in the RMT buffer, not relying on idle-low between separate transmissions |

   Both bit times sit in the WS2812B ±150 ns band. Use `esp_hal::rmt` (`PulseCode` newtype
   on the 1.2 line). Enable `esp-hal` `unstable` if the RMT API requires it on this pin.

4. **GRB byte order is mandatory.** WS2812B expects `{g, r, b}`, not RGB. RGB order
   produces wrong colours with no compile-time warning.

5. **Init sequence (high level):**
   - Construct `Rmt` from the peripheral clock.
   - Configure a TX channel on the LED GPIO from `board.rs`.
   - Hold the line LOW after a frame (WS2812 reset).
   - Write `(0, 0, 0)` once so the LED starts dark regardless of prior state.
   - Log at INFO (`log` + `esp-println`) the GPIO, GRB order, and that RMT is driving NRZ —
     this distinguishes software init from a missing solder jumper.

6. **GPIO LED path** — when `board.rs` says the LED is a simple GPIO (XIAO C6), drive it
   with `esp_hal::gpio` at the documented polarity. Do not run the RMT WS2812 encoder on
   that board.

7. **Apply mandatory brightness and visibility rules** (sourced from
   `shared-specs/AIGenLessonsLearned.md` — non-negotiable):

   - **Minimum brightness: 64/255 per active channel.** Values of 8/255 (~3%) are
     invisible under normal room lighting and indistinguishable from a hardware fault
     (e.g. unsoldered LED jumper). Always validate that CodingSpec-specified colours
     meet this floor before generating the source.
   - **Minimum blink count: 3 repetitions** for any pass/fail/status indicator. A
     single 500 ms flash is impossible to observe reliably in a real demo environment.
   - **Scope**: these rules govern the *application-layer* colours and timing that the
     caller specifies in CodingSpec.md. This skill does not hardcode colours — it
     enforces the rules in review and flags any CodingSpec values that fall below the
     minimums before generating code.

8. **Dependencies** — no extra ESP-IDF components. RMT lives in `esp-hal`. Do not add
   CMake `REQUIRES`, Kconfig symbols, or `sdkconfig.defaults.<target>` fragments.
