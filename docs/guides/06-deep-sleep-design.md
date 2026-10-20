<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# 06 — Deep Sleep Design

Edit the example FunctionalSpec (observable behavior: wakeup source, what must survive sleep, and the current target), ask the agent to follow `esp32-deep-sleep-engineer`, then regenerate the crate with `esp32-sdd-full-project-generator`. Phrases to type to the agent, not in the shell: `Activate esp32-deep-sleep-engineer skill`, then `Activate esp32-sdd-full-project-generator`. A **spec** is Markdown a person writes. **Generate** means the agent writes the crate from that spec. **Regenerate** overwrites the previous crate output and leaves `specs/` in place. How specs work: [Using With AI Agents](using-with-ai-agents.md).

Read [`shared-specs/AIGenLessonsLearned.md`](../../shared-specs/AIGenLessonsLearned.md) and [`shared-specs/power-budget-template.md`](../../shared-specs/power-budget-template.md) before you write the sleep constraints. A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. The deep-sleep skill's first step is that lessons file.

## Rules that bite on ESP32-C6

- **RTC GPIOs are 0–7 only.** GPIO 9 (the XIAO boot button) is not an RTC GPIO and cannot wake the chip from deep sleep. DevKitC-C6 boot is GPIO 0, which is RTC-capable. If the only button is not RTC-capable, use timer wakeup or an external button on LP GPIO 0–7. Do not invent a software workaround.
- **Tear down `esp-radio` before sleep.** Stop advertising, STA, or ESP-NOW and deinitialize the radio. Skipping teardown leaves the radio powered, and sleep current stays in milliamps instead of microamps.
- **Load the LP-core image before the HP core sleeps.** The LP core cannot fetch its image from flash after the HP core powers the flash down. Program LP RAM first, then enter HP deep sleep.
- **Persist state with `#[esp_hal::ram(persistent)]`.** On the pinned crate the equivalent form is `#[ram(unstable(persistent))]`. There is no sdkconfig RTC-memory option. The type must be safe to persist. A checksum is worth specifying: a reset that races initialization can leave random bytes in the static.
- **The monitor depends on the USB port.** A UART-bridge board (DevKitC **left** USB-C, CH343P) stays connected. Silence during sleep, then log lines on wake, is expected. Native USB Serial/JTAG (DevKitC right USB-C, and the XIAO ESP32-C6) drops `/dev/cu.usbmodem*` when USB powers down. `espflash monitor` exits on the first sleep. There is no firmware workaround that keeps native USB up through true deep sleep. Use the DevKitC left port for counter-persistence logs. On native USB, watch the LED.

## Unstable APIs

Sleep, RTC, and radio need the `esp-hal` feature `unstable`. `esp-radio` is an unstable beta. Say that in the example README. Do not describe those APIs as frozen.

Do not hand-edit `src/` when the current is wrong. Change the FunctionalSpec (and CodingSpec constraints) and regenerate.
