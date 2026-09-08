<!-- AGENT-GENERATED — do not edit by hand; regenerate via esp32-board-spec-generator -->
<!-- Generated: 2026-02-22 -->

# Board Spec — Adafruit HUZZAH32 (ESP32 Feather)

## Module / SoC
| Item | Value |
|---|---|
| SoC | ESP32-D0WDQ6 (dual-core Xtensa LX6, 240 MHz) |
| Module | ESP32 WROOM-32 |
| Flash | 4 MB SPI NOR |
| PSRAM | None |
| Antenna | PCB trace (WROOM-32 integrated) |

## Onboard Peripherals
| Item | GPIO | Notes |
|---|---|---|
| Red LED (status) | GPIO 13 | Active HIGH; shared with SPI MOSI on default SPI bus |
| Battery voltage divider | GPIO 35 (ADC1_CH7) | Reads LiPo voltage |
| Charge indicator (input) | GPIO 35 | See Adafruit schematic |
| USB-UART | TX=GPIO1, RX=GPIO3 | CP2104 |

## Pin Assignment Table (key GPIOs)
| GPIO | Function |
|---|---|
| 0 | Boot/strapping; also ADC2_CH1 |
| 2 | Strapping pin (must float/low at boot) |
| 4 | ADC2_CH0, touch |
| 5 | SPI CS / strapping |
| 12 | Strapping (MTDI) — keep LOW at boot for 3.3 V flash |
| 13 | Onboard red LED, SPI MOSI |
| 14 | SPI CLK |
| 15 | SPI MISO / strapping |
| 21 | I2C SDA (default) |
| 22 | I2C SCL (default) |
| 34,35,36,39 | Input-only ADC pins |

## Power
| Rail | Detail |
|---|---|
| 3V3 | 500 mA max from AP2112 LDO |
| VBAT | JST connector; MCP73831 charger from USB |
| VUSB | USB 5 V input |

## USB Serial / Console

USB-UART bridge (CP2104) on TX=GPIO1, RX=GPIO3. Classify as **UART bridge (`espflash` on the UART-C port)**. There is no `sdkconfig`. Firmware logs via `esp-println`; flash/monitor via `espflash`.

Do **not** emit `CONFIG_ESP_CONSOLE_*`.

This SoC is **not in v1**. v1 firmware compiles only for ESP32-C6.

## Cargo chip / board features

v1 firmware generation consumes **ESP32-C6** only. This SoC is **not in v1**.

| Item | Value |
| --- | --- |
| SoC | ESP32 |
| Chip Cargo feature | not in v1 |
| Board Cargo feature | none |
| RTC / LP GPIOs | Classic ESP32 RTC GPIO set. Not a v1 firmware target. |
