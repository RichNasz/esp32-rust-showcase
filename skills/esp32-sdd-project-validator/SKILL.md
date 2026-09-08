---
name: esp32-sdd-project-validator
description: Validates any example folder against the permanent locked no_std Cargo crate structure in specs/example-project-structure-spec.md. Checks structure, spec file content, Cargo board-feature guards, agent-generated headers, and AIGenLessonsLearned.md compliance. Fails if main.c, CMakeLists.txt, or sdkconfig.defaults appear. Reports PASS/FAIL for every check.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, validator, structure, compliance, audit, rust, esp-hal, no_std, cargo
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 SDD Project Validator

## When to use
- Before committing or submitting a PR — verify a generated example meets all SDD rules.
- After running `esp32-sdd-full-project-generator` — confirm the output is correct.
- When debugging a build failure — rule out a structural or spec compliance issue first.
- Any time you want to audit an example folder on demand.

## Workflow (follow exactly)
MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else.

1. Read `specs/example-project-structure-spec.md` — load the permanent locked structure.
2. Read `shared-specs/AIGenLessonsLearned.md` — load the core rules.
3. List all files inside the target example folder (e.g. `examples/blinky/`).
4. Run every check below in order. For each check record: ✅ PASS, ❌ FAIL, or ⚠️ WARNING.

---

## Check Catalogue

### CHECK-01 — Required files present
Verify that every required path exists (relative to the example root):

| Path | Required? |
|---|---|
| `specs/FunctionalSpec.md` | Required |
| `specs/CodingSpec.md` | Required |
| `src/main.rs` | Required |
| `Cargo.toml` | Required |
| `build.rs` | Required |
| `.cargo/config.toml` | Required |
| `README.md` | Required |
| `.gitignore` | Required |
| `src/board.rs` | Required when the example supports more than one board, or whenever FunctionalSpec references board-specs for GPIO / LED / button. Treat as required in v1 (C6 DevKitC + XIAO). |
| `partitions.csv` / `espflash.toml` | Allowed (OTA / custom partition table) |
| `lp-core/` (nested crate) | Allowed when FunctionalSpec requires LP-core firmware |

**FAIL** if any Required file is absent.

### CHECK-02 — specs/ directory contains only human-authored files
`specs/` must contain `FunctionalSpec.md` and `CodingSpec.md`. `TestSpec.md` is also allowed (and encouraged). No other files, no subdirectories.

**FAIL** if any file other than `FunctionalSpec.md`, `CodingSpec.md`, or `TestSpec.md` is found inside `specs/`.

### CHECK-03 — No generated files have been placed inside specs/
`specs/FunctionalSpec.md` and `specs/CodingSpec.md` must NOT start with the agent-generated comment header (`<!-- AGENT-GENERATED` or `// AGENT-GENERATED`).

**FAIL** if either spec file starts with an agent-generated header.

### CHECK-04 — Agent-generated header present in generated Markdown files
Every generated `.md` file at the example root (`README.md`) must begin with:
```
<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
```
**FAIL** if `README.md` is missing this header.

### CHECK-05 — src/main.rs is no_std no_main with agent header
`src/main.rs` must:

1. Start with `// AGENT-GENERATED` on line 1.
2. Contain `#![no_std]`.
3. Contain `#![no_main]`.

**FAIL** if any of these three are absent.

### CHECK-06 — AIGenLessonsLearned.md referenced in CodingSpec.md
`specs/CodingSpec.md` line 1 must contain a reference to `shared-specs/AIGenLessonsLearned.md`.

**FAIL** if the reference is absent.

### CHECK-07 — .gitignore correctness
`.gitignore` must exclude `target/` and firmware binaries (`*.bin`, `*.elf`, `*.map`).

**FAIL** if `target/` is missing or binaries are not excluded.

### CHECK-08 — Cargo.toml has feature `esp32c6`
`Cargo.toml` must enable `esp-hal` feature `esp32c6` (for example `esp-hal = { version = "1.2.1", features = ["esp32c6"] }` or the git-pin equivalent). v1 has no other chip feature.

