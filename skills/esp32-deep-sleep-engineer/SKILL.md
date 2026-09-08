---
name: esp32-deep-sleep-engineer
description: Designs and generates deep-sleep configurations for ESP32-C6 rust-on-metal firmware. Handles esp-hal RTC sleep, #[esp_hal::ram(persistent)] state, wakeup sources (timer, RTC GPIO 0–7, LP-core), power-budget calculations, and radio teardown before sleep.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, deep-sleep, rtc, low-power, wakeup, power-budget, esp-hal, rust, persistent-ram
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 Deep Sleep Engineer

## When to use
Any time a FunctionalSpec requires deep sleep, RTC-persistent state, or a sub-1 mA average
current target. Invoked automatically as a sub-workflow by `esp32-sdd-full-project-generator`
when sleep is specified.

## Workflow (follow exactly)
MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else.

1. Read the FunctionalSpec.md wakeup strategy and hardware list. Copy the power-budget tables
   from `shared-specs/power-budget-template.md` if the spec includes them.

2. Select wakeup source(s): RTC timer, RTC GPIO (ext1-style), LP-core, or combination; justify
   the choice.

   **ESP32-C6 RTC GPIOs are 0–7 only.** GPIO 9 (XIAO boot button) is **not** an RTC GPIO and
   cannot wake the chip from deep sleep. DevKitC-C6 boot is GPIO 0 (RTC-capable). When a
   board's only physical button is not RTC-capable, use timer-only wakeup (or an external
   button on LP GPIO 0–7). Document the limitation; do not invent a software workaround.

3. Calculate power budget:
   `(active_mA × active_s + sleep_µA × sleep_s) / period_s = avg_µA`.
   Include radio teardown time (~5–10 ms) in the active window when BLE / Wi-Fi / ESP-NOW is used.

4. Enable `esp-hal` feature `unstable` (sleep / RTC APIs require it on this HAL line) in
   addition to `esp32c6`. Document that honesty in the example README.

5. Generate sleep flow in Rust (`esp-hal` RTC / `rtc_cntl` sleep, not ESP-IDF
   `esp_deep_sleep_start`):
   - Configure wakeup sources from the spec.
   - Enter deep sleep (`sleep_deep` / equivalent on the 1.2 line). Deep sleep does not return;
     the chip resets on wake.
   - Do not emit `CONFIG_ESP_SLEEP_*`, `CONFIG_PM_*`, or any sdkconfig fragment.

6. Persist state that must survive deep sleep with `#[esp_hal::ram(persistent)]` (or the
   equivalent `#[ram(unstable(persistent))]` form on the pinned crate). There is no sdkconfig
   RTC-memory option. Types must be safe to persist (bit-pattern / zeroable as required by
   the attribute). Consider a checksum; a reset that races initialization can leave random
   bytes in the static.

7. **Tear down radios before sleep.** If the example uses `esp-radio`, stop advertising / STA /
   ESP-NOW and drop the connector so the radio is not left powered (`shared-specs/ble-patterns.md`
   for BLE). Skipping teardown leaves sleep current in milliamps instead of microamps. Coordinate
   with `esp32-ble-beacon-engineer` — that skill owns BLE teardown; this skill owns the sleep call.

8. **Load the LP-core image before the HP core sleeps** when the FunctionalSpec uses the LP core.
   The LP core cannot fetch its image from flash after the HP core has powered down the flash.
   Coordinate with `esp32-lp-core-engineer`. Then `lp_core.enable_wakeup()` (or equivalent) and
   enter HP deep sleep.

9. GPIO hold: if an output must keep its level across sleep (peripheral power gate, LED off),
   apply `esp-hal` GPIO / RTC hold **after** radio teardown and **before** `sleep_deep`.

10. USB console behaviour (document in README; sourced from AIGenLessonsLearned):
    - UART-bridge boards (ESP32-C6-DevKitC-1-N8 **left** USB-C, CH343P → UART0) keep the serial
      port alive across sleep. Monitor shows silence during sleep, output resumes on wake.
    - Native USB Serial/JTAG (DevKitC-C6 **right** USB-C; XIAO ESP32-C6) drops `/dev/cu.usbmodem*`
      when USB is powered down. `espflash monitor` / `cargo run` exits on the first sleep entry.
      There is no firmware workaround that preserves the USB connection during true deep sleep.
    Direct readers to the DevKitC-C6 UART-bridge port for counter-persistence tests; USB-native
    boards are LED-only observation.

11. Document the wakeup sequence and power budget table in the example README. Do not mention
    menuconfig or sdkconfig.
