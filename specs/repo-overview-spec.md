# Repo Overview Specification

## Purpose
Show how to use an AI coding agent for rust-on-metal ESP32-C6 firmware (`no_std` `esp-hal`).

The method is spec-driven development: a person writes Markdown specs, and the agent generates the firmware and the documentation from those specs. Readers are not assumed to know that method. The root README and `docs/README.md` follow `specs/documentation-generation-spec.md` section **Readers**, including the plain-language definition of a spec and the link to `docs/guides/using-with-ai-agents.md`.

## Key Features
Copy example one-liners and skill purposes from `specs/showcase-master-spec.md`. Do not invent a second list.

- Board specs generated from vendor docs (Espressif and Seeed), then turned into `src/board.rs`
- 9 examples on ESP32-C6: breathing LED, deep sleep, a BME280 MQTT sensor, hardware AES, a BLE beacon, HTTPS OTA slot flip, an ESP-NOW sensor-to-gateway pair, LP-core GPIO wakeup, and raw IEEE 802.15.4 frames. Status counts come from the master-spec table.
- Skills: Markdown workflows in `skills/<name>/SKILL.md` that a person asks the agent to follow. They are not a separate install.
- People edit specs. The agent writes Rust and Markdown.
- No ESP-IDF / `idf.py` / `sdkconfig` — rust-on-metal `esp-hal` on RISC-V ESP32-C6 (see Why ESP-IDF is not required)
- Chip (SoC) ≠ board: v1 chip feature `esp32c6` only; boards are `board-devkitc-c6` / `board-xiao-c6`. rust-on-metal is not IDF `set-target` portability

## Why ESP-IDF is not required
C developers arriving from ESP-IDF should be told *why* they skip that toolkit, not only that it is unused. Generated docs (root README Prerequisites + `docs/guides/why-esp-idf-is-not-required.md`) must follow `specs/documentation-generation-spec.md` section “Guide: Why ESP-IDF is not required”.

This repository uses Espressif’s official `no_std` `esp-hal` path. It does **not** use the community `esp-idf-*` crates that wrap the C SDK. ESP32-C6 is RISC-V, so rustup **stable** plus target `riscv32imac-unknown-none-elf` is sufficient; `espup` is for Xtensa. Build/flash is Cargo + `espflash`.

## Hardware bar
ESP32-C6-DevKitC-1-N8 required; XIAO ESP32-C6 also supported. The board catalog is those two ESP32-C6 boards. Do not add other SoCs. Chip feature `esp32c6` only.

## Quick Start
The root README Quick Start follows `specs/documentation-generation-spec.md` section **Quick Start (root README)**. Two tasks. Do not tell the reader to install skills as a required step. `skills/` is already in the clone.

**Task A — flash blinky.** No AI agent.
1. git clone this repo
2. rustup stable + `rust-src` + target `riscv32imac-unknown-none-elf`; `cargo install espflash`
3. `cd examples/blinky && cargo run --release`

**Task B — change firmware or docs with an AI coding agent.** The reader may not know what a spec is. Point them at `docs/guides/using-with-ai-agents.md` before any activate phrase.
1. Open the cloned repo in the agent.
2. Edit a human spec. `examples/blinky/specs/FunctionalSpec.md` is what the LED does.
3. Ask the agent: "Activate esp32-sdd-full-project-generator" for the crate, or "Activate esp32-sdd-documentation-generator and rebuild all documentation." for Markdown. Those lines go to the agent, not the shell.

Full documentation → [docs/README.md](docs/README.md)
