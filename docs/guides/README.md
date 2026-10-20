<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Guides

These guides are for someone who wants an AI coding agent to help with ESP32 Rust, and for someone who only wants to flash a crate that is already in the tree.

**Spec-driven development (SDD)** means a **spec** — Markdown a person writes — is the source of truth. The agent reads it and writes the Rust and the docs. A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You ask the agent to follow it. It is not a shell command. **Generate** means the agent writes crate files or Markdown from the specs. **Regenerate** means generate again, which overwrites the previous agent output and leaves the spec files in place.

You do not need those words to flash an example. You do need them before you change firmware or docs.

## Available guides

| Guide | What you are trying to do |
| --- | --- |
| [Why ESP-IDF is not required](why-esp-idf-is-not-required.md) | Why C developers do not need ESP-IDF / `idf.py` for this rust-on-metal repo |
| [01 — Toolchain Setup](01-toolchain-setup.md) | Install rustup stable and espflash so you can build a crate |
| [02 — Install Skills](02-install-skills.md) | Point an agent at the `skills/` tree that is already in the clone |
| [03 — Run Your First Example](03-first-example.md) | Flash `blinky` with Cargo. No agent |
| [Using With AI Agents](using-with-ai-agents.md) | Learn how specs drive firmware and docs before you change either |
| [04 — Create a New Example](04-new-example.md) | Add a catalog row and three spec files, then ask the agent to generate the crate |
| [05 — Add a Board Spec](05-add-board-spec.md) | Describe one ESP32-C6 board's pins so the agent can write `src/board.rs` |
| [Creating New Board Specs](creating-new-board-specs.md) | Walk the DevKitC-1-N8 board spec section by section |
| [06 — Deep Sleep Design](06-deep-sleep-design.md) | Edit the FunctionalSpec, then ask the agent to design sleep and regenerate the crate |
| [07 — Secure Your Project](07-secure-your-project.md) | Ask the agent for hardware AES and flash-backed keys, not Secure Boot |

## Recommended reading order

If you write C or C++ for ESP32, read [Why ESP-IDF is not required](why-esp-idf-is-not-required.md) before either task.

**Task A — flash an existing example.** No agent.

```
01 Toolchain Setup
  └─► 03 Run Your First Example
```

**Task B — change firmware or docs with an AI coding agent.** If you have not used specs as the source of truth, read [Using With AI Agents](using-with-ai-agents.md) before guide 04.

```
Using With AI Agents
  └─► 04 Create a New Example
```

Guide 02 is optional for task B: `skills/` is already in the clone. Guides 05–07 are topic pages after you can generate a crate.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md))
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` (optional: `cargo-espflash`)
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6

ESP-IDF is not required. Neither are `espup` and the `esp` rustup channel. [Why ESP-IDF is not required](why-esp-idf-is-not-required.md).

Task B also needs an agent that can read Markdown. Task A does not.
