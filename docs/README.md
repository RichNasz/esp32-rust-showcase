<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# ESP32 Rust SDD Showcase — Full Documentation

This documentation is for a reader who wants an AI coding agent to help with ESP32 Rust and may not already know spec-driven development.

**Spec-driven development (SDD)** means a **spec** — Markdown a person writes — is the source of truth. The agent reads those files and **generates** the Rust firmware and these docs. The firmware is rust-on-metal: `no_std` (Rust without the standard library, running on the chip through `esp-hal`) on the ESP32-C6. As of 2026-10-20 all eleven examples are **Firmware generated**: eleven crates, zero specs-only. That label means the crate is in the tree. It is not a hardware pass.

The catalog matches that status. ESP-NOW is a sensor-to-gateway pair, not a mesh. HTTPS OTA is a slot flip with a CRC check, not a Secure Boot demo. LP-core wakeup is GPIO on RTC GPIO 0–7, not capacitive touch. Raw IEEE 802.15.4 is node and peer data frames, not Thread, Zigbee, or a mesh. Thread is a two-node UDP lab link, not a border router. Matter over Wi-Fi uses test credentials. It is not Matter over Thread and not a certified product.

**Chip ≠ board.** The chip (SoC) is Cargo feature `esp32c6` only. The board is pins, LED, and USB: default `board-devkitc-c6` (ESP32-C6-DevKitC-1-N8) or optional `board-xiao-c6` (XIAO ESP32-C6). rust-on-metal is not ESP-IDF `idf.py set-target`. Swapping an S3 or classic ESP32 spec onto the C6 is not a flag flip. No ESP-IDF install is required — [why](guides/why-esp-idf-is-not-required.md).

## Documentation sections

| Section | Description |
| --- | --- |
| [Guides](guides/README.md) | Flash an existing example, or change firmware and docs with an agent |
| [Reference](reference/README.md) | Spec format, skill phrases, pin tables, shared constraints |
| [Workflows](workflows/README.md) | Which file you edit, and which files the agent writes |
| [Board specs](../board-specs/README.md) | The two ESP32-C6 boards; see [Creating New Board Specs](guides/creating-new-board-specs.md) |

## Examples catalog

One-liners are copied from `specs/showcase-master-spec.md`. Status is the master-spec table as of 2026-10-20.

| # | Example | Description | Status |
| --- | --- | --- | --- |
| 1 | [blinky](../examples/blinky/) | breathing LED, multi-board (ESP32-C6-DevKitC-1-N8 + XIAO ESP32-C6) | Firmware generated |
| 2 | [deep-sleep-timer-node](../examples/deep-sleep-timer-node/) | RTC wakeup + persistence | Firmware generated |
| 3 | [deep-sleep-bme280-mqtt-sensor](../examples/deep-sleep-bme280-mqtt-sensor/) | BME280 readings over Wi-Fi MQTT, then deep sleep | Firmware generated |
| 4 | [hardware-aes-secure-storage](../examples/hardware-aes-secure-storage/) | hardware AES and flash-backed storage (plaintext key demo) | Firmware generated |
| 5 | [ble-beacon-deep-sleep](../examples/ble-beacon-deep-sleep/) | duty-cycled BLE beacon (demo window, not a multi-year budget) | Firmware generated |
| 6 | [secure-ota-https](../examples/secure-ota-https/) | HTTPS download into the inactive OTA slot, CRC check, reboot. Not Secure Boot or flash-encryption enrollment | Firmware generated |
| 7 | [esp-now-low-power-mesh](../examples/esp-now-low-power-mesh/) | ESP-NOW sensor-to-gateway unicast. The directory name is historical. This is not a multi-hop mesh | Firmware generated |
| 8 | [lp-core-wakeup](../examples/lp-core-wakeup/) | LP-core GPIO wakeup on RTC GPIO 0–7 (C6 has no capacitive touch) | Firmware generated |
| 9 | [ieee802154-raw-frames](../examples/ieee802154-raw-frames/) | raw IEEE 802.15.4 data frames between a node and a peer. Not Thread, not Zigbee, not a mesh | Firmware generated |
| 10 | [thread-udp-pair](../examples/thread-udp-pair/) | two-node Thread lab network. A leader and a child exchange one UDP payload. Not a border router and not Zigbee | Firmware generated |
| 11 | [matter-wifi-onoff](../examples/matter-wifi-onoff/) | Matter on/off light over Wi-Fi, commissioned over BLE with test credentials. Not Matter over Thread and not a certified product | Firmware generated |

