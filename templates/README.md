<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# templates/

These files are starting points the agent reads when it generates a crate or a board spec. A **spec** is Markdown a person writes. `FunctionalSpec.template.md` shows the sections that file needs. You do not copy a template by hand. If you have not used specs as the source of truth, start at [Using an AI coding agent](../docs/guides/using-with-ai-agents.md).

## Available Templates

| Template | Used By | Purpose |
|---|---|---|
| `FunctionalSpec.template.md` | esp32-sdd-functional-spec-creator | Structure and required sections for a new example functional specification (C6 board checklist) |
| `BoardSpec.template.md` | esp32-board-spec-generator | Standard section layout for a new board specification |
| `main.rs.template` | esp32-sdd-full-project-generator | `#![no_std]` `#![no_main]` Embassy entry stub |
| `Cargo.toml.template` | esp32-sdd-full-project-generator | Non-radio crates.io pins + comment pointing at radio git pin; feature `esp32c6` |
| `build.rs.template` | esp32-sdd-full-project-generator | Linker config; emit `linkall.x` last |
| `board.rs.template` | esp32-sdd-full-project-generator | Mutually exclusive board-feature `compile_error!` |
| `gitignore.template` | esp32-sdd-full-project-generator | Must exclude `target/` and binaries |

`.cargo/config.toml.template` may also be present for the RISC-V target + `espflash` runner.

## Usage

Templates are read automatically by their associated skill — you do not need to copy or instantiate them manually. The skills handle template expansion and output to the correct destination.

If you want to see what sections a `FunctionalSpec.md` needs before running the creator skill, `FunctionalSpec.template.md` is the reference.

`esp32-sdd-full-project-generator` does **not** seed or overwrite `specs/FunctionalSpec.md`. Only `esp32-sdd-functional-spec-creator` writes that file.

## Changing a template

Templates are agent-generated. A structural change starts in `specs/example-project-structure-spec.md` or `specs/documentation-generation-spec.md`. Then ask the agent (not the shell):

> Activate esp32-sdd-documentation-generator and rebuild all documentation.

Do not hand-edit the template files.
