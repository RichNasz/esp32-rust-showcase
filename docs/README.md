<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# ESP32 Rust SDD Showcase — Full Documentation

> **100% AI-first.** Every line of firmware and every piece of documentation in this repository is generated exclusively by AI agents from human-written specifications in `specs/`.

## What Is This Project?

The ESP32 Rust SDD Showcase is a reference implementation of **Spec-Driven Development (SDD)** for **rust-on-metal ESP32** (`no_std` `esp-hal` on ESP32-C6). Humans write specs. Agents generate firmware and these docs. As of 2026-10-20 the tree has generated crates for all eight examples, including LP-core GPIO wakeup. `specs/showcase-master-spec.md` still lists the OTA example, the ESP-NOW example, and `lp-core-wakeup` as Specs only; humans own that catalog.

**Chip ≠ board.** v1 firmware is **ESP32-C6 only** (`esp32c6`). rust-on-metal does not port across ESP32 / S3 / C6 the way ESP-IDF `idf.py set-target` often does. Required **board:** ESP32-C6-DevKitC-1-N8. XIAO ESP32-C6 is also supported. Other SoCs in `board-specs/` are hardware reference until a chip feature exists. No ESP-IDF install is required — [why](guides/why-esp-idf-is-not-required.md).

## Documentation Sections

| Section | Description |
|---|---|
| [Guides](guides/README.md) | Step-by-step tutorials: why no ESP-IDF, rustup + espflash, running your first example, adding a new board |
| [Reference](reference/README.md) | Toolchain pins, Cargo conventions, pin mapping tables, spec format |
| [Workflows](workflows/README.md) | The complete SDD workflow: from spec to flashed firmware |
| [Board Specs](../board-specs/README.md) | AI-generated hardware specs for supported ESP32 boards; see also [Creating New Board Specs](guides/creating-new-board-specs.md) |

## Examples Catalog

| # | Example | Concept | Status |
|---|---|---|---|
| 1 | [blinky](../examples/blinky/) | Baseline: SDD structure + breathing LED, multi-board C6 | Firmware generated |
| 2 | [deep-sleep-timer-node](../examples/deep-sleep-timer-node/) | RTC wakeup + persistent RAM | Firmware generated |
| 3 | [deep-sleep-bme280-mqtt-sensor](../examples/deep-sleep-bme280-mqtt-sensor/) | IoT sensor node: BME280 + Wi-Fi + MQTT | Firmware generated |
| 4 | [hardware-aes-secure-storage](../examples/hardware-aes-secure-storage/) | Hardware AES accelerator (plaintext key demo) | Firmware generated |
| 5 | [ble-beacon-deep-sleep](../examples/ble-beacon-deep-sleep/) | Duty-cycled BLE beacon (10 s / 10 s demo) | Firmware generated |
| 6 | [secure-ota-https](../examples/secure-ota-https/) | HTTPS slot flip, TLS 1.3, CRC. Not Secure Boot or auto-rollback | Firmware generated |
| 7 | [esp-now-low-power-mesh](../examples/esp-now-low-power-mesh/) | ESP-NOW sensor-to-gateway unicast (not a mesh) | Firmware generated |
| 8 | [lp-core-wakeup](../examples/lp-core-wakeup/) | LP-core GPIO 0 wakeup | Firmware generated |

## Agent Skills

Eleven skills implement the full SDD pipeline (10 original plus `esp32-lp-core-engineer`). Skills live in `skills/<name>/SKILL.md` inside this repository. No extra install command is required once you open the project in an agent that reads that tree.

See [docs/guides/using-with-ai-agents.md](guides/using-with-ai-agents.md) for setup and workflow details.

| Skill | Purpose |
|---|---|
| `esp32-sdd-functional-spec-creator` | Guides creation of a new `FunctionalSpec.md` |
| `esp32-sdd-full-project-generator` | Generates a complete `no_std` `esp-hal` Cargo crate from specs |
| `esp32-board-spec-generator` | Produces board specs from vendor datasheets |
| `esp32-ws2812-led-engineer` | Generates WS2812B RMT driver via `esp-hal` |
| `esp32-ble-beacon-engineer` | Designs `esp-radio` + TrouBLE advertising and teardown before deep sleep |
| `esp32-deep-sleep-engineer` | Designs sleep configurations and power budgets |
| `esp32-hardware-crypto-configurer` | Configures hardware AES and `esp-storage` key storage |
| `esp32-ansi-monitor-engineer` | Generates ANSI TUI serial dashboards via `esp-println` |
| `esp32-sdd-documentation-generator` | Regenerates all documentation from specs (this file!) |
| `esp32-sdd-project-validator` | Validates any example against the locked crate structure |
| `esp32-lp-core-engineer` | LP-core firmware (`esp-lp-hal`) and HP load/handshake |

See [skills/README.md](../skills/README.md) for full details.

## Repository Layout

```
esp32-rust-showcase/
├── specs/              ← HUMAN-AUTHORED: source of truth for everything
├── shared-specs/       ← HUMAN-AUTHORED: cross-example platform constraints
├── examples/           ← AGENT-GENERATED crates (specs/ is human)
├── board-specs/        ← AGENT-GENERATED board hardware specs
├── skills/             ← AGENT-GENERATED skill definitions
├── templates/          ← AGENT-GENERATED crate and spec templates
└── docs/               ← AGENT-GENERATED documentation (you are here)
```

## The SDD Workflow in Three Steps

1. **Write a spec** — edit or create a file in `specs/` or `examples/*/specs/FunctionalSpec.md`
2. **Activate a skill** — say "Activate esp32-sdd-full-project-generator skill"
3. **Build and flash** — open the example folder in VS Code / Cursor and run `cargo run --release`

Full workflow details: [docs/workflows/README.md](workflows/README.md)

## IDE Integration

Each example is a standalone Cargo package. For the best development experience open **only the example folder** (e.g. `examples/blinky/`) in VS Code or Cursor — not the repository root — so rust-analyzer uses that crate.

See [guides/using-with-ai-agents.md](guides/using-with-ai-agents.md) for full setup details.

## Contributing

Only `specs/`, `shared-specs/`, and `examples/*/specs/{Functional,Coding,Test}Spec.md` are open to human edits. Everything else is agent-generated. After any spec change:

> "Activate esp32-sdd-documentation-generator and rebuild all documentation."

See [specs/contributing-spec.md](../specs/contributing-spec.md) for the full contribution rules.
