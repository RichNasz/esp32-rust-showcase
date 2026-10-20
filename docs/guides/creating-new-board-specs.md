<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Creating New Board Specs

This is the long form of [05 — Add a Board Spec](05-add-board-spec.md). The walkthrough board is the Espressif **ESP32-C6-DevKitC-1-N8**, which is the required default in v1.

A **board spec** is the Markdown description of one board's pins, LED, USB, and power; the agent turns those facts into `src/board.rs`. A **spec** is Markdown a person writes. A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You say the activate phrase to the agent, not in the shell. **Generate** means the agent writes that Markdown from the vendor page. Do not hand-edit a generated board spec. Change the source facts and generate again.

v1 is ESP32-C6 boards only. A board spec does not add a chip feature. Cargo chip feature `esp32c6` stays the SoC. Board features in v1 are `board-devkitc-c6` (default) and `board-xiao-c6`. Do not add other SoCs.

The filled file for this walkthrough is [`board-specs/espressif/esp32-c6-devkitc-1-n8.md`](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md). The section order is [`templates/BoardSpec.template.md`](../../templates/BoardSpec.template.md). This page does not publish a second template.

## When to generate a board spec

| Situation | Action |
| --- | --- |
| A FunctionalSpec names a C6 board that has no board spec yet | Generate the board spec first, then name it in the FunctionalSpec checklist |
| A board spec is missing power, strapping, or USB facts | Regenerate from the vendor page. Do not hand-edit the file |
| The vendor shipped a revision with different pins | Regenerate with the updated source URL |
| A draft contains unverified pin numbers | Regenerate from the schematic or the vendor wiki |

## What you give the agent

### Authoritative source

Prefer the vendor's own page:

- Espressif dev kits: `docs.espressif.com`
- Seeed: `wiki.seeedstudio.com`

DevKitC-1-N8 source used for the current board spec:

`https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32c6-devkitc-1/index.html`

### Facts to verify before you ask

| Fact | How to verify |
| --- | --- |
| Onboard LED GPIO, type (simple GPIO or WS2812), and polarity | Vendor pinout or schematic |
| Strapping pins | ESP32-C6 Technical Reference Manual plus the board schematic |
| PSRAM (none, SPI, or OPI) and size | Module datasheet |
| Deep-sleep current | Vendor power table |
| USB console | UART bridge chip, USB Serial/JTAG, or USB CDC. Do not emit `CONFIG_ESP_CONSOLE_*`. There is no sdkconfig |
| RTC / LP GPIOs | On the C6, GPIO **0–7 only**. GPIO 9 is not RTC-capable |

### Slug

Lowercase kebab: `esp32-c6-devkitc-1-n8`, `xiao-esp32c6`. The file lands at `board-specs/<vendor-slug>/<board-slug>.md`.

### Example invocation

Type this to the agent:

```
Activate esp32-board-spec-generator skill

Source: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32c6-devkitc-1/index.html
Board: Espressif ESP32-C6-DevKitC-1-N8

Pre-verified facts:
- Onboard LED: WS2812 on GPIO 8
- Boot button: GPIO 0 (RTC-capable, active LOW)
- Strapping pins: GPIO 0, 4, 5, 8, 9, 15
- PSRAM: none
- Deep sleep: ~7–15 µA
- USB: left USB-C = UART bridge (CH343P → UART0 GPIO 16 TX / GPIO 17 RX); right USB-C = USB Serial/JTAG
```

## Section list

Every board spec uses these sections, in this order, from `templates/BoardSpec.template.md`. Do not invent a parallel outline.

