<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# SDD Workflows

**Spec-driven development (SDD)** means a **spec** is the source of truth for firmware and for documentation. A spec is Markdown a person writes. The agent reads it and writes the Rust and the Markdown. A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You say the activate phrase to the agent, not in the shell. **Generate** means the agent writes crate files or Markdown from the specs. **Regenerate** means generate again: previous agent output is overwritten and the spec files stay.

The longer lesson is [Using With AI Agents](../guides/using-with-ai-agents.md).

## Core principle

```
Human writes specs  →  AI reads specs  →  AI generates everything else
```

A spec is the source of truth for both firmware and docs, not only the slogan in that diagram. When output is wrong, change the spec and regenerate. Do not hand-edit `src/`, `Cargo.toml`, `build.rs`, `.cargo/config.toml`, READMEs, or `docs/`.

## Workflow 1 — New example

You add a catalog row in `specs/showcase-master-spec.md` and you write `CodingSpec.md` and `TestSpec.md`; the agent drafts `FunctionalSpec.md` and then writes the crate (`src/`, `Cargo.toml`, `build.rs`, `.cargo/config.toml`, `README.md`).

```
1. Edit specs/showcase-master-spec.md
2. Say to the agent: Activate esp32-sdd-functional-spec-creator
3. Review examples/<name>/specs/FunctionalSpec.md
4. Add examples/<name>/specs/CodingSpec.md
        (architecture and constraints — no code)
5. Add examples/<name>/specs/TestSpec.md
6. Say: Activate esp32-sdd-full-project-generator
7. From examples/<name>/: cargo run --release
8. Say: Activate esp32-sdd-documentation-generator and rebuild all documentation.
```

A domain skill runs only when the spec needs it (WS2812, BLE, deep sleep, hardware AES, or LP-core). ESP-NOW unicast, raw IEEE 802.15.4, Thread UDP pair, Matter over Wi-Fi, and an HTTPS CRC slot flip are steps of the full-project generator. Detail: [04 — Create a New Example](../guides/04-new-example.md).

## Workflow 2 — Board spec

A **board spec** is the Markdown description of one board's pins, LED, USB, and power; the agent turns those facts into `src/board.rs`.

v1 is ESP32-C6 boards only. A board spec does not add a chip feature.

```
1. Give the agent a vendor URL or a datasheet
2. Say: Activate esp32-board-spec-generator
3. Add the board to the example FunctionalSpec checklist
4. Regenerate docs so Supported Boards tables pick the file up
```

Walkthrough: [05 — Add a Board Spec](../guides/05-add-board-spec.md) and [Creating New Board Specs](../guides/creating-new-board-specs.md).

## Workflow 3 — Regenerate docs

You edit a file in `specs/` or `shared-specs/` (or an example spec, when the docs must follow it); the agent rewrites the generated Markdown and does not edit those spec files.

```
1. Edit the spec
2. Say: Activate esp32-sdd-documentation-generator and rebuild all documentation.
3. Commit the spec edit and the regenerated files together (see below)
```

Never generate anything inside `.github/workflows/`.

## Workflow 4 — Update an example

You edit `examples/<name>/specs/FunctionalSpec.md`, `CodingSpec.md`, or `TestSpec.md`; the agent overwrites the generated crate files in that example and leaves `specs/` in place.

```
1. Edit the example spec that matches the change
        (behavior, architecture, or checks)
2. Say: Activate esp32-sdd-full-project-generator
3. Review the diff, then from examples/<name>/: cargo run --release
```

If the docs must change too, run workflow 3 after the crate generation.

## Workflow 5 — Validate

Say `Activate esp32-sdd-project-validator`. The agent checks the locked crate layout and reports PASS or FAIL. `main.c`, `CMakeLists.txt`, or `sdkconfig` is a hard FAIL.

Fix a failure in the spec and regenerate. Do not hand-edit the crate. Run the validator again until the checks pass.

## Skill map

Each line is the purpose from `specs/showcase-master-spec.md`.

```
esp32-sdd-functional-spec-creator
  Drafts a new FunctionalSpec.md with you
    └─► esp32-sdd-full-project-generator
          Writes the no_std Cargo crate from the example specs
          │
          ├─► only when the spec needs it:
          │     esp32-ws2812-led-engineer
          │       Applies the WS2812 timing and driver constraints
          │     esp32-ble-beacon-engineer
          │       Designs BLE advertising and radio teardown before sleep
          │     esp32-deep-sleep-engineer
          │       Designs sleep, wakeup, and the power budget
          │     esp32-hardware-crypto-configurer
          │       Configures hardware AES and flash-backed key storage
          │     esp32-lp-core-engineer
          │       Writes the LP-core program and the HP-core handshake
          │
          ├─► steps of this skill, not separate skills:
          │     ESP-NOW unicast
          │     raw IEEE 802.15.4
          │     Thread UDP pair
          │     Matter over Wi-Fi
          │     HTTPS CRC slot-flip (not the AES skill)
          │
          └─► cargo run --release

esp32-sdd-documentation-generator
  Rewrites the Markdown docs from specs/

esp32-sdd-project-validator
  Checks an example against the locked crate layout

esp32-board-spec-generator
  Writes a board spec from a vendor page or datasheet
```

Phrases: [skill invocation](../reference/skill-invocation.md).

## Commit convention

Commits follow Conventional Commits: `<type>(<scope>): <short summary>`. Summary is imperative, lowercase, no trailing period, and at most 72 characters in total. Scope is optional (folder name, such as `blinky` or `shared-specs`).

| Type | When |
| --- | --- |
| `feat` | New example, new skill, or new spec section |
| `fix` | Correction to spec content, with the regenerated output in the same commit or the following `gen` commit |
| `spec` | Spec-only change, only when generation has not been run yet |
| `gen` | Regenerated files that follow a spec change already committed |
| `spec+gen` | Spec edit and the regenerated files in one commit |
| `chore` | Maintenance of files that are not agent-generated (for example `.gitignore`) |
| `docs` | **Do not use this type.** A documentation correction is a spec edit plus regeneration (`spec+gen`, or `spec` followed by `gen`). Generated Markdown is not hand-edited |

For `gen` and `spec+gen` commits, append a trailer. The name is the agent that produced the commit, not a fixed product string:

```
spec+gen: describe the spec workflow and regenerate the docs

Generated-by: <invoking agent name>
```

Do not commit generated files unless the spec change is in the same commit or an earlier one. From the repository root, `scripts/check-firmware-pins.sh` must pass. It checks every example `src/main.rs` for the application descriptor and the exact crates.io pins in `shared-specs/rust-toolchain.md`.

## Why this method

A spec is Markdown a person writes. The agent reads it and writes Rust and documentation. Specs are the source of truth for both the firmware and the docs.

- **Reproducibility.** Regeneration from the same spec produces the same kind of output. The spec, not a one-off patch, is what you keep.
- **Review.** A pull request contains the spec edit and the regenerated files. Reviewers read the behavior change in Markdown.
- **One hardware story.** Chip, board, sleep, and radio constraints live in specs, so the crate and the docs do not drift apart.
- **A smaller blast radius.** A wrong LED or a wrong current is a spec fix and a regenerate, not a hand-edit of `src/` that the next generate will wipe out.
