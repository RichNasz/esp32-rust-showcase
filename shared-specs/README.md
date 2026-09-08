<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# shared-specs/

Specifications that apply across multiple examples or the entire project. These files capture platform-level constraints, coding standards, and reusable hardware patterns. Individual example `FunctionalSpec.md` files reference these by name; the **esp32-sdd-full-project-generator** skill reads them automatically during crate generation.

## Essential Reading

> **Every AI agent working in this repository must read `AIGenLessonsLearned.md` before generating or modifying any file.** It is the gold standard institutional memory and quality guardrail for the entire project.

| File | Purpose |
|---|---|
| **[`AIGenLessonsLearned.md`](AIGenLessonsLearned.md)** | **MANDATORY FIRST READ — Gold standard institutional memory, core rules, and lessons from real AI generation** |
| `rust-toolchain.md` | rustup stable, MSRV 1.95.0, `esp-hal` 1.2.1 pins, radio git-pin at tag `esp-hal-v1.2.1` |
| `cargo-conventions.md` | Standalone crates, board Cargo features, `build.rs`, `.cargo/config.toml` |
| `coding-conventions.md` | `no_std` Rust style, naming, error handling, Embassy notes |
| `power-budget-template.md` | Standard power budget calculation template for all low-power examples |
| `security-baseline.md` | Minimum security requirements and acceptable configurations |
| `partition-table-profiles.md` | Reusable partition table CSV profiles for **espflash** (not sdkconfig) |
| `ble-patterns.md` | BLE advertising / manufacturer-data patterns for `esp-radio` + TrouBLE |

## Usage

Reference a shared spec in any `FunctionalSpec.md` like this:

```markdown
## Shared Specs
- shared-specs/AIGenLessonsLearned.md
- shared-specs/rust-toolchain.md
- shared-specs/security-baseline.md
```

The full-project-generator skill will pull in the referenced shared specs automatically.

## Editing

Shared spec files are **human-authored** — they live here as a primary source of truth alongside example-level specs. Edit them directly when platform-level constraints change, then regenerate all documentation.
