<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Skill Invocation

A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You ask the agent to follow it. It is not a shell command and not a Rust crate.

The phrases below are typed to the agent, not run in a shell. Every skill's first step is to read `shared-specs/AIGenLessonsLearned.md`. **Generate** means the agent writes the files in the middle column. **Regenerate** means generate again, which overwrites the previous agent output and leaves spec files in place.

The middle column is the purpose text from `specs/showcase-master-spec.md`.

| Skill | What the agent writes | Phrase |
| --- | --- | --- |
| `esp32-sdd-functional-spec-creator` | Drafts a new `FunctionalSpec.md` with you | Activate esp32-sdd-functional-spec-creator skill |
| `esp32-sdd-full-project-generator` | Writes the `no_std` Cargo crate from the example specs | Activate esp32-sdd-full-project-generator skill |
| `esp32-sdd-documentation-generator` | Rewrites the Markdown docs from `specs/` | Activate esp32-sdd-documentation-generator and rebuild all documentation. |
| `esp32-board-spec-generator` | Writes a board spec from a vendor page or datasheet | Activate esp32-board-spec-generator skill |
| `esp32-ws2812-led-engineer` | Applies the WS2812 timing and driver constraints | Activate esp32-ws2812-led-engineer skill |
| `esp32-ble-beacon-engineer` | Designs BLE advertising and radio teardown before sleep | Activate esp32-ble-beacon-engineer skill |
| `esp32-deep-sleep-engineer` | Designs sleep, wakeup, and the power budget | Activate esp32-deep-sleep-engineer skill |
| `esp32-hardware-crypto-configurer` | Configures hardware AES and flash-backed key storage | Activate esp32-hardware-crypto-configurer skill |
| `esp32-sdd-project-validator` | Checks an example against the locked crate layout | Activate esp32-sdd-project-validator skill |
| `esp32-lp-core-engineer` | Writes the LP-core program and the HP-core handshake | Activate esp32-lp-core-engineer skill |

`skills/` is already in the clone. How specs work: [Using With AI Agents](../guides/using-with-ai-agents.md).

## New-example order

1. `esp32-sdd-functional-spec-creator` — draft the FunctionalSpec with you.
2. `esp32-sdd-full-project-generator` — write the crate. Leave `specs/` in place.
3. A domain skill only when the spec needs it (WS2812, BLE, deep sleep, hardware AES, or LP-core). Skip this step otherwise. ESP-NOW unicast, raw IEEE 802.15.4, and an HTTPS CRC slot flip are steps of `esp32-sdd-full-project-generator`, not separate skills. Hardware AES is not the skill for a CRC slot flip.
4. From `examples/<name>/`: `cargo run --release`.
5. `esp32-sdd-documentation-generator` — rewrite the Markdown.
6. `esp32-sdd-project-validator` — check the locked layout. Fix a failure in the spec and regenerate. Do not hand-edit the crate.
