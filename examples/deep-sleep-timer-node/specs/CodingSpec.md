<!-- See shared-specs/AIGenLessonsLearned.md — read before generating. -->

# Coding Specification — Deep Sleep Timer Node (ESP32-C6)

## Architecture

Single-shot Embassy firmware. Entry is an async main provided by `esp-rtos`; after
`esp_hal::init`, start the Embassy executor on TIMG0 + `FROM_CPU_INTR0` (C6). Each reset
(including a deep-sleep wake) runs main once: persist the boot count, log the wakeup
cause, blink the LED, arm the RTC timer, enter deep sleep. Deep sleep does not return;
the chip resets on wake. There is no long-lived task loop and no radio.

Top-level wiring lives in `src/main.rs`. Board-specific GPIO, LED kind, and polarity live
only in generated `src/board.rs`. Two compile-time LED paths, selected by mutually
exclusive Cargo board features:

- **WS2812 path** (default `board-devkitc-c6`): RMT generates 800 kHz NRZ. One colour
  frame, then black, then sleep.
- **GPIO LED path** (`board-xiao-c6`): simple active-LOW LED. One binary blink, then sleep.
  Do not run the RMT encoder on this board.

## Board Abstraction

`src/board.rs` is generated from `board-specs/`. It exposes LED GPIO, LED kind
(WS2812 vs simple GPIO), and polarity. `compile_error!` if both `board-devkitc-c6` and
`board-xiao-c6` are enabled, or if neither is. Application code must not hardcode pin
numbers. Chip feature is `esp32c6` only.

## Persistence

Store the boot counter in RTC-backed RAM that survives deep sleep and is lost on
power-cycle. Use `#[esp_hal::ram(persistent)]` or the equivalent
`#[ram(unstable(...))]` form required by the pinned `esp-hal` 1.2.1 proc-macro
(`persistent` is an unstable option and, on C6, belongs in RTC fast memory). Types must
be persistable bit-patterns as required by that attribute.

Treat uninitialized or corrupted persistent RAM as first boot: a distinctive magic plus
a checksum (or equivalent) so a reset that races zero-init cannot look like a huge
count. Increment the counter as the first application action after HAL/Embassy init,
before branching on wakeup cause. Do not use NVS or flash for this integer.

## Sleep and wakeup

Enable `esp-hal` feature `unstable` — sleep, RTC, and RMT APIs require it on this pin.
Arm an RTC timer wakeup for about 5 seconds measured from sleep entry (absolute
deadline so work between arming and sleep does not shorten the sleep). Enter deep
sleep via the 1.2-line `LowPower` / `sleep_deep` API. Panic only if no wakeup source is
armed; otherwise never abort the sleep loop.

Read wakeup cause after init. Empty cause is first boot / not-from-sleep. Timer is the
expected subsequent cause. Any other cause logs a warning and still sleeps.

This example has no radio, so there is no `esp-radio` teardown. GPIO 9 is not an RTC
wakeup pin; do not attempt button wakeup on the XIAO boot button.

Drive the LED to its inactive/dark level before sleep. There is no public per-pad hold
API on this HAL pin that the application must call; do not emit sdkconfig sleep
fragments.

## WS2812 path

Use `esp_hal::rmt` (`PulseCode` newtype on the 1.2 line). Encode 800 kHz NRZ (T0H ~300 ns
/ T0L ~900 ns, T1H ~900 ns / T1L ~300 ns) and a reset/latch of at least 50 µs low,
preferably in the RMT buffer. Hold the line LOW after a frame. Pack colours as GRB, not
RGB.

Write black once at init so the LED starts dark. Cap every channel at 64/255 (a
maximum, not a floor). Select one of red, green, blue, cyan, magenta, amber via the
hardware RNG (no software seed). Display that colour for about 100 ms, then black for
about 100 ms. Time those waits with Embassy, not busy-wait. Log GPIO, GRB order, and
that RMT is driving NRZ at INFO before the heartbeat.

## GPIO LED path

Drive the XIAO LED at the polarity documented in its board-spec (active LOW). On for
about 100 ms, off for about 100 ms. No PWM, no LEDC, no RMT — a brief binary blink
needs none of it.

## Key Constraints

- Sleep duration: 5 seconds from sleep entry.
- Active window budget: about 200 ms (heartbeat plus log). Keep the path linear.
- No heap. No `std`. No FreeRTOS. No `sdkconfig`. No NVS.
- Non-radio crates.io pins: `esp-hal` 1.2.1, `esp-rtos` 0.4.0,
  `esp-bootloader-esp-idf` 0.6.0. Do not git-pin the radio stack.
- The image must include an ESP-IDF-compatible application descriptor
  (`esp-bootloader-esp-idf`). Current `espflash` will not flash without it.
- `build.rs` emits `-Tlinkall.x` last.
- Embassy entry: `#[esp_rtos::main] async fn main(spawner: Spawner) -> !`. Do not use
  blocking `#[esp_hal::main]`.
- Zero `unwrap()` / `expect()` on fallible HAL in the wake path. Init may halt if
  clocks or RMT cannot start.

## Preferred Crates and APIs

- `esp-hal` RTC / `LowPower` for timer wakeup and deep sleep (reason: metal sleep APIs;
  there is no `esp_deep_sleep_start` / sdkconfig).
- `esp-hal` ram persistent attribute for the boot counter (reason: RTC RAM without NVS).
- `esp-hal` RMT for WS2812 (reason: hardware NRZ; a GPIO level produces no light).
- `esp-hal` RNG for palette index (reason: hardware entropy, no seed).
- `embassy-time` for the 100 ms heartbeat waits (reason: cooperative delay; no busy-wait).
- `log` + `esp-println` (`log-04`) for INFO operational events.

## Non-Functional Requirements

- Logging: INFO for first boot, timer wake + count, LED init, and sleep entry. WARN for
  unexpected wakeup causes. No debug in the heartbeat.
- Concurrency: one Embassy task is enough; do not spawn extra tasks.
- Error recovery: log RMT transmit errors and still enter sleep. Unrecognised wakeup
  causes must not abort.
- Allow the UART-bridge console a brief drain after the last log line so the monitor
  captures “sleeping” before the chip powers down.

## Gotchas

- Persistent RAM is retained across deep sleep but cleared on power-on. Document that;
  it is not a bug. A checksum/magic is required because a reset can skip zero-init.
- GPIO 9 cannot wake C6 from deep sleep. Timer-only is the correct XIAO design.
- A GPIO level on the DevKitC WS2812 produces no light — indistinguishable from an
  unsoldered LED jumper until NRZ + brightness ≥ room-light visibility are confirmed.
- Channel values above 64/255 are painfully bright; 8/255 is invisible in room light.
- XIAO LED is active LOW: forgetting polarity looks like “stuck on” or “never on”.
- DevKitC serial for persistence tests is the **left** USB-C UART-C port. Native USB
  Serial/JTAG (DevKitC right port, XIAO) drops the host CDC port on true deep sleep;
  `espflash monitor` exiting is expected, not a crash.
- Sleep, RTC, and RMT live behind `esp-hal` `unstable` on this pin — document that in
  the README.
- Do not mix crates.io `esp-rtos` 0.4.0 with git `esp-hal`. This example is non-radio.
- `sleep_deep` panics if no wakeup source is enabled or if the timer deadline is already
  too near to catch. Arm the 5 s deadline immediately before sleep, not at the start of
  a long active window.
