<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Agent instructions — ESP32 Rust SDD Showcase

Spec-Driven Development for **rust-on-metal ESP32-C6** (`no_std` [`esp-hal`](https://github.com/esp-rs/esp-hal)). There is **no ESP-IDF**, no `idf.py`, and no `sdkconfig`.

## Before any generate / edit / debug

1. Read [`shared-specs/AIGenLessonsLearned.md`](shared-specs/AIGenLessonsLearned.md).
2. Follow the matching skill in [`skills/<name>/SKILL.md`](skills/README.md).

## Role split

- Humans edit only: `specs/`, `shared-specs/`, `examples/*/specs/{Functional,Coding,Test}Spec.md`.
- Never hand-edit generated files (`src/`, `Cargo.toml`, `build.rs`, `.cargo/config.toml`, example READMEs, `/docs/`, root README, this file, …). If output is wrong, fix the spec and regenerate.

## Toolchain

- rustup **stable** (MSRV 1.95.0), component `rust-src`, target `riscv32imac-unknown-none-elf`, `espflash`.
- Not the `esp` rustup channel, not `espup`.
- Every firmware crate depends on `esp-bootloader-esp-idf` (feature `esp32c6`) and invokes `esp_bootloader_esp_idf::esp_app_desc!()`. Current `espflash` will not flash without that descriptor.
- From `examples/<name>/`: `cargo run --release`. Optional XIAO: `--no-default-features --features board-xiao-c6`.

Pins: [`shared-specs/rust-toolchain.md`](shared-specs/rust-toolchain.md).

## Chip vs boards

- **Chip (SoC)** feature `esp32c6` only. rust-on-metal is not `idf.py set-target`; do not copy S3/ESP32 specs onto C6.
- **Board** GPIO / LED / buttons come from [`board-specs/`](board-specs/) via generated `src/board.rs`. Never hardcode pins in `main.rs`.
- Default board feature `board-devkitc-c6` (ESP32-C6-DevKitC-1-N8). Optional `board-xiao-c6`. Other SoCs are catalog-only until a chip feature exists.

## Skills

Source of truth is [`skills/`](skills/README.md), **not** `.grok/skills/`. Invoke by name (for example `esp32-sdd-full-project-generator`). Index: [`skills/README.md`](skills/README.md).

## Layout

Examples are standalone Cargo packages under `examples/<name>/`. Locked structure: [`specs/example-project-structure-spec.md`](specs/example-project-structure-spec.md).
