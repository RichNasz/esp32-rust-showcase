<!-- See shared-specs/AIGenLessonsLearned.md — read before generating. -->

# Coding Specification — Breathing LED (ESP32-C6)

## Architecture

Single Embassy firmware. Entry is an async main provided by `esp-rtos`; after `esp_hal::init`,
start the Embassy executor on TIMG0 + `FROM_CPU_INTR0` (C6). Top-level wiring lives in
`src/main.rs`. Board-specific GPIO, LED kind, and polarity live only in generated
`src/board.rs`. Two compile-time paths, selected by mutually exclusive Cargo board features:

- **WS2812 path** (default `board-devkitc-c6`): RMT generates 800 kHz NRZ. Brightness is
  a software ramp in the Embassy loop, not a GPIO level.
- **GPIO LED path** (`board-xiao-c6`): simple active-LOW LED. Prefer LEDC hardware PWM
  with Embassy-timed duty steps. Do not busy-wait on fade-complete flags — LEDC
  interrupts are not available on this `esp-hal` pin. If LEDC is impractical, step
  duty with `embassy_time` only.

The breath always starts dark and brightens first, regardless of polarity.

## Board Abstraction

`src/board.rs` is generated from `board-specs/`. It exposes LED GPIO, LED kind
(WS2812 vs simple GPIO), and polarity. `compile_error!` if both `board-devkitc-c6` and
`board-xiao-c6` are enabled, or if neither is. Application code must not hardcode pin
numbers. Chip feature is `esp32c6` only.

## WS2812 Path

Use `esp_hal::rmt` (`PulseCode` newtype on the 1.2 line). Enable `esp-hal` feature
`unstable` — the RMT API requires it. Encode 800 kHz NRZ (T0H ~300 ns / T0L ~900 ns,
T1H ~900 ns / T1L ~300 ns) and a reset/latch of at least 280 µs low in the RMT
buffer. The WS2812B minimum is 50 µs; the longer gap is what latches clone parts.
Hold the line LOW after a frame. Pack colours as GRB, not RGB.

Write black once at init so the LED starts dark. Cap every channel at 64/255 (a
maximum, not a floor). Cycle a fixed table at that cap: red, green, blue, cyan,
magenta, amber — one colour per complete up+down breath. Scale all three channels by
the ramp fraction each step. Time steps with Embassy, not busy-wait. Log GPIO, GRB
order, and that RMT is driving NRZ at INFO before the loop.

## GPIO LED Path

Drive the XIAO LED at the polarity documented in its board-spec (active LOW). Invert
duty so “dark” is still LED off. Prefer LEDC PWM (low-speed only on C6) at a
flicker-free frequency. Step duty on an Embassy timer to meet the 4-second period.
Do not run the RMT WS2812 encoder on this board.

## Key Constraints

- Breathing period: 4 seconds total (2 s up, 2 s down) on both paths.
- No heap. No `std`. No FreeRTOS. No `sdkconfig`.
- Non-radio crates.io pins use an exact `=` so Cargo cannot select a later
  1.2.x: `esp-hal` `=1.2.1`, `esp-rtos` `=0.4.0`, `esp-bootloader-esp-idf`
  `=0.6.0`. Do not git-pin the radio stack.
- The image must include an ESP-IDF-compatible application descriptor
  (`esp-bootloader-esp-idf`). Current `espflash` will not flash without it.
  This crate is image metadata, not ESP-IDF.
- `build.rs` emits `-Tlinkall.x` last.
- Embassy entry: `#[esp_rtos::main] async fn main(spawner: Spawner) -> !`. Do not use
  blocking `#[esp_hal::main]`.
- Zero `unwrap()` / `expect()` on fallible HAL in the breathing loop. Init may halt
  if clocks or the LED peripheral cannot start.

## Preferred Crates and APIs

- `esp-hal` RMT for WS2812 (reason: hardware NRZ; bit-bang and GPIO levels cannot
  produce a visible WS2812 frame).
- `esp-hal` LEDC for the simple LED (reason: hardware PWM without spinning the core).
- `embassy-time` for the breath cadence (reason: cooperative delay; no busy-wait).
- `log` + `esp-println` (`log-04`) for INFO operational events.
- `esp-bootloader-esp-idf` for the application descriptor `espflash` requires
  (reason: the 2nd-stage bootloader and current `espflash` reject images without
  it; this is not an ESP-IDF project).

## Non-Functional Requirements

- Logging: INFO for driver init and loop start; no debug in the per-step hot path.
- Concurrency: one Embassy task is enough; do not spawn extra tasks unless needed.
- Error recovery: log RMT/LEDC transmit or duty errors and continue the loop.

## Gotchas

- A GPIO level on the DevKitC WS2812 produces no light — indistinguishable from an
  unsoldered LED jumper until NRZ + brightness ≥ room-light visibility are confirmed.
- Channel values above 64/255 are painfully bright; 8/255 is invisible in room light.
- XIAO LED is active LOW: forgetting to invert duty turns “fade up” into fade down.
- DevKitC serial is the **left** USB-C UART-C port; XIAO is USB Serial/JTAG. There is
  no sdkconfig console symbol.
- RMT and LEDC live behind `esp-hal` `unstable` on this pin — document that in the README.
- Do not mix crates.io `esp-rtos` 0.4.0 with git `esp-hal`. This example is non-radio.
- A missing application descriptor fails at `espflash` with "ESP-IDF App
  Descriptor missing" — the firmware never reaches the board. That is not a
  serial-port or `--release` problem.
