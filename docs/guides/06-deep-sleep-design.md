<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# 06 — Deep Sleep Design

Use `esp32-deep-sleep-engineer` whenever a FunctionalSpec sleeps the HP core.

## Mandatory first read

[`shared-specs/AIGenLessonsLearned.md`](../../shared-specs/AIGenLessonsLearned.md) and [`shared-specs/power-budget-template.md`](../../shared-specs/power-budget-template.md).

## Rules that bite on ESP32-C6

- **RTC GPIOs are 0–7 only.** GPIO 9 (XIAO boot button) cannot wake the chip from deep sleep. DevKitC-C6 boot is GPIO 0 (RTC-capable).
- **Tear down `esp-radio` before deep sleep.** Skipping teardown leaves the radio powered and sleep current in milliamps.
- **Load the LP-core image before the HP core sleeps.** The LP core cannot fetch from flash after HP powers down flash.
- **Persist counters with `#[esp_hal::ram(persistent)]`.** There is no sdkconfig RTC-memory option.
- **USB console during sleep:** UART-bridge (DevKitC **left** USB-C) stays connected. Native USB Serial/JTAG (DevKitC right port, XIAO C6) drops `/dev/cu.usbmodem*` and `espflash monitor` exits. That is expected, not a crash.

## Sleep / RTC APIs are unstable

Examples that use radio or some sleep / RTC APIs must enable `esp-hal` feature `unstable` and say so in the README. Those APIs may change with `esp-hal` minor releases.

## Invoke

> Activate esp32-deep-sleep-engineer skill

Then regenerate the example with `esp32-sdd-full-project-generator`.
