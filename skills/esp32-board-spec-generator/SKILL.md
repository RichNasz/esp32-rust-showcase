---
name: esp32-board-spec-generator
description: Generates structured board-spec Markdown files from vendor documentation for ESP32-C6 boards only. Produces pinout tables, peripheral maps, power budgets, flash/PSRAM inventory, and antenna notes. Refuses other SoCs. USB console is documented without sdkconfig.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, board-spec, hardware, pinout, vendor, datasheet, rust, esp-hal
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 Board Spec Generator

## When to use
When adding an ESP32-C6 board, or when vendor documentation has changed and the board spec needs to be refreshed.

Write board-specs only for **ESP32-C6** (DevKitC-C6 required, XIAO C6 also supported). If the board is another SoC, stop. Do not add it as hardware reference.

## Inputs
- Vendor product page URL or datasheet PDF (provide in the prompt)
- Board/module name and variant (e.g. "Seeed XIAO ESP32-C6")

## Workflow (follow exactly)
MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else.

1. Parse the provided vendor documentation (datasheet, product page, or schematic).
2. Extract and normalise: full GPIO pinout, peripheral assignments (I2C, SPI, UART, PWM, ADC), power supply rails and limits, flash size, antenna type, and any reserved or strapping pins. The ESP32-C6 has **ADC1 only**. It has **no DAC** and **no capacitive touch**. Do not add those peripherals. PSRAM on the v1 boards is **None**. Explicitly list which GPIOs are **RTC / LP capable (GPIO 0–7 only)** and note that GPIO 9 is **not** an RTC GPIO. Call the low-power core the LP core. Do not write that it runs ULP programs. Active-current examples must not use the Xtensa ~68 mA figure. Fill-in parts must not be `ESP32-WROOM-32`, Feather, or the HUZZAH32 `AP2112`.
3. Cross-reference against `shared-specs/rust-toolchain.md` for platform-level constraints (v1 chip = ESP32-C6; others not in v1). Do **not** add non-C6 SoCs as Supported in that file.
4. Generate `board-specs/<vendor>/<board-name>.md` using the standard section layout from `templates/BoardSpec.template.md`.
5. Add the standard agent-generated header to the output file.
6. USB Serial / Console: document UART-bridge chip vs native USB Serial/JTAG vs USB CDC, with TX/RX GPIOs. Do **not** emit `CONFIG_ESP_CONSOLE_*` sdkconfig lines — there is no sdkconfig. Firmware uses `esp-println` + `espflash`.
7. Cargo / chip notes (chip and board are Cargo features, not a set-target command):
   - Chip feature: `esp32c6`. Refuse any other SoC.
   - Board Cargo feature: DevKitC-C6 → `board-devkitc-c6`; XIAO C6 → `board-xiao-c6`. A new C6 board needs a new feature name in the FunctionalSpec before firmware uses it.
8. Output: file path written and a one-line summary of key specs (chip variant, flash, PSRAM, antenna, v1 status).

## Markdown Formatting Requirements

All generated board spec files must pass markdownlint with zero warnings:

- **Table separators**: use spaced style `| --- | --- |` — never compact `|---|---|`. The separator row must match the spacing of the content rows.
- **Lists**: always surround bullet and numbered lists with blank lines (one blank line before the first item, one blank line after the last item).
- **Fenced code blocks**: always specify a language identifier (e.g. ` ```sh `, ` ```toml `, ` ```rust `). Never use bare ` ``` ` without a language.
