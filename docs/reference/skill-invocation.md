<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Skill Invocation

Canonical phrases. Every skill's first step is to read `shared-specs/AIGenLessonsLearned.md`.

| Skill | Phrase |
|---|---|
| `esp32-sdd-functional-spec-creator` | Activate esp32-sdd-functional-spec-creator skill |
| `esp32-sdd-full-project-generator` | Activate esp32-sdd-full-project-generator skill |
| `esp32-sdd-documentation-generator` | Activate esp32-sdd-documentation-generator and rebuild all documentation. |
| `esp32-board-spec-generator` | Activate esp32-board-spec-generator skill |
| `esp32-ws2812-led-engineer` | Activate esp32-ws2812-led-engineer skill |
| `esp32-ble-beacon-engineer` | Activate esp32-ble-beacon-engineer skill |
| `esp32-deep-sleep-engineer` | Activate esp32-deep-sleep-engineer skill |
| `esp32-hardware-crypto-configurer` | Activate esp32-hardware-crypto-configurer skill |
| `esp32-ansi-monitor-engineer` | Activate esp32-ansi-monitor-engineer skill |
| `esp32-sdd-project-validator` | Activate esp32-sdd-project-validator skill |
| `esp32-lp-core-engineer` | Activate esp32-lp-core-engineer skill |

## Typical new-example order

```
esp32-sdd-functional-spec-creator
  └─► esp32-sdd-full-project-generator
        ├─► domain skills (deep-sleep, ble-beacon, ws2812, ansi-monitor, hardware-crypto, lp-core)
        └─► cargo run --release
esp32-sdd-documentation-generator
esp32-sdd-project-validator
```
