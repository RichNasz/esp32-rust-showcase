<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Guides

Step-by-step guides for getting started with the ESP32 Rust SDD Showcase, from rustup + espflash through creating your own rust-on-metal SDD examples.

## Available Guides

| Guide | Description |
|---|---|
| [Why ESP-IDF is not required](why-esp-idf-is-not-required.md) | Why C developers do not need ESP-IDF / `idf.py` for this rust-on-metal repo |
| [01 — Toolchain Setup](01-toolchain-setup.md) | Install rustup stable, `rust-src`, the RISC-V target, and espflash |
| [02 — Install Skills](02-install-skills.md) | Use the project-local `skills/` tree with Claude Code, Grok, and other agents |
| [03 — Run Your First Example](03-first-example.md) | Build and flash `blinky` with `cargo run --release` |
| [04 — Create a New Example](04-new-example.md) | Use `esp32-sdd-functional-spec-creator` to spec and generate a new crate |
| [05 — Add a Board Spec](05-add-board-spec.md) | Use `esp32-board-spec-generator` to add your board to the catalog |
| [06 — Deep Sleep Design](06-deep-sleep-design.md) | Use `esp32-deep-sleep-engineer` to hit your power budget |
| [07 — Secure Your Project](07-secure-your-project.md) | Use `esp32-hardware-crypto-configurer` for hardware AES and flash-backed keys |
| [Using With AI Agents](using-with-ai-agents.md) | Project-local skill setup for Claude Code, Grok, and other AI agents |
| [Creating New Board Specs](creating-new-board-specs.md) | How to use `esp32-board-spec-generator` — includes a C6 DevKitC walkthrough |

## Recommended Reading Order

If you are new to the project, follow the guides in order:

```
Why ESP-IDF is not required   (C developers: read this first)
  └─► 01 Toolchain Setup
        └─► 02 Install Skills
              └─► 03 Run Your First Example
                    └─► 04 Create a New Example
```

Then dive into topic guides (05–07) as needed for your project.

## Prerequisites

- macOS, Linux, or Windows
- Git
- rustup **stable** (MSRV 1.95.0)
- An AI agent that can read `skills/<name>/SKILL.md`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- A USB-C cable (DevKitC: **left** UART-C port for `espflash` monitor through deep sleep)

ESP-IDF is **not** required — [why](why-esp-idf-is-not-required.md).
