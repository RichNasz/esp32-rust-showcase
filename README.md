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

> People write specs. An AI coding agent writes the firmware and the docs.

This repository shows how to use an AI coding agent for rust-on-metal ESP32-C6 firmware (`no_std` [`esp-hal`](https://github.com/esp-rs/esp-hal)). The method is spec-driven development (SDD): a **spec** is Markdown a person writes, and it is the source of truth for both the Rust and the documentation. The agent reads those files and generates everything else. You do not need to have used that method before. The lesson is [Using an AI coding agent](docs/guides/using-with-ai-agents.md).

**Chip ≠ board.** v1 firmware is ESP32-C6 only (Cargo feature `esp32c6`). rust-on-metal is not ESP-IDF `idf.py set-target`: an S3 or classic ESP32 project does not become C6 by flipping a flag. Boards on that chip (DevKitC-C6 default, optional XIAO C6) change pins and USB through `board-specs/` and generated `src/board.rs`. The board catalog is those two boards.

## Features

- **Eleven examples** on ESP32-C6 — a breathing LED, deep sleep, a BME280 MQTT sensor, hardware AES, a BLE beacon, an HTTPS OTA slot flip, an ESP-NOW sensor-to-gateway pair, LP-core GPIO wakeup, raw IEEE 802.15.4 frames, a two-node Thread UDP lab link, and a Matter on/off light over Wi-Fi. **Eleven crates are generated** and **zero examples are specs only** as of 2026-10-20. **Firmware generated** means the crate is in the tree. It is not a hardware test result. Thread is not a border router. Matter over Wi-Fi uses test credentials and is not Matter over Thread and not a certified product.
- **Ten skills** — Markdown workflows in `skills/` that you ask the agent to follow. They ship in this clone. A separate install is not required.
- **Board specs** from Espressif and Seeed vendor docs. The agent turns them into `src/board.rs`.
- **People edit specs. The agent writes Rust and Markdown.** When output is wrong, change the spec and generate again.
- **No ESP-IDF install.** rustup stable and `espflash`. Firmware is `no_std` `esp-hal`, not the C SDK. [Why](docs/guides/why-esp-idf-is-not-required.md).

## Table of Contents

- [Quick Start](#quick-start)
- [Prerequisites](#prerequisites)
- [Examples](#examples)
- [Skills](#skills)
- [Full Documentation](#full-documentation)
- [Contributing](#contributing)
- [License](#license)

## Quick Start

### Task A — flash an example

No AI agent is required. This builds the crate already in the tree and flashes it. Setup detail: [Toolchain](docs/guides/01-toolchain-setup.md) and [Run blinky](docs/guides/03-first-example.md).

```sh
git clone https://github.com/RichNasz/esp32-rust-showcase.git
cd esp32-rust-showcase/examples/blinky
cargo run --release
```

Optional XIAO ESP32-C6 board:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

`cargo run --release` builds for `riscv32imac-unknown-none-elf` and flashes through the `espflash` runner.

### Task B — change firmware or docs with an AI coding agent

A spec is the Markdown that says what the firmware must do. You edit that file. The agent writes the Rust.

1. Open this repository in the agent. The workflows are already in `skills/`. A separate skill install is not required.
2. Read [Using an AI coding agent](docs/guides/using-with-ai-agents.md) for how specs drive code and docs.
3. Edit a human spec. For the blink example, `examples/blinky/specs/FunctionalSpec.md` is what the LED does.
4. Ask the agent to generate. These lines are typed to the agent, not in the shell.
   - Crate: `Activate esp32-sdd-full-project-generator`
   - Markdown: `Activate esp32-sdd-documentation-generator and rebuild all documentation.`

Open **only** `examples/<name>/` in VS Code or Cursor when you are building that crate, so rust-analyzer uses it.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see [`shared-specs/rust-toolchain.md`](shared-specs/rust-toolchain.md))
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6

ESP-IDF, `idf.py`, `espup`, and the `esp` rustup channel are not prerequisites. This repository uses Espressif’s `no_std` HAL, `esp-hal`. ESP32-C6 is RISC-V, so rustup stable already has the compiler target. `espup` exists for Xtensa chips, which are not in v1. Build and flash with Cargo and `espflash`. Full explanation: [Why ESP-IDF is not required](docs/guides/why-esp-idf-is-not-required.md).

## Examples

Eleven examples, ordered by complexity. Start with `blinky`. **Firmware generated** means the crate is in the tree. It is not a hardware test result. As of 2026-10-20 every row is Firmware generated.

| # | Example | Description | Status | Link |
|---|---|---|---|---|
| 1 | blinky | Breathing LED, multi-board (ESP32-C6-DevKitC-1-N8 + XIAO ESP32-C6) | Firmware generated | [examples/blinky](examples/blinky/) |
| 2 | deep-sleep-timer-node | RTC wakeup + persistence | Firmware generated | [examples/deep-sleep-timer-node](examples/deep-sleep-timer-node/) |
| 3 | deep-sleep-bme280-mqtt-sensor | BME280 readings over Wi-Fi MQTT, then deep sleep | Firmware generated | [examples/deep-sleep-bme280-mqtt-sensor](examples/deep-sleep-bme280-mqtt-sensor/) |
| 4 | hardware-aes-secure-storage | Hardware AES and flash-backed storage (plaintext key demo) | Firmware generated | [examples/hardware-aes-secure-storage](examples/hardware-aes-secure-storage/) |
| 5 | ble-beacon-deep-sleep | Duty-cycled BLE beacon (demo window, not a multi-year budget) | Firmware generated | [examples/ble-beacon-deep-sleep](examples/ble-beacon-deep-sleep/) |
| 6 | secure-ota-https | HTTPS download into the inactive OTA slot, CRC check, reboot. Not Secure Boot or flash-encryption enrollment | Firmware generated | [examples/secure-ota-https](examples/secure-ota-https/) |
| 7 | esp-now-low-power-mesh | ESP-NOW sensor-to-gateway unicast. The directory name is historical. This is not a multi-hop mesh | Firmware generated | [examples/esp-now-low-power-mesh](examples/esp-now-low-power-mesh/) |
| 8 | lp-core-wakeup | LP-core GPIO wakeup on RTC GPIO 0–7 (C6 has no capacitive touch) | Firmware generated | [examples/lp-core-wakeup](examples/lp-core-wakeup/) |
| 9 | ieee802154-raw-frames | Raw IEEE 802.15.4 data frames between a node and a peer. Not Thread, not Zigbee, not a mesh | Firmware generated | [examples/ieee802154-raw-frames](examples/ieee802154-raw-frames/) |
| 10 | thread-udp-pair | Two-node Thread lab network. A leader and a child exchange one UDP payload. Not a border router and not Zigbee | Firmware generated | [examples/thread-udp-pair](examples/thread-udp-pair/) |
| 11 | matter-wifi-onoff | Matter on/off light over Wi-Fi, commissioned over BLE with test credentials. Not Matter over Thread and not a certified product | Firmware generated | [examples/matter-wifi-onoff](examples/matter-wifi-onoff/) |

## Skills

A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You ask the agent to follow it. It is not a shell command. Purposes come from `specs/showcase-master-spec.md`.

| Skill | Purpose |
|---|---|
| [esp32-sdd-functional-spec-creator](skills/esp32-sdd-functional-spec-creator/SKILL.md) | Drafts a new `FunctionalSpec.md` with you |
| [esp32-sdd-full-project-generator](skills/esp32-sdd-full-project-generator/SKILL.md) | Writes the `no_std` Cargo crate from the example specs |
| [esp32-sdd-documentation-generator](skills/esp32-sdd-documentation-generator/SKILL.md) | Rewrites the Markdown docs from `specs/` |
| [esp32-board-spec-generator](skills/esp32-board-spec-generator/SKILL.md) | Writes a board spec from a vendor page or datasheet |
| [esp32-ws2812-led-engineer](skills/esp32-ws2812-led-engineer/SKILL.md) | Applies the WS2812 timing and driver constraints |
| [esp32-ble-beacon-engineer](skills/esp32-ble-beacon-engineer/SKILL.md) | Designs BLE advertising and radio teardown before sleep |
| [esp32-deep-sleep-engineer](skills/esp32-deep-sleep-engineer/SKILL.md) | Designs sleep, wakeup, and the power budget |
| [esp32-hardware-crypto-configurer](skills/esp32-hardware-crypto-configurer/SKILL.md) | Configures hardware AES and flash-backed key storage |
| [esp32-sdd-project-validator](skills/esp32-sdd-project-validator/SKILL.md) | Checks an example against the locked crate layout |
| [esp32-lp-core-engineer](skills/esp32-lp-core-engineer/SKILL.md) | Writes the LP-core program and the HP-core handshake |

## Full Documentation

| Section | Link |
|---|---|
| Documentation hub | [docs/README.md](docs/README.md) |
| Why ESP-IDF is not required | [docs/guides/why-esp-idf-is-not-required.md](docs/guides/why-esp-idf-is-not-required.md) |
| Guides | [docs/guides/README.md](docs/guides/README.md) |
| Using an AI coding agent | [docs/guides/using-with-ai-agents.md](docs/guides/using-with-ai-agents.md) |
| Workflows | [docs/workflows/README.md](docs/workflows/README.md) |
| Reference | [docs/reference/README.md](docs/reference/README.md) |
| Examples catalog | [examples/README.md](examples/README.md) |
| Board specs | [board-specs/README.md](board-specs/README.md) |
| Skills | [skills/README.md](skills/README.md) |
| Human specs | [specs/README.md](specs/README.md) |

## Contributing

A change starts as an edit to a spec. The agent regenerates the Rust or the Markdown, and the pull request contains both. People do not hand-edit generated files. See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT — see [LICENSE](LICENSE).

---

*Generated by [esp32-sdd-documentation-generator](skills/esp32-sdd-documentation-generator/SKILL.md) — Grok*
