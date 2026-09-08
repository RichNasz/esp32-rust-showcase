---
name: esp32-sdd-functional-spec-creator
description: Interactively guides creation of a FunctionalSpec.md for a new ESP32-C6 rust-on-metal example. Captures feature goals, hardware dependencies, wakeup strategy, connectivity model, and success criteria (cargo run --release) in the canonical SDD format consumed by esp32-sdd-full-project-generator.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, functional-spec, requirements, planning, specification, rust, esp-hal, esp32-c6
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 SDD Functional Spec Creator

## When to use
When starting a new example or feature. Always run this skill before `esp32-sdd-full-project-generator` — the full-project generator requires a completed `FunctionalSpec.md` as its primary input.

## v1 board catalog

| Role | Board | Board-spec | Cargo feature |
|---|---|---|---|
| **Default** | Espressif ESP32-C6-DevKitC-1-N8 | `board-specs/espressif/esp32-c6-devkitc-1-n8.md` | `board-devkitc-c6` |
| Secondary | Seeed XIAO ESP32-C6 | `board-specs/seeed/xiao-esp32c6.md` | `board-xiao-c6` |

Chip (SoC) ≠ board. Chip is ESP32-C6 only (`esp32c6`). Other SoCs (ESP32, S3, C5, …) are not in v1 — do not offer them. Do not copy S3 touch / classic ULP / Xtensa behaviour onto a C6 spec. rust-on-metal is not `idf.py set-target`.

RTC GPIOs on C6 are **0–7 only**. GPIO 9 (XIAO boot button) is **not** an RTC GPIO and cannot wake from deep sleep.

## Workflow (follow exactly)
MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else.

1. Ask the user for: example name (kebab-case slug), target **board** (default DevKitC-C6; offer XIAO C6 as secondary — chip is always C6), and high-level goal in one sentence. Name SoC `esp32c6` in Target Hardware separately from the board checklist.
2. Derive hardware requirements from the goal (sensors, radios, power mode, security needs, connectivity, LP-core).
3. Ask clarifying questions only if a requirement is genuinely ambiguous (wakeup source, MQTT topic, etc.).
4. Draft all `FunctionalSpec.md` sections using `templates/FunctionalSpec.template.md`:
   - Overview
   - Target Hardware: **SoC** table (`esp32c6`) then Board Compatibility Checklist (C6 DevKitC default, XIAO C6 secondary). Do not describe another SoC’s peripherals.
   - Features
   - Wakeup Strategy (if applicable)
   - Connectivity
   - Security Requirements
   - Success Criteria — **must** include `cargo run --release` as the primary on-device success command (plus `cargo build --release` / `cargo clippy -- -D warnings` as appropriate)
5. Present the complete draft for user review; apply any corrections verbatim.
6. Write the approved spec to `examples/<name>/specs/FunctionalSpec.md`. This file is **human-authored**: do **not** add an AGENT-GENERATED header. Create `examples/<name>/specs/` if needed.
7. Prompt: "Spec complete. Add a `CodingSpec.md` (and `TestSpec.md`) under `examples/<name>/specs/` if needed, then activate `esp32-sdd-full-project-generator` to generate the crate."
