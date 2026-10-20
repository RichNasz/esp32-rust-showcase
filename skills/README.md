<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# skills/

A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You ask an AI coding agent to follow it. It is not a shell command and not a Rust crate. The phrases are in [Skill invocation](../docs/reference/skill-invocation.md). If you have not used specs as the source of truth, start at [Using an AI coding agent](../docs/guides/using-with-ai-agents.md).

Ten skills. Purposes are copied from `specs/showcase-master-spec.md`.

## Available skills

| Skill | What the agent does when you ask it to follow this skill |
|---|---|
| [esp32-sdd-functional-spec-creator](esp32-sdd-functional-spec-creator/SKILL.md) | Drafts a new `FunctionalSpec.md` with you |
| [esp32-sdd-full-project-generator](esp32-sdd-full-project-generator/SKILL.md) | Writes the `no_std` Cargo crate from the example specs |
| [esp32-sdd-documentation-generator](esp32-sdd-documentation-generator/SKILL.md) | Rewrites the Markdown docs from `specs/` |
| [esp32-board-spec-generator](esp32-board-spec-generator/SKILL.md) | Writes a board spec from a vendor page or datasheet |
| [esp32-ws2812-led-engineer](esp32-ws2812-led-engineer/SKILL.md) | Applies the WS2812 timing and driver constraints |
| [esp32-ble-beacon-engineer](esp32-ble-beacon-engineer/SKILL.md) | Designs BLE advertising and radio teardown before sleep |
| [esp32-deep-sleep-engineer](esp32-deep-sleep-engineer/SKILL.md) | Designs sleep, wakeup, and the power budget |
| [esp32-hardware-crypto-configurer](esp32-hardware-crypto-configurer/SKILL.md) | Configures hardware AES and flash-backed key storage |
| [esp32-sdd-project-validator](esp32-sdd-project-validator/SKILL.md) | Checks an example against the locked crate layout |
| [esp32-lp-core-engineer](esp32-lp-core-engineer/SKILL.md) | Writes the LP-core program and the HP-core handshake |

## Using skills

Open this repository in the agent and say, for example:

> Activate esp32-sdd-full-project-generator skill

That line goes to the agent, not the shell. `skills/` is already in the clone.

Optional pack install:

```sh
npx @agentskills/cli install RichNasz/esp32-rust-showcase#skills
```

## Order for a new example

```
esp32-sdd-functional-spec-creator
  └─► esp32-sdd-full-project-generator
        ├─► esp32-deep-sleep-engineer        (if the spec sleeps)
        ├─► esp32-ble-beacon-engineer        (if the spec advertises BLE)
        ├─► esp32-ws2812-led-engineer        (if the spec uses a WS2812)
        ├─► esp32-hardware-crypto-configurer (if the spec needs AES or stored keys)
        └─► esp32-lp-core-engineer           (if the spec uses the LP core)
              └─► cargo run --release
esp32-sdd-documentation-generator
esp32-sdd-project-validator
```

The agent reads the specs and writes the crate and the docs. People do not hand-edit that output.
