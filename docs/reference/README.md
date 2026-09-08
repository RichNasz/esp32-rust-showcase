<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Reference

Quick-reference documentation for the ESP32 Rust SDD Showcase — toolchain pins, Cargo conventions, pin tables, and spec format.

There is **no sdkconfig**. Chip, board, and optional behaviour are Cargo features plus generated `src/board.rs`.

## Available References

| Reference | Description |
|---|---|
| [pin-mapping-tables.md](pin-mapping-tables.md) | GPIO → peripheral tables for v1 C6 boards (auto-derived from `board-specs/`) |
| [spec-format.md](spec-format.md) | Required sections for `FunctionalSpec.md`, `CodingSpec.md`, and `TestSpec.md` |
| [skill-invocation.md](skill-invocation.md) | Canonical invocation phrases for all eleven skills |

## Human-authored shared specs (source of truth)

| File | Description |
|---|---|
| [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md) | rustup stable, MSRV 1.95.0, crate pins, radio git-pin |
| [`shared-specs/cargo-conventions.md`](../../shared-specs/cargo-conventions.md) | Standalone crates, board features, `build.rs`, `.cargo/config.toml` |
| [`shared-specs/coding-conventions.md`](../../shared-specs/coding-conventions.md) | `no_std` Rust style, error handling, Embassy notes |
| [`shared-specs/partition-table-profiles.md`](../../shared-specs/partition-table-profiles.md) | CSV profiles for espflash (not Kconfig) |
| [`shared-specs/power-budget-template.md`](../../shared-specs/power-budget-template.md) | Power budget template for low-power examples |
| [`shared-specs/security-baseline.md`](../../shared-specs/security-baseline.md) | Minimum security requirements |
| [`shared-specs/ble-patterns.md`](../../shared-specs/ble-patterns.md) | BLE advertising / manufacturer-data patterns |
| [`shared-specs/AIGenLessonsLearned.md`](../../shared-specs/AIGenLessonsLearned.md) | **Mandatory first read** |

## Toolchain

All reference material targets rustup **stable**, MSRV **1.95.0**, `esp-hal` **1.2.1**, chip feature `esp32c6`, target `riscv32imac-unknown-none-elf`. See `shared-specs/rust-toolchain.md`.

## Chip variant notes (v1)

| Feature | ESP32-C6 (v1 required) | Other C-showcase chips |
|---|---|---|
| Status | **Required** | Not in v1 |
| Hardware AES | Yes | — |
| BLE | BLE 5.3 | — |
| Wi-Fi | 2.4 GHz Wi-Fi 6 (ax) | — |
| RTC / LP GPIO | **0–7 only** | — |
| LP core | RISC-V 20 MHz (`esp-lp-hal`) | — |
| Capacitive touch | No (use `lp-core-wakeup`) | — |

Consult the relevant board spec in `board-specs/` for your exact module.
