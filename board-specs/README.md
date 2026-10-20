<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# board-specs/

A **board spec** is the Markdown description of one board's pins, LED, USB, and power. The agent turns those facts into `src/board.rs`. People do not hardcode pin numbers in `main.rs`. How to add one: [Creating new board specs](../docs/guides/creating-new-board-specs.md). The short path is [guide 05](../docs/guides/05-add-board-spec.md).

v1 firmware compiles only for ESP32-C6 (`esp32c6`). **Chip ≠ board.** The chip feature selects the SoC. A board spec selects pins and USB. Required board: **ESP32-C6-DevKitC-1-N8**. Also supported: **Seeed XIAO ESP32-C6**. Do not add other SoCs.

## Boards

| Folder | Board | Cargo feature | Spec |
|---|---|---|---|
| `espressif/` | ESP32-C6-DevKitC-1-N8 *(default)* | `board-devkitc-c6` | [esp32-c6-devkitc-1-n8.md](espressif/esp32-c6-devkitc-1-n8.md) |
| `seeed/` | XIAO ESP32-C6 | `board-xiao-c6` | [xiao-esp32c6.md](seeed/xiao-esp32c6.md) |

## What a board spec contains

- Module / SoC
- Pin assignment (GPIO to peripheral)
- Strapping and reserved pins
- Power rails, and whether a LiPo charger is present
- Flash size
- Antenna
- USB console: UART bridge (`espflash` on the UART-C port), USB Serial/JTAG, or USB CDC. There is no `sdkconfig`
- Onboard LED and buttons, and the C6 rule that only GPIO 0–7 are RTC / LP pads

## Adding a board

1. Give the agent a vendor product page or datasheet.
2. Ask the agent (not the shell): "Activate esp32-board-spec-generator".
3. The agent writes `board-specs/<vendor>/<board-name>.md`.
4. Add that board to the example's FunctionalSpec checklist, then regenerate the docs so Supported Boards tables pick it up.

v1 still accepts only ESP32-C6 boards. A new board spec does not add a chip feature.
