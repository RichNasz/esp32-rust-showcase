<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
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

`secure-ota-https` has a generated crate as of 2026-10-20. It is an HTTPS slot flip: TLS 1.3 via `embedded-tls` 0.19.0 (`rustpki`, one embedded CA, direct signature), profile `dual-ota` (`partitions.csv` / `espflash.toml`, not sdkconfig), and a CRC32 check before the inactive slot is activated. Stock `espflash` does not auto-rollback. This is not Secure Boot and not an eFuse burn. Radio crates, including `esp-storage`, git-pin the `esp-hal` monorepo at tag `esp-hal-v1.2.1` — see [`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md). `esp-radio` is an **unstable beta**. Host builds are clean; that is not a hardware result. `specs/showcase-master-spec.md` still lists this example as Specs only.

## Invoke

> Activate esp32-hardware-crypto-configurer skill
