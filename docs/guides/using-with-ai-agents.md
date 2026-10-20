<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Using With AI Agents

You would use an AI coding agent for ESP32 Rust when you want `no_std` firmware — Rust without the standard library, running on the chip through `esp-hal` — and you do not want to hand-maintain a second copy of the same facts in Rust and in Markdown. Pins, deep-sleep wakeup, and radio teardown are easy for an agent to invent. A written spec gives the agent one place to read, and gives you one place to review, before any crate exists.

`no_std` on ESP32-C6 in this repository does not use ESP-IDF. If you are arriving from C, read [Why ESP-IDF is not required](why-esp-idf-is-not-required.md) once. You do not need to know spec-driven development before this page.

## 1. Specs are the source of truth

**Spec-driven development (SDD)** means specs are the source of truth for both firmware and documentation. A **spec** is a Markdown file a person writes. It states what the firmware must do and what the docs must say. An AI coding agent reads those files and writes the Rust and the Markdown. People do not hand-edit `src/`, `Cargo.toml`, `build.rs`, `.cargo/config.toml`, READMEs, or `docs/`. When generated output is wrong, change the spec and ask the agent to generate again.

## 2. How to set up specs

Human files live in three places:

- `specs/` is repo-wide rules (the example catalog, how docs are generated, how to contribute).
- `shared-specs/` is cross-example platform constraints (toolchain, Cargo, coding, power, security).
- `examples/<name>/specs/FunctionalSpec.md` is observable behavior. `CodingSpec.md` is architecture and constraints with no code. `TestSpec.md` is how to check the example.

A new example starts by adding a row to `specs/showcase-master-spec.md` and creating those three example spec files. The section list is [`templates/FunctionalSpec.template.md`](../../templates/FunctionalSpec.template.md). The directory lock is [`specs/example-project-structure-spec.md`](../../specs/example-project-structure-spec.md).

## 3. How to modify specs

Edit the human files in your editor. Behavior changes go in `FunctionalSpec.md`. Architecture, crates, and gotchas go in `CodingSpec.md`. Checks go in `TestSpec.md`. Repo-wide documentation rules go in `specs/`. Do not hand-edit generated Rust or generated Markdown to change behavior or docs.

## 4. How to generate

**Generate** means the agent writes crate files or Markdown from the specs. **Regenerate** means generate again. Generation overwrites the previous agent output and leaves the spec files in place. A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You type these phrases to the agent, not in the shell:

- Activate esp32-sdd-functional-spec-creator — drafts a FunctionalSpec
- Activate esp32-sdd-full-project-generator — writes the crate (`src/`, `Cargo.toml`, `build.rs`, `.cargo/config.toml`) and leaves `specs/` in place
- Activate esp32-sdd-documentation-generator and rebuild all documentation. — rewrites Markdown after a spec change (root README, `CONTRIBUTING.md`, `docs/`, example READMEs, and the rest of the generated-docs list)

The numbered path for a new crate is [04 — Create a New Example](04-new-example.md).

## Agent setup

`skills/` is already in the clone. A separate install is not required.

```sh
git clone https://github.com/RichNasz/esp32-rust-showcase.git
cd esp32-rust-showcase
```

Open that folder in the agent.

- **Claude Code.** The source of truth is `skills/`. A mirror under `.claude/skills/` is optional.
- **Grok.** Root `AGENTS.md` loads at session start and points at `skills/` and at `shared-specs/AIGenLessonsLearned.md`.
- **Any agent that can read Markdown.** Ask it to read `skills/<name>/SKILL.md` and follow it. Skills do not depend on one vendor's API.

Every skill starts by reading [`shared-specs/AIGenLessonsLearned.md`](../../shared-specs/AIGenLessonsLearned.md). Optional only: `npx @agentskills/cli install RichNasz/esp32-rust-showcase#skills`. See [02 — Install Skills](02-install-skills.md).

## Skills

Purposes are copied from `specs/showcase-master-spec.md`.

| Skill | Purpose |
| --- | --- |
| `esp32-sdd-functional-spec-creator` | Drafts a new `FunctionalSpec.md` with you |
| `esp32-sdd-full-project-generator` | Writes the `no_std` Cargo crate from the example specs |
| `esp32-sdd-documentation-generator` | Rewrites the Markdown docs from `specs/` |
| `esp32-board-spec-generator` | Writes a board spec from a vendor page or datasheet |
| `esp32-ws2812-led-engineer` | Applies the WS2812 timing and driver constraints |
| `esp32-ble-beacon-engineer` | Designs BLE advertising and radio teardown before sleep |
| `esp32-deep-sleep-engineer` | Designs sleep, wakeup, and the power budget |
| `esp32-hardware-crypto-configurer` | Configures hardware AES and flash-backed key storage |
| `esp32-sdd-project-validator` | Checks an example against the locked crate layout |
| `esp32-lp-core-engineer` | Writes the LP-core program and the HP-core handshake |

Phrases: [skill invocation](../reference/skill-invocation.md).

## Incremental workflow

```
1. Edit a spec in specs/, shared-specs/, or examples/<name>/specs/
      │
      ▼
2. Ask the agent to generate (phrase to the agent, not the shell)
      │
      ├─► Activate esp32-sdd-functional-spec-creator
      ├─► Activate esp32-sdd-full-project-generator
      └─► Activate esp32-sdd-documentation-generator and rebuild all documentation.
      │
      ▼
3. Review the agent output. Do not edit generated files
      │
      ▼
4. From examples/<name>/: cargo run --release
```

A domain skill (WS2812, BLE, deep sleep, hardware AES, LP-core) is only for a spec that needs that topic. It is not a step on every example.

## Open only the example folder

Each example is a standalone Cargo package. There is no root Cargo workspace.

For day-to-day build and rust-analyzer: **File → Open Folder… → select `examples/<name>/`**, not the repository root. That crate's `.cargo/config.toml` sets the RISC-V target and the `espflash` runner. Build and flash with `cargo run --release`.

You can still open the repository root to edit specs. After a spec edit, regenerate, then `cargo run --release` from the example folder.

## Troubleshooting

**The agent cannot find a skill.** Confirm it is working from the repository root and can read `skills/<name>/SKILL.md`.

**Generated output is wrong.** Do not edit `src/`, `Cargo.toml`, or a generated README. Edit the spec and regenerate.

**You are not sure which skill to use.** See [workflows](../workflows/README.md).

**You want a structural check.** Say `Activate esp32-sdd-project-validator skill`. It reports PASS or FAIL per check. `main.c`, `CMakeLists.txt`, or `sdkconfig.defaults` is a hard FAIL. Fix a failure in the spec and regenerate. Do not hand-edit the crate.
