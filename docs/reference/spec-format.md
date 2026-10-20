<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Spec Format

A **spec** is Markdown a person writes. **Spec-driven development (SDD)** means that file is the source of truth. The agent **generates** the Rust and the docs from it. **Regenerate** means generate again: agent output is overwritten and the spec stays. A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. How this fits together: [Using With AI Agents](../guides/using-with-ai-agents.md).

Example specs are human-written. They live in `examples/<name>/specs/` and do not start with an agent-generated header. The section list to copy is [`templates/FunctionalSpec.template.md`](../../templates/FunctionalSpec.template.md). The directory lock is [`specs/example-project-structure-spec.md`](../../specs/example-project-structure-spec.md).

## Set up the three files

A new example starts with a row in `specs/showcase-master-spec.md`, then three files under `examples/<name>/specs/`. **FunctionalSpec** is observable behavior. **CodingSpec** is architecture and constraints with no code. **TestSpec** is how to check the example. `specs/` is repo-wide rules. `shared-specs/` is cross-example platform constraints. Say `Activate esp32-sdd-functional-spec-creator` to the agent to draft the FunctionalSpec with you. You write CodingSpec and TestSpec.

## Modify them

Behavior changes go in `FunctionalSpec.md`. Architecture, crates, and gotchas go in `CodingSpec.md`. Checks go in `TestSpec.md`. Repo-wide documentation rules go in `specs/`. Edit those files. Do not hand-edit `src/`, `Cargo.toml`, or a generated README. Ask the agent to regenerate.

## FunctionalSpec.md

Describes what a person can observe, and why the example exists.

| Section | Purpose |
| --- | --- |
| Overview | 2–4 sentences: what the example demonstrates |
| Target Hardware | **SoC** named separately from the board checklist (below) |
| Features | Observable behavior |
| Wakeup Strategy | Timer, RTC GPIO 0–7, LP-core, or none |
| Connectivity | None, BLE, Wi-Fi, ESP-NOW, MQTT, raw IEEE 802.15.4, Thread, or Matter over Wi-Fi. Radio examples copy the pin from `shared-specs/rust-toolchain.md`. Raw 802.15.4 and Thread do not also enable Wi-Fi. Matter over Wi-Fi enables `wifi`, `ble`, and `coex`, and does not enable `ieee802154` |
| Security Requirements | None, AES / `esp-storage`, or TLS / CRC OTA. CRC OTA is not the AES skill |
| Success Criteria | Includes `cargo build --release`, `cargo clippy -- -D warnings`, and `cargo run --release` |
| Implementation status | The label, date, and note from `specs/showcase-master-spec.md` |

If the spec has a heading that contains **Limitations**, the generated example README must reproduce it.

### Target Hardware

Chip (SoC) and board are different axes. Name **one** SoC. Do not describe behavior that needs another SoC's peripherals. rust-on-metal is not `idf.py set-target`.

| Item | v1 value |
| --- | --- |
| **SoC** | ESP32-C6 only |
| **Chip Cargo feature** | `esp32c6` |
| **rustc target** | `riscv32imac-unknown-none-elf` |

Board Compatibility Checklist — C6 boards only. Pins come from `board-specs/`, not from this file:

- [x] Espressif ESP32-C6-DevKitC-1-N8 *(default)* — feature `board-devkitc-c6`
- [ ] Seeed XIAO ESP32-C6 — feature `board-xiao-c6`

Do not add non-C6 boards. RTC / LP GPIOs on the C6 are **0–7 only**. GPIO 9 (XIAO boot button) is not RTC-capable. Switching SoC is a new spec plus a `rust-toolchain.md` row, not a feature flip.

## CodingSpec.md

High-level guidance: architecture, constraints, preferred crates and why, non-functional requirements, trade-offs, and chip-class gotchas (C6 RTC GPIO 0–7, `unstable` sleep and radio, pins only in `board.rs`).

| Include | Never include |
| --- | --- |
| Architecture and module breakdown | Code snippets, `const` item definitions, or pseudo-code |
| Timing, memory, power, and security constraints | Function signatures or API call sequences |
| Preferred crates and the reason for each | Numbered step-by-step implementation instructions |
| Trade-offs and known gotchas | Exact log-message strings |

Philosophy: "The model we use today is the worst model we will ever use." A high-level spec benefits from a better model later. A low-level spec duplicates code in prose.

## TestSpec.md

Philosophy: automated first, manual only when hardware observation is unavoidable.

| Tier | What belongs there |
| --- | --- |
| 1. Automated — native Cargo / rustc | `cargo build --release`, `cargo clippy -- -D warnings`, `cargo run --release` / Wokwi |
| 2. Automated — additional tools | Only with an explicit justification and documented setup |
| 3. Manual — hardware-only | LED, RF, or current that cannot be simulated. USB-during-sleep checks name the board (DevKitC left UART-C versus XIAO native USB) |

Do not specify `idf.py`, ESP-IDF Unity, or a CMake harness. There is no `sdkconfig`.