| Section | What it records |
| --- | --- |
| **Module / SoC** | Chip, module, flash, PSRAM, antenna, form factor |
| **Power Management** | 3V3 rail, USB input, battery input, deep-sleep current |
| **Onboard Peripherals** | LEDs and buttons: GPIO, polarity, simple GPIO versus WS2812 |
| **GPIO & Pin Rules / Warnings** | The four subsections below |
| Strapping Pins (do not drive during reset/boot) | Pins sampled at reset, and the safe default |
| ADC Restrictions | Which ADC the SoC actually has, and what not to use during radio |
| USB Serial / Console | Exactly one of: UART bridge (`espflash` on the UART-C port), USB Serial/JTAG, or USB CDC |
| Reserved / Internal Pins (do not use in user code) | Flash and USB pads the application must leave alone |
| **Complete Pin Mapping Table** | The two subsections below |
| User-Accessible Pads | Header label, GPIO, ADC, and default peripheral |
| Internal / Special GPIOs | Onboard LED and other pads that are not on the Arduino-style header |
| **Board Advantages for SDD Projects** | What a FunctionalSpec author should know about this board |
| **Cargo chip / board features** | `esp32c6` plus one board feature. Not `idf.py set-target` |

USB console wording is one of:

- UART bridge (`espflash` on the UART-C port) — name the bridge chip and the TX/RX GPIOs
- USB Serial/JTAG
- USB CDC

If a part is absent, write `None`. Do not drop the section.

## Walkthrough: ESP32-C6-DevKitC-1-N8

These facts match [`board-specs/espressif/esp32-c6-devkitc-1-n8.md`](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md). The pin table in that file is the source. This page does not retype a second full map.

| Fact | Value |
| --- | --- |
| SoC / module | ESP32-C6, module ESP32-C6-WROOM-1 (PCB antenna). N8 means 8 MB flash. PSRAM: none |
| LED | WS2812 on GPIO 8 (GRB, 800 kHz NRZ). A GPIO level produces no visible output — use RMT. GPIO 8 is also a strapping pin |
| Boot button | GPIO 0, active LOW, RTC-capable (LP GPIO 0) |
| Strapping | GPIO 0, 4, 5, 8, 9, 15. Leave them floating at reset |
| Left USB-C | UART bridge, CH343P, UART0 GPIO 16 TX / GPIO 17 RX. This is the `espflash` port. The monitor stays up through deep sleep |
| Right USB-C | Native USB Serial/JTAG (GPIO 12 D−, GPIO 13 D+). It drops when USB powers down for deep sleep. It is not USB CDC |
| Deep sleep | About 7 µA typical with the RTC domain off; about 15 µA with the RTC domain active |
| Battery | None. No onboard LiPo charger |
| Cargo | Chip `esp32c6`. Board `board-devkitc-c6` (default). RTC / LP GPIOs are 0–7 only |

How those facts sit in the template:

- **Module / SoC** records the WROOM-1, 8 MB flash, no PSRAM, and the PCB antenna.
- **Power Management** records the dual USB-C 5 V input, no battery connector, and the 7–15 µA floor.
- **Onboard Peripherals** records the WS2812 on GPIO 8 and the boot button on GPIO 0.
- **USB Serial / Console** classifies the board as a UART bridge for the default workflow, and names the right port as USB Serial/JTAG.
- **Cargo chip / board features** records `esp32c6` and `board-devkitc-c6`. It does not add another chip.

XIAO ESP32-C6 is the other v1 board (`board-xiao-c6`). Its user LED is GPIO 15, active LOW. Its boot button is GPIO 9, which is **not** an RTC GPIO. Switch boards with Cargo features, not `menuconfig`. Do not add a non-C6 board beside these two.

## After the board spec exists

1. Add the board to the example FunctionalSpec checklist. Pins still come from `board-specs/`, not from that checklist.
2. If a pin changed, regenerate the crate so `src/board.rs` updates. Say: Activate esp32-sdd-full-project-generator.
3. Say: Activate esp32-sdd-project-validator skill. Fix a failure in the spec and regenerate. Do not hand-edit the crate.
4. Say: Activate esp32-sdd-documentation-generator and rebuild all documentation. Supported Boards tables pick up `board-specs/` by discovery.

For a WS2812, state GRB, 800 kHz NRZ, and a brightness cap of 64/255. Brightness 8/255 is invisible in room light. For an active-LOW LED, state the polarity so `src/board.rs` inverts the drive.
