<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# 07 — Secure Your Project

Use `esp32-hardware-crypto-configurer` when a FunctionalSpec requires AES, secure storage, or related key handling.

## What this skill does

- Hardware AES via `esp-hal` (not ESP-IDF mbedTLS, not Kconfig).
- Key material in `esp-storage` (flash-backed), not a hard-coded constant in `main.rs`.
- Aligns with [`shared-specs/security-baseline.md`](../../shared-specs/security-baseline.md).

## What this skill does not do

Chip-level secure boot, eFuse programming, and silicon flash-encryption enablement are **out of scope** for generated examples (see [SECURITY.md](../../SECURITY.md)).

## OTA

`secure-ota-https` uses HTTPS + embedded CA + dual OTA partitions (`partitions.csv` / `espflash.toml`, not sdkconfig). Radio examples git-pin the `esp-hal` monorepo stack at tag `esp-hal-v1.2.1` — see [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md). `esp-radio` is an **unstable beta**.

## Invoke

> Activate esp32-hardware-crypto-configurer skill
