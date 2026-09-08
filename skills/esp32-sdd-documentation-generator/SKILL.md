---
name: esp32-sdd-documentation-generator
description: Generates ALL Markdown files in the repo for rust-on-metal ESP32-C6 SDD. Root README badges are License / Rust / esp-hal / ESP32-C6. Prerequisites are rustup + espflash, not ESP-IDF. Keeps root minimal and places extended docs in /docs/. Enforces 100% AI-first rule. Never generates .github/workflows/.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, documentation, agent-generated, docs-folder, rust, esp-hal, esp32-c6
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 SDD Showcase Documentation Generator

## When to use
Any time specs/ changes or you add examples/boards.

## Workflow (follow exactly)
MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else.

1. Read all files in `specs/` (including `example-project-structure-spec.md` and `documentation-generation-spec.md`). Follow `specs/documentation-generation-spec.md` as the single source of truth for Markdown.
2. Generate the following files per documentation-generation-spec.md:
   - README.md (full OSS-standard structure — see Root README.md Required Sections in the spec)
   - CONTRIBUTING.md
   - CODE_OF_CONDUCT.md (source content from contributing-spec.md)
   - SECURITY.md (source content from contributing-spec.md)
   - AGENTS.md (short new-session context — see Root AGENTS.md in the spec; do not clone the README)
   - .github/ISSUE_TEMPLATE/spec-improvement.md
   - .github/ISSUE_TEMPLATE/bug-report.md
   - .github/PULL_REQUEST_TEMPLATE.md
   NEVER generate anything inside `.github/workflows/` — this directory is off-limits.
3. Generate full docs/ folder + all example READMEs + board-specs/README.md + skills/README.md + etc.
   - NEVER generate or place files inside `.github/workflows/` — this directory is off-limits.
4. Root README **badge row** must be exactly:

   ```markdown
   [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
   ![Rust](https://img.shields.io/badge/Rust-stable-orange)
   ![esp-hal](https://img.shields.io/badge/esp--hal-1.2-blue)
   ![Chip](https://img.shields.io/badge/chip-ESP32--C6-orange)
   ```

   Do not add an ESP-IDF badge. Do not add a CI/build-status badge.

5. Root README **Prerequisites** must list:
   - rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
   - rustup component `rust-src`
   - rustup target `riscv32imac-unknown-none-elf`
   - `espflash` and/or `cargo-espflash`
   - Hardware: ESP32-C6-DevKitC-1-N8 (required default) or XIAO ESP32-C6

   Must **not** list ESP-IDF, `espup`, or the `esp` rustup channel as prerequisites.

6. Root README **Quick Start** from `examples/<name>`:

   ```sh
   cargo run --release
   ```

   Optional XIAO board:

   ```sh
   cargo run --release --no-default-features --features board-xiao-c6
   ```

7. For each example README.md, check whether `examples/<name>/specs/TestSpec.md` exists.
   - If it exists: generate a **Testing** section after “Key Concepts” (and after Limitations if present) and before “Spec Files”:
     - Open with one sentence: “Testing philosophy: automated first, manual only when hardware observation is unavoidable.”
     - **Automated Tests** subsection: numbered steps, native Cargo / rustc tools first (`cargo build --release`, `cargo clippy -- -D warnings`, `cargo run --release` / Wokwi). Include additional tools only if TestSpec.md documents them with justification.
     - **Manual Tests** subsection (label: “Manual — hardware required”): numbered steps, only for checks that cannot be automated. Each step must note why automation is not practical.
     - Never mix automated and manual steps in the same list.
     - Steps must be beginner-friendly (assume rustup + espflash installed, not an embedded-Rust expert).
   - If TestSpec.md does not exist: omit the Testing section entirely. Do not generate a placeholder.
8. Example READMEs **Build & Flash** must document:

   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   cargo run --release
   ```

   and the XIAO feature variant. Do not present any other primary flash tool.

9. **Opening in VS Code / Cursor** (every example README): this folder is a complete standalone Cargo package. Open **only this folder** (not the repository root) so rust-analyzer uses this crate. **File → Open Folder… → select `examples/<name>/`**. Build/flash: `cargo run --release`.

10. Supported Boards tables are auto-discovered from `board-specs/` filtered to ESP32-C6 (`documentation-generation-spec.md`). Do not emit `CONFIG_ESP_CONSOLE_*` sdkconfig lines — there is no sdkconfig. USB console is UART bridge (`espflash` on the UART-C port), USB Serial/JTAG, or USB CDC.

11. Every example that enables `esp-hal` feature `unstable` must state in its README that those APIs are unstable. Radio example READMEs must say `esp-radio` is an unstable beta.

12. Every generated `.md` file must carry the standard agent-generated header. Generated directory-layout descriptions must match `specs/example-project-structure-spec.md` exactly. Purpose of this repository: Spec-Driven Development for **rust-on-metal ESP32** (`no_std` `esp-hal` on ESP32-C6).

13. End with “All documentation is now in sync.”
