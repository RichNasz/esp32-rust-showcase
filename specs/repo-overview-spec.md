# Repo Overview Specification

## Purpose
Comprehensive showcase of Spec-Driven Development (SDD) for rust-on-metal ESP32 (`no_std` `esp-hal` on ESP32-C6).
Every line of firmware and every piece of documentation is generated exclusively by AI agents from human-written specifications.

## Key Features
- AI-generated board specs from vendor docs (Espressif, Seeed, Adafruit, LilyGo, etc.)
- 8 progressive examples on ESP32-C6 leveraging hardware superpowers (deep sleep, LP core, radio, AES, OTA)
- Full suite of published Agent Skills (`skills/<name>/SKILL.md`; mirrored for Claude; Grok via `AGENTS.md`)
- Strict 100 % AI-first workflow: humans edit specs only; agents generate Rust and Markdown

## Hardware bar
ESP32-C6-DevKitC-1-N8 required; XIAO ESP32-C6 also supported. Other chips from the C-language showcase are not in v1.

## Quick Start
1. git clone this repo
2. Install Agent Skills from this repository's `skills/` tree
3. rustup stable + `rust-src` + target `riscv32imac-unknown-none-elf`; `cargo install espflash`
4. `cd examples/blinky && cargo run --release`
5. Edit any `FunctionalSpec.md` → activate the full-project-generator skill

Full documentation → [docs/README.md](docs/README.md)