## Agent skills

A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. You ask the agent to follow it. It is not a shell command and not a Rust crate. `skills/` is already in the clone. Purposes below are copied from the master spec. Do not treat a different sentence as the purpose.

| Skill | Purpose |
| --- | --- |
| `esp32-sdd-functional-spec-creator` | Drafts a new `FunctionalSpec.md` with you |
| `esp32-sdd-full-project-generator` | Writes the `no_std` Cargo crate from the example specs |
| `esp32-sdd-documentation-generator` | Rewrites the Markdown docs from `specs/` |
| `esp32-board-spec-generator` | Writes a board spec from a vendor page or datasheet |
| `esp32-ws2812-led-engineer` | Applies the WS2812 timing and driver constraints |
| `esp32-ble-beacon-engineer` | Designs BLE advertising and radio teardown before sleep |
| `esp32-deep-sleep-engineer` | Designs sleep, wakeup, and the power budget |
| `esp32-hardware-crypto-configurer` | Configures hardware AES and flash-backed key storage |
| `esp32-sdd-project-validator` | Checks an example against the locked crate layout |
| `esp32-lp-core-engineer` | Writes the LP-core program and the HP-core handshake |

Index: [skills/README.md](../skills/README.md). How to use them: [Using With AI Agents](guides/using-with-ai-agents.md).

## Repository layout

People write specs. The agent writes the Rust and the Markdown.

```
esp32-rust-showcase/
├── specs/                      ← human-written (repo-wide rules)
├── shared-specs/               ← human-written (cross-example constraints)
├── examples/<name>/specs/      ← human-written (Functional, Coding, Test)
├── examples/<name>/src/        ← agent-written
├── examples/<name>/Cargo.toml  ← agent-written
├── examples/<name>/build.rs    ← agent-written
├── examples/<name>/.cargo/     ← agent-written
├── examples/<name>/README.md   ← agent-written
├── board-specs/                ← agent-written
├── skills/                     ← agent-written
├── templates/                  ← agent-written
├── docs/                       ← agent-written (you are here)
├── README.md                   ← agent-written
├── CONTRIBUTING.md             ← agent-written
├── CODE_OF_CONDUCT.md          ← agent-written
├── SECURITY.md                 ← agent-written
└── AGENTS.md                   ← agent-written
```

## What a spec is

A spec is Markdown a person writes. It states what the firmware must do and what the docs must say. Specs are the source of truth for both firmware and documentation. An AI coding agent reads those files and writes the Rust and the Markdown. People do not hand-edit `src/`, `Cargo.toml`, `build.rs`, `.cargo/config.toml`, READMEs, or `docs/`. When generated output is wrong, change the spec and ask the agent to generate again. **Regenerate** means generate again: the agent overwrites the previous output and leaves the spec files in place.

## The SDD Workflow in Three Steps

1. **Write a spec** — edit `specs/`, `shared-specs/`, or `examples/<name>/specs/`.
2. **Ask the agent to follow a skill** — for a crate, say "Activate esp32-sdd-full-project-generator". That phrase goes to the agent, not the shell.
3. **Build and flash** — open the example folder and run `cargo run --release`.

Full workflows: [docs/workflows/README.md](workflows/README.md). The longer lesson is [Using With AI Agents](guides/using-with-ai-agents.md).
