<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Reference

Quick reference for spec sections, the phrases you type to an agent, and the two v1 boards' pins.

A **spec** is Markdown a person writes. **Spec-driven development (SDD)** means that file is the source of truth for firmware and docs. A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`. **Generate** means the agent writes Rust or Markdown from the specs. **Regenerate** means generate again. The lesson is [Using With AI Agents](../guides/using-with-ai-agents.md).

There is **no sdkconfig**. Chip, board, and optional behavior are Cargo features plus generated `src/board.rs`. Do not document `menuconfig`.

## Pages

| Page | What it is for |
| --- | --- |
| [spec-format.md](spec-format.md) | How to set up and edit FunctionalSpec, CodingSpec, and TestSpec |
| [skill-invocation.md](skill-invocation.md) | The phrase to type for each skill, and what the agent writes |
| [pin-mapping-tables.md](pin-mapping-tables.md) | GPIO summary for the two ESP32-C6 boards, taken from `board-specs/` |

## Human-written shared specs

| File | What it constrains |
| --- | --- |
| [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md) | rustup stable, MSRV 1.95.0, target, crate pins |
| [`shared-specs/cargo-conventions.md`](../../shared-specs/cargo-conventions.md) | Standalone crates, board features, `build.rs`, `.cargo/config.toml` |
| [`shared-specs/coding-conventions.md`](../../shared-specs/coding-conventions.md) | `no_std` style, errors, Embassy notes |
| [`shared-specs/partition-table-profiles.md`](../../shared-specs/partition-table-profiles.md) | CSV profiles for espflash (not Kconfig) |
| [`shared-specs/power-budget-template.md`](../../shared-specs/power-budget-template.md) | Power budget for examples that sleep |
| [`shared-specs/security-baseline.md`](../../shared-specs/security-baseline.md) | Minimum security requirements |
| [`shared-specs/ble-patterns.md`](../../shared-specs/ble-patterns.md) | BLE advertising and manufacturer data |
| [`shared-specs/AIGenLessonsLearned.md`](../../shared-specs/AIGenLessonsLearned.md) | Mandatory first read before generate or debug |

## Toolchain

rustup **stable**, MSRV **1.95.0**, `esp-hal` **1.2.1**, chip feature `esp32c6`, target `riscv32imac-unknown-none-elf`. Flash with `espflash`. Not the `esp` rustup channel, and not `espup`.

## Chip variant notes (v1)

| Feature | ESP32-C6 (v1) |
| --- | --- |
| Status | **Required.** v1 has no other SoC |
| rustc target | `riscv32imac-unknown-none-elf` |
| Hardware AES | Yes |
| BLE | BLE 5.3 |
| Wi-Fi | 2.4 GHz Wi-Fi 6 (ax) |
| RTC / LP GPIO | **0–7 only** |
| LP core | RISC-V LP core (`esp-lp-hal`) |
| Capacitive touch | **No.** The ESP32-C6 has no touch peripheral |
| `lp-core-wakeup` | LP-core GPIO wakeup on RTC GPIO 0–7. **Firmware generated** as of 2026-10-20. That label is not a hardware pass |

Capacitive touch is absent on this chip. Do not treat `lp-core-wakeup` as a touch example.

Board pins: [`board-specs/`](../../board-specs/README.md). Chip ≠ board. `esp32c6` is the SoC. `board-devkitc-c6` and `board-xiao-c6` are boards. rust-on-metal is not `idf.py set-target`.