**FAIL** if `esp32c6` is absent from `Cargo.toml`.
**FAIL** if a chip feature other than `esp32c6` is declared (`esp32`, `esp32s3`, `esp32c3`, `esp32c5`, `esp32h2`, …).

### CHECK-09 — .cargo/config.toml target and runner
`.cargo/config.toml` must contain:

- `target = "riscv32imac-unknown-none-elf"`
- `runner = "espflash flash --monitor"` (additional `--partition-table` args are allowed)

**FAIL** if either value is absent or a different `--target` / runner (`probe-rs`, CMake) is selected.
**FAIL** if an Xtensa target is selected.

### CHECK-10 — Board module
`src/board.rs` (or equivalent board module) must exist and hold GPIO / LED / button constants derived from `board-specs/`. `src/main.rs` must not hardcode pin numbers.

**FAIL** if the board module is missing when FunctionalSpec references board-specs.
**FAIL** if `src/main.rs` contains hardcoded GPIO numbers that belong in `board.rs`.

### CHECK-11 — Multi-board Cargo feature guard
`src/board.rs` (or `src/main.rs`) must `compile_error!` if both `board-devkitc-c6` and `board-xiao-c6` are enabled, or if neither is enabled.

**FAIL** if the mutually exclusive board-feature guard is absent.

### CHECK-12 — Board-spec files exist
If any board is mentioned in `FunctionalSpec.md`, the corresponding `board-specs/<vendor>/<board>.md` file must exist.

**FAIL** if a referenced board-spec file is not found on disk.

### CHECK-13 — Forbidden ESP-IDF artifacts (hard FAIL)
**FAIL** if any of the following exist anywhere under the example folder:

- `main.c` (including `main/main.c` and `src/main.c`)
- `CMakeLists.txt` (root, `main/`, or any subdirectory except an unrelated nested crate that must still not use CMake)
- `sdkconfig.defaults` (including `sdkconfig.defaults.<target>`)
- `sdkconfig`, `sdkconfig.old`
- `Kconfig.projbuild`
- `idf_component.yml`

These files belong to ESP-IDF. This repository is rust-on-metal. Their presence is never a warning.

### CHECK-14 — No extra top-level files
Only these names are permitted at the example root:

`specs/`, `src/`, `Cargo.toml`, `build.rs`, `.cargo/`, `README.md`, `.gitignore`, `target/`

**Allowed extra** (not a FAIL): `Cargo.lock`, `partitions.csv`, `espflash.toml`, `lp-core/` (nested LP-core crate when FunctionalSpec requires LP-core wakeup / firmware).

**FAIL** for any forbidden IDF file listed in CHECK-13 even if it would otherwise be "extra".
**WARNING** (not FAIL) for any other unexpected file at the root — report the unexpected filename so the user can decide.

---

## Output Format

After all checks, output a summary table:

```
Validation Report — examples/<name>/
─────────────────────────────────────────────────────
CHECK-01  Required files present           ✅ PASS
CHECK-02  specs/ contains only human files ✅ PASS
CHECK-03  No generated files in specs/     ✅ PASS
CHECK-04  README.md agent header           ✅ PASS
CHECK-05  main.rs no_std/no_main + header  ✅ PASS
CHECK-06  AIGenLessonsLearned ref          ✅ PASS
CHECK-07  .gitignore completeness          ✅ PASS
CHECK-08  Cargo.toml esp32c6 feature       ✅ PASS
CHECK-09  .cargo/config.toml target/runner ✅ PASS
CHECK-10  Board module                     ✅ PASS
CHECK-11  Multi-board feature guard        ✅ PASS
CHECK-12  Board-spec files exist           ✅ PASS
CHECK-13  No ESP-IDF artifacts             ✅ PASS
CHECK-14  No extra top-level files         ✅ PASS
─────────────────────────────────────────────────────
Result: 14/14 PASS  |  0 FAIL  |  0 WARNING
```

Then output one of:
- `"✅ <name> passed all SDD structure checks."` — if zero FAILs.
- `"❌ <name> failed <N> check(s). Fix the spec(s) and regenerate."` — if any FAILs.

Do NOT suggest editing generated files. All fixes go through the spec → regenerate cycle.
