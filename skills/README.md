<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# skills/

Agent skills published for this repository. Each skill is a self-contained AI workflow definition. Together they implement the complete 100% AI-first SDD pipeline for rust-on-metal ESP32-C6.

Ten skills, including `esp32-lp-core-engineer`.

## Available Skills

| Skill | Description |
|---|---|
| [esp32-sdd-functional-spec-creator](esp32-sdd-functional-spec-creator/SKILL.md) | Interactively guides creation of a `FunctionalSpec.md` for a new ESP32-C6 example. |
| [esp32-sdd-full-project-generator](esp32-sdd-full-project-generator/SKILL.md) | Primary code-gen skill — produces a complete `no_std` `esp-hal` Cargo crate from spec files. |
| [esp32-sdd-documentation-generator](esp32-sdd-documentation-generator/SKILL.md) | Generates ALL Markdown documentation from `specs/`. Run after any spec change. Never writes `.github/workflows/`. |
| [esp32-board-spec-generator](esp32-board-spec-generator/SKILL.md) | Creates structured board-spec Markdown from vendor datasheets and product pages. |
| [esp32-ws2812-led-engineer](esp32-ws2812-led-engineer/SKILL.md) | Canonical WS2812B RMT driver via `esp-hal` (GRB, 800 kHz NRZ, brightness cap 64/255). DevKitC vs XIAO C6 via Cargo board features. |
| [esp32-ble-beacon-engineer](esp32-ble-beacon-engineer/SKILL.md) | BLE advertising via `esp-radio` + TrouBLE, manufacturer data PDUs, bounded windows, teardown before deep sleep. |
| [esp32-deep-sleep-engineer](esp32-deep-sleep-engineer/SKILL.md) | `esp-hal` RTC sleep, `#[esp_hal::ram(persistent)]` state, wakeup sources, power budgets, radio teardown. |
| [esp32-hardware-crypto-configurer](esp32-hardware-crypto-configurer/SKILL.md) | Hardware AES via `esp-hal` and key storage via `esp-storage`. No ESP-IDF mbedTLS. |
| [esp32-sdd-project-validator](esp32-sdd-project-validator/SKILL.md) | Validates any example against the locked no_std crate structure. Hard FAIL on `main.c` / `CMakeLists.txt` / `sdkconfig.defaults`. |
| [esp32-lp-core-engineer](esp32-lp-core-engineer/SKILL.md) | LP-core firmware (`esp-lp-hal`) plus HP `load_lp_code` handshake. GPIO 0–7 only (GPIO 9 is not RTC). |

## Using Skills

Skills are project-local — they live in this directory. Open the repository in your agent and invoke by name:

> "Activate esp32-sdd-full-project-generator skill"

Optional pack install:

```sh
npx @agentskills/cli install RichNasz/esp32-rust-showcase#skills
```

## Skill Execution Order (typical new example)

```
esp32-sdd-functional-spec-creator
  └─► esp32-sdd-full-project-generator
        ├─► esp32-deep-sleep-engineer        (if deep sleep required)
        ├─► esp32-ble-beacon-engineer        (if BLE advertising required)
        ├─► esp32-ws2812-led-engineer        (if WS2812 RGB LED required)
        ├─► esp32-hardware-crypto-configurer (if security required)
        └─► esp32-lp-core-engineer           (if LP-core firmware / wakeup required)
```

> All skills follow the 100% AI-first rule: they read specs and generate code or docs without human modification of output files.
