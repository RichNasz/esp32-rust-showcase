# Repo Overview Specification

## Purpose
Comprehensive showcase of Spec-Driven Development (SDD) for rust-on-metal ESP32 (`no_std` `esp-hal` on ESP32-C6).
Every line of firmware and every piece of documentation is generated exclusively by AI agents from human-written specifications.

## Key Features
- AI-generated board specs from vendor docs (Espressif, Seeed, Adafruit, LilyGo, etc.)
- 9 progressive examples on ESP32-C6 leveraging hardware superpowers (deep sleep, LP core, radio, AES, OTA, raw IEEE 802.15.4)
- Full suite of published Agent Skills (`skills/<name>/SKILL.md`; mirrored for Claude; Grok via `AGENTS.md`)
- Strict 100 % AI-first workflow: humans edit specs only; agents generate Rust and Markdown
- No ESP-IDF / `idf.py` / `sdkconfig` — rust-on-metal `esp-hal` on RISC-V ESP32-C6 (see Why ESP-IDF is not required)
- Chip (SoC) ≠ board: v1 chip feature `esp32c6` only; boards are `board-devkitc-c6` / `board-xiao-c6`. rust-on-metal is not IDF `set-target` portability

## Why ESP-IDF is not required
C developers arriving from ESP-IDF should be told *why* they skip that toolkit, not only that it is unused. Generated docs (root README Prerequisites + `docs/guides/why-esp-idf-is-not-required.md`) must follow `specs/documentation-generation-spec.md` section “Guide: Why ESP-IDF is not required”.

This repository uses Espressif’s official `no_std` `esp-hal` path. It does **not** use the community `esp-idf-*` crates that wrap the C SDK. ESP32-C6 is RISC-V, so rustup **stable** plus target `riscv32imac-unknown-none-elf` is sufficient; `espup` is for Xtensa. Build/flash is Cargo + `espflash`.

## Hardware bar
ESP32-C6-DevKitC-1-N8 required; XIAO ESP32-C6 also supported. Other SoCs (ESP32, S3, C5, …) are catalog-only until a chip feature exists. Chip feature `esp32c6` only.

## Quick Start
1. git clone this repo
2. Install Agent Skills from this repository's `skills/` tree
3. rustup stable + `rust-src` + target `riscv32imac-unknown-none-elf`; `cargo install espflash`
4. `cd examples/blinky && cargo run --release`
5. Edit any `FunctionalSpec.md` → activate the full-project-generator skill

Full documentation → [docs/README.md](docs/README.md)
