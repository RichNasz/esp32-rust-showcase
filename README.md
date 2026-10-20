<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
![Rust](https://img.shields.io/badge/Rust-stable-orange)
![esp-hal](https://img.shields.io/badge/esp--hal-1.2-blue)
![Chip](https://img.shields.io/badge/chip-ESP32--C6-orange)
[![Made with Grok Build](https://img.shields.io/badge/Made%20with-Grok%20Build-black)](https://x.ai/cli)

# ESP32 Rust SDD Showcase

> **100% AI-first.** Humans write specs. AI writes everything else.

This repository is a production-grade demonstration of **Spec-Driven Development (SDD)** for **rust-on-metal ESP32** (`no_std` [`esp-hal`](https://github.com/esp-rs/esp-hal) on ESP32-C6). Humans author concise specifications; AI agents generate all firmware, documentation, and Cargo configuration from those specs. It exists to prove that strict human/AI role separation produces better embedded firmware faster, and to give the ESP32 community a reusable rust-on-metal SDD toolkit they can fork.

**Chip ≠ board.** v1 firmware is **ESP32-C6 only** (Cargo feature `esp32c6`). rust-on-metal is not ESP-IDF `idf.py set-target`: an S3 or classic ESP32 project does not become C6 by flipping a flag. Boards on that chip (DevKitC-C6 default, optional XIAO C6) change pins and USB via `board-specs/` and generated `src/board.rs`. Other SoCs in the catalog are hardware reference until a chip feature exists.

## Features

- **8 progressive examples** on ESP32-C6 — breathing LED, deep sleep, MQTT, hardware AES, a duty-cycled BLE beacon, an HTTPS OTA slot flip, an ESP-NOW sensor-to-gateway pair, and LP-core GPIO wakeup. **Eight crates are generated** as of 2026-10-20 (blinky through lp-core-wakeup). `specs/showcase-master-spec.md` still lists examples 6, 7, and 8 as Specs only; humans own that catalog
- **11 published agent skills** (10 original SDD skills plus `esp32-lp-core-engineer`) covering spec creation, crate generation, WS2812, BLE, deep sleep, LP-core, ANSI dashboards, hardware AES, board specs, documentation, and validation
- **AI-generated board specs** from vendor datasheets (Espressif, Seeed, Adafruit, LilyGo, and more); v1 **chip** is ESP32-C6 (`esp32c6`); **boards** are ESP32-C6-DevKitC-1-N8 (required) and XIAO ESP32-C6
- **Strict human/AI separation** — generated files carry agent headers; only `specs/`, `shared-specs/`, and `examples/*/specs/` are human-authored
- **No ESP-IDF install required** — rustup stable + `espflash`; firmware is `no_std` rust-on-metal via `esp-hal`, not the C SDK. [Why](docs/guides/why-esp-idf-is-not-required.md)
- **Full OSS governance** — Contributing guide, Code of Conduct, Security policy, and GitHub issue templates

## Table of Contents

- [Quick Start](#quick-start)
- [Prerequisites](#prerequisites)
- [Examples](#examples)
- [Skills](#skills)
- [Full Documentation](#full-documentation)
- [Contributing](#contributing)
- [License](#license)

## Quick Start

```sh
git clone https://github.com/RichNasz/esp32-rust-showcase.git
cd esp32-rust-showcase
# Skills live in skills/ — no extra install needed.
cd examples/blinky
cargo run --release
```

Optional XIAO ESP32-C6 board:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

`cargo run --release` builds for `riscv32imac-unknown-none-elf` and flashes via the `espflash` runner. Open **only** `examples/<name>/` in VS Code / Cursor so rust-analyzer uses that crate.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see [`shared-specs/rust-toolchain.md`](shared-specs/rust-toolchain.md))
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6

ESP-IDF, `espup`, and the `esp` rustup channel are **not** required and are not used in v1.

C developers usually expect Espressif’s **ESP-IDF** (CMake, `idf.py`, `sdkconfig`). This repository does not wrap that C SDK. Firmware talks to the chip through Espressif’s official `no_std` HAL, [`esp-hal`](https://github.com/esp-rs/esp-hal). ESP32-C6 is RISC-V, so rustup **stable** already has the compiler target; `espup` exists for Xtensa chips, which are not in v1. Build and flash with Cargo + `espflash`, not `idf.py`. Full explanation: [Why ESP-IDF is not required](docs/guides/why-esp-idf-is-not-required.md).

## Examples

Eight examples, ordered by complexity. Start with `blinky`. Required board: **ESP32-C6-DevKitC-1**. **Firmware generated** means the crate is in the tree. **Specs only** means there is nothing to flash yet. Neither label is a hardware test result. `specs/showcase-master-spec.md` still lists examples 6, 7, and 8 as Specs only; that human catalog was not edited.

| # | Example | Description | Status | Link |
|---|---|---|---|---|
| 1 | blinky | Breathing LED, multi-board (ESP32-C6-DevKitC-1-N8 + XIAO ESP32-C6) | Firmware generated | [examples/blinky](examples/blinky/) |
| 2 | deep-sleep-timer-node | RTC wakeup + persistence | Firmware generated | [examples/deep-sleep-timer-node](examples/deep-sleep-timer-node/) |
| 3 | deep-sleep-bme280-mqtt-sensor | Real IoT sensor node (BME280 → Wi-Fi STA → MQTT) | Firmware generated | [examples/deep-sleep-bme280-mqtt-sensor](examples/deep-sleep-bme280-mqtt-sensor/) |
| 4 | hardware-aes-secure-storage | Hardware AES accelerator + flash-backed storage | Firmware generated | [examples/hardware-aes-secure-storage](examples/hardware-aes-secure-storage/) |
| 5 | ble-beacon-deep-sleep | Duty-cycled BLE beacon (10 s / 10 s demo, not a multi-year budget) | Firmware generated | [examples/ble-beacon-deep-sleep](examples/ble-beacon-deep-sleep/) |
| 6 | secure-ota-https | HTTPS download into the inactive OTA slot, CRC check, reboot. Not Secure Boot | Firmware generated | [examples/secure-ota-https](examples/secure-ota-https/) |
| 7 | esp-now-low-power-mesh | ESP-NOW sensor-to-gateway unicast. The directory name is historical. Not a mesh | Firmware generated | [examples/esp-now-low-power-mesh](examples/esp-now-low-power-mesh/) |
| 8 | lp-core-wakeup | LP-core GPIO wakeup on RTC GPIO 0–7 (C6 has no capacitive touch) | Firmware generated | [examples/lp-core-wakeup](examples/lp-core-wakeup/) |

## Skills

Eleven agent skills implement the complete SDD pipeline. Invoke any skill by name in your agent.

| Skill | Purpose |
|---|---|
| [esp32-sdd-functional-spec-creator](skills/esp32-sdd-functional-spec-creator/SKILL.md) | Interactively guides creation of `FunctionalSpec.md` for a new ESP32-C6 example |
| [esp32-sdd-full-project-generator](skills/esp32-sdd-full-project-generator/SKILL.md) | Primary code-gen skill — produces a complete `no_std` `esp-hal` Cargo crate from spec files |
| [esp32-sdd-documentation-generator](skills/esp32-sdd-documentation-generator/SKILL.md) | Generates all Markdown documentation from `specs/`; run after any spec change |
| [esp32-board-spec-generator](skills/esp32-board-spec-generator/SKILL.md) | Creates structured board-spec Markdown from vendor datasheets |
| [esp32-ws2812-led-engineer](skills/esp32-ws2812-led-engineer/SKILL.md) | Canonical WS2812B RMT driver via `esp-hal` (GRB, 800 kHz NRZ, brightness cap 64/255) |
| [esp32-ble-beacon-engineer](skills/esp32-ble-beacon-engineer/SKILL.md) | BLE advertising via `esp-radio` + TrouBLE, manufacturer data, teardown before deep sleep |
| [esp32-deep-sleep-engineer](skills/esp32-deep-sleep-engineer/SKILL.md) | `esp-hal` RTC sleep, `#[esp_hal::ram(persistent)]` state, power budgets, radio teardown |
| [esp32-hardware-crypto-configurer](skills/esp32-hardware-crypto-configurer/SKILL.md) | Hardware AES via `esp-hal` and key storage via `esp-storage` |
| [esp32-ansi-monitor-engineer](skills/esp32-ansi-monitor-engineer/SKILL.md) | ANSI TUI serial dashboards over UART via `esp-println` (Patterns A / B / B-Lifecycle) |
| [esp32-sdd-project-validator](skills/esp32-sdd-project-validator/SKILL.md) | Validates any example against the locked no_std crate structure; reports PASS/FAIL |
| [esp32-lp-core-engineer](skills/esp32-lp-core-engineer/SKILL.md) | LP-core firmware (`esp-lp-hal`), HP `load_lp_code` handshake, GPIO 0–7 only on C6 |

## Full Documentation

| Section | Link |
|---|---|
| Documentation hub | [docs/README.md](docs/README.md) |
| Why ESP-IDF is not required | [docs/guides/why-esp-idf-is-not-required.md](docs/guides/why-esp-idf-is-not-required.md) |
| SDD workflow guide | [docs/workflows/README.md](docs/workflows/README.md)
| Using with AI agents | [docs/guides/using-with-ai-agents.md](docs/guides/using-with-ai-agents.md) |
| Creating new board specs | [docs/guides/creating-new-board-specs.md](docs/guides/creating-new-board-specs.md) |
| Examples catalog | [examples/README.md](examples/README.md) |
| Board specs | [board-specs/README.md](board-specs/README.md) |
| Skills reference | [skills/README.md](skills/README.md) |
| Specs (human-authored) | [specs/README.md](specs/README.md) |

## Contributing

Contributions are welcome — but this project follows strict SDD rules. Only files inside `specs/`, `shared-specs/`, and `examples/*/specs/` are human-authored; everything else is agent-generated and must not be edited by hand. If something looks wrong in a generated file, fix the driving spec and regenerate. See [CONTRIBUTING.md](CONTRIBUTING.md) for the full workflow, commit convention, and PR checklist.

## License

MIT — see [LICENSE](LICENSE).

---

*Generated by [esp32-sdd-documentation-generator](skills/esp32-sdd-documentation-generator/SKILL.md) — Grok*
