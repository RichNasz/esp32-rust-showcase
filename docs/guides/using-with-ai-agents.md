<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Using With AI Agents

This guide covers how to use the ESP32 Rust SDD Showcase skills with Claude Code, Grok, and other AI agents. Skills are **project-local**: they live in `skills/<name>/SKILL.md`.

## Institutional Memory — Read This First

> **Every AI agent working in this repository must read [`shared-specs/AIGenLessonsLearned.md`](../../shared-specs/AIGenLessonsLearned.md) before generating or modifying any file.**

This file is the gold standard institutional memory of the project. It contains:

- **Core rules** that must never be violated (no hand-editing generated files, always use board-specs for GPIO, no sdkconfig)
- **Key lessons** distilled from real AI generation sessions (retargeted Sep 2026 for rust-on-metal)
- **Debugging workflow** for when something goes wrong (improve the spec; never patch generated code)
- **A template** for adding new lessons as the project grows

All skills enforce reading this file as their mandatory first step.

---

## Setup

### 1. Clone the repository

```sh
git clone https://github.com/RichNasz/esp32-rust-showcase.git
cd esp32-rust-showcase
```

### 2. Open the project in your agent

Skills are Markdown workflows. Point the agent at `skills/` (Claude Code may also mirror them under `.claude/skills/`; Grok may use `AGENTS.md`).

### 3. Invoke a skill by name

> "Activate esp32-sdd-functional-spec-creator"

or

> "Activate esp32-sdd-full-project-generator skill"

The agent loads the skill's `SKILL.md` and follows it step by step.

## Available Skills

Eleven skills (10 original + `esp32-lp-core-engineer`):

| Skill | Purpose |
|---|---|
| `esp32-sdd-functional-spec-creator` | Guides creation of a new `FunctionalSpec.md` |
| `esp32-sdd-full-project-generator` | Generates a complete `no_std` `esp-hal` crate from specs |
| `esp32-board-spec-generator` | Produces board specs from vendor datasheets |
| `esp32-ws2812-led-engineer` | Generates WS2812B RMT driver via `esp-hal` |
| `esp32-ble-beacon-engineer` | Designs `esp-radio` + TrouBLE advertising and teardown before deep sleep |
| `esp32-deep-sleep-engineer` | Designs sleep configurations and power budgets |
| `esp32-hardware-crypto-configurer` | Configures hardware AES and `esp-storage` |
| `esp32-ansi-monitor-engineer` | Generates ANSI TUI serial dashboards via `esp-println` |
| `esp32-sdd-documentation-generator` | Regenerates all documentation from specs |
| `esp32-sdd-project-validator` | Validates any example against the locked crate structure |
| `esp32-lp-core-engineer` | LP-core firmware (`esp-lp-hal`) and HP load/handshake |

Full details: [skills/README.md](../../skills/README.md)

## Incremental Workflow

```
1. Edit a spec file in specs/ or examples/*/specs/
      │
      ▼
2. Activate the appropriate skill
      │
      ├─► esp32-sdd-functional-spec-creator   (new example)
      ├─► esp32-sdd-full-project-generator    (generate/update crate files)
      └─► esp32-sdd-documentation-generator  (after any spec change)
      │
      ▼
3. Review agent output — never edit generated files directly
      │
      ▼
4. Build and flash
      cargo run --release
```

Always modify specs, then regenerate. If something looks wrong in a generated file, fix the spec and rerun the skill.

Every generated example must follow the permanently locked directory layout defined in `specs/example-project-structure-spec.md` — human-authored files (`FunctionalSpec.md`, `CodingSpec.md`, `TestSpec.md`) live in `specs/`, all other files are agent-generated.

## Opening in VS Code / Cursor

Each example folder is a **complete, standalone Cargo package**.

**Recommended workflow (for development):**

1. **File → Open Folder…**
2. Open **only the specific example folder**, for example `examples/blinky/`
3. rust-analyzer uses that crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target + `espflash` runner)
4. Build/flash: `cargo run --release`

**Important notes:**

- Do **not** open the top-level repository folder as your main workspace when actively working on an example.
- You can still open the entire repository for browsing and editing specs.
- After editing `specs/FunctionalSpec.md` or `CodingSpec.md`, regenerate the example with the full-project-generator skill, then `cargo run --release`.

## Other AI Agents

Skills are plain Markdown. Any agent that can read a workflow file can follow the same instructions:

1. Read the relevant `SKILL.md`.
2. Read the spec files referenced by the workflow.
3. Generate output following the instructions in `SKILL.md`.

The skills do not depend on any vendor-specific API.

## Troubleshooting

**Skill not found**
: Confirm the agent is working from the repository root and can see `skills/<name>/SKILL.md`.

**Generated file looks wrong**
: Do not edit the generated file. Update the relevant spec and reactivate the skill.

**Unsure which skill to use**
: See [docs/workflows/README.md](../workflows/README.md).

**Want to verify a generated example is structurally correct**
: Say "Activate esp32-sdd-project-validator skill" — it reports PASS/FAIL per check. Presence of `main.c`, `CMakeLists.txt`, or `sdkconfig.defaults` is a hard FAIL.
