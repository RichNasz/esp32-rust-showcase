<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Pin Mapping Tables (v1 — ESP32-C6)

Facts come from `board-specs/`, not from a second hand-typed source. The tables below are a short extract. If they ever disagree with the board spec, the board spec wins and this page is regenerated. v1 firmware uses chip feature `esp32c6` only. Do not add other SoCs.

## Espressif ESP32-C6-DevKitC-1-N8 *(default)*

Board-spec: [`board-specs/espressif/esp32-c6-devkitc-1-n8.md`](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)

Cargo feature: `board-devkitc-c6`

| Function | GPIO | Notes |
| --- | --- | --- |
| WS2812 RGB LED | GPIO 8 | Addressable; also a strapping pin — leave floating at reset |
| Boot button | GPIO 0 | Active LOW; **RTC-capable** (LP_GPIO0) |
| UART0 TX / RX | GPIO 16 / 17 | Left USB-C UART bridge (`espflash` on the UART-C port) |
| Default I2C SDA / SCL | GPIO 6 / 7 | Also ADC1_CH6 / CH7 |
| Native USB D− / D+ | GPIO 12 / 13 | Right USB-C USB Serial/JTAG |
| LP / RTC GPIOs | GPIO 0–7 | Only these can wake from deep sleep |

USB console: UART bridge (`espflash` on the UART-C port). Deep sleep floor: ~7–15 µA.

## Seeed XIAO ESP32-C6

Board-spec: [`board-specs/seeed/xiao-esp32c6.md`](../../board-specs/seeed/xiao-esp32c6.md)

Cargo feature: `board-xiao-c6`

| Function | GPIO | Notes |
| --- | --- | --- |
| User LED (blue) | GPIO 15 | **Active LOW**; also a strapping pin |
| Boot button | GPIO 9 | Active LOW; **not** RTC-capable — cannot wake from deep sleep |
| D0 / A0 | GPIO 0 | LP_GPIO0 — use this (or 1–7) for RTC wakeup |
| I2C SDA / SCL (D4 / D5) | GPIO 22 / 23 | Default I2C |
| UART0 TX / RX (D6 / D7) | GPIO 16 / 17 | |
| LP / RTC GPIOs | GPIO 0–7 | GPIO 9 is **not** included |

USB console: USB Serial/JTAG. Deep sleep floor: ~15 µA. Native USB drops during deep sleep.

## WS2812 footnote

A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.
