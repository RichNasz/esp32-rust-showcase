<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Creating New Board Specs

This guide explains how to add a new ESP32 board to the `board-specs/` catalog using the **esp32-board-spec-generator** skill. The Espressif ESP32-C6-DevKitC-1-N8 is the v1 default board.

---

## Overview

A board spec is an authoritative, structured Markdown file that describes the hardware layout of one specific development board or module. It is the single source of truth for GPIO numbers, power limits, strapping pin constraints, and peripheral assignments. All example projects reference board specs instead of hard-coding pin numbers. Generated `src/board.rs` is the only place pin numbers belong.

Location: `board-specs/<vendor-slug>/<board-slug>.md`

v1 **firmware** compiles only for ESP32-C6 (`esp32c6`). Chip ≠ board: a board-spec does not add a chip feature. Add only ESP32-C6 boards. Do not add other SoCs.

---

## When to Use esp32-board-spec-generator

| Situation | Action |
|---|---|
| Adding a new board to a `FunctionalSpec.md` that has no existing board spec | Generate the spec first, then write the FunctionalSpec |
| Existing board spec is missing sections (power, strapping, ADC warnings) | Regenerate — never hand-edit |
| Vendor released a hardware revision with changed pin assignments | Regenerate with updated source URL |
| A board spec was created as a placeholder with unverified data | Regenerate with verified authoritative source |

---

## What to Provide to the Agent

When invoking the skill, include all of the following in your prompt:

### 1. Authoritative source URL

Prefer the **vendor's official wiki** over GitHub READMEs or third-party guides:

- Espressif: `docs.espressif.com` + devkit product pages
- Seeed Studio: `wiki.seeedstudio.com`

### 2. Key facts to verify before running

Cross-check these against the vendor schematic or wiki before handing them to the agent — they are the highest-risk items for hallucination:

| Fact | How to verify |
|---|---|
| Onboard LED GPIO number, type (simple GPIO vs WS2812), and polarity | Vendor wiki "Pinout" or schematic net list |
| Strapping pin numbers | ESP32-C6 Technical Reference Manual + board schematic |
| PSRAM type (none / SPI / OPI) and size | Module datasheet |
| Deep sleep current | Vendor datasheet power table |
| USB console | UART bridge chip vs native USB Serial/JTAG vs USB CDC — **do not** emit `CONFIG_ESP_CONSOLE_*` (there is no sdkconfig) |
| RTC / LP GPIOs | C6: GPIO **0–7 only**. GPIO 9 is **not** RTC-capable. |

### 3. Board name slug

Use lowercase-kebab: `esp32-c6-devkitc-1-n8`, `xiao-esp32c6`.

### Example invocation

```
Activate esp32-board-spec-generator.

Source: https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32c6-devkitc-1/index.html
Board: Espressif ESP32-C6-DevKitC-1-N8

Pre-verified facts:
- Onboard LED: WS2812 on GPIO 8
- Boot button: GPIO 0 (RTC-capable, active LOW)
- Strapping pins: GPIO 0, 4, 5, 8, 9, 15
- PSRAM: none
- Deep sleep: ~7–15 µA
- USB: left USB-C = UART bridge (CH343P → UART0); right USB-C = USB Serial/JTAG
```

---

## Board Spec Template (Required Sections)

Every board spec must contain these sections in this order (see `templates/BoardSpec.template.md`):

| Section | Purpose |
|---|---|
| Agent-generated header | Marks file as generated; includes source URL and date |
| **Module / SoC** | Chip variant, flash, PSRAM, antenna type, form factor |
| **Power Management** | 3V3 regulator limit, USB input, battery charger, deep sleep µA |
| **Onboard Peripherals** | GPIO-mapped peripherals (LEDs, buttons, sensors) with polarity |
| **GPIO & Pin Rules / Warnings** | Strapping pins, ADC restrictions, reserved internal pins, USB console |
| **Complete Pin Mapping Table** | All user-accessible pads with GPIO, ADC channel, and default peripheral |
| **Board Advantages for SDD Projects** | Why this board suits which types of SDD examples |
| **Chip / board Cargo features** | `esp32c6` plus `board-devkitc-c6` or `board-xiao-c6` (no `idf.py set-target`) |

USB console wording:

- UART bridge chip → "UART bridge (`espflash` on the UART-C port)"
- Native USB Serial/JTAG → "USB Serial/JTAG"
- Native USB CDC → "USB CDC"

---

## Best Practices

**Verify the LED GPIO with a blink crate first.** Board wikis sometimes describe an older PCB revision.

**Prefer the vendor wiki over community GitHub repos.** GitHub READMEs are often outdated.

**List strapping pins even if they seem obvious.** Strapping pins are the #1 cause of "my board won't boot" issues.

**For WS2812, note that a GPIO level produces no visible output.** Use RMT (or the equivalent `esp-hal` driver) for the 800 kHz NRZ waveform. Brightness cap 64/255.

**For active-LOW LEDs, note polarity explicitly** so generated `src/board.rs` inverts drive logic.

**Never omit a section because the board "probably" doesn't have it.** If no PSRAM: write `None`. If no LiPo charger: write `None`.

---

## Live Example: Espressif ESP32-C6-DevKitC-1-N8

### Step 1 — Identify the authoritative source

Source used: `https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32c6-devkitc-1/index.html`

### Step 2 — Extract and verify key facts

| Fact | Value | Verification |
|---|---|---|
| LED | WS2812 GPIO 8 | Dev-kit user guide + schematic |
| Boot button | GPIO 0, active LOW, RTC-capable | C6 TRM RTC GPIO 0–7; schematic |
| UART | GPIO 16 TX / GPIO 17 RX via CH343P on **left** USB-C | User guide dual USB-C section |
| Deep sleep | ~7 µA typical / ~15 µA RTC domain active | Datasheet power table |
| Flash | 8 MB (N8) | Module suffix |

### Step 3 — Downstream impact

When a board spec is corrected or updated:

1. Check all `FunctionalSpec.md` files that reference this board spec.
2. If GPIO assignments changed, regenerate `src/board.rs` via `esp32-sdd-full-project-generator`.
3. Run `esp32-sdd-project-validator` on the regenerated example.
4. Run `esp32-sdd-documentation-generator` so Supported Boards tables stay in sync.

> **blinky reference**: DevKitC-C6 LED is WS2812 GPIO 8. XIAO ESP32-C6 LED is GPIO 15, active LOW, simple GPIO. Switch boards with Cargo features, not menuconfig.
