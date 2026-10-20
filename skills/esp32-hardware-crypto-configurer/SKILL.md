---
name: esp32-hardware-crypto-configurer
description: Configures ESP32-C6 hardware AES via esp-hal and key storage via esp-storage. Generates no_std encrypt/decrypt + flash-backed key workflows. Does not use ESP-IDF mbedTLS, Kconfig, or sdkconfig. Does not handle CRC HTTPS OTA or Secure Boot.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, security, aes, esp-hal, esp-storage, crypto, rust, no_std
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 Hardware Crypto Configurer

## When to use
When a FunctionalSpec requires hardware AES or flash-backed key storage. Invoked by `esp32-sdd-full-project-generator` for those requirements only.

Do not run this skill for HTTPS OTA that checks a CRC and flips a slot. That design is `shared-specs/security-baseline.md` and the generator's HTTPS OTA steps. CRC is not a signature and not Secure Boot.

Follow `shared-specs/security-baseline.md`. ESP-IDF Kconfig (`CONFIG_SECURE_*`, `MBEDTLS_*_ALT`) **does not apply**.

## Workflow (follow exactly)
MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else.

1. Read the FunctionalSpec.md security requirements section.

2. Select the minimal hardware path needed. v1 default for this skill is the **ESP32-C6 AES accelerator** via `esp_hal::aes` (feature `unstable` in addition to `esp32c6`). Document the mode (AES-128 vs AES-256, ECB vs CBC) and the rationale. Prefer hardware AES over a pure-software crate.

3. Generate a `src/` module (e.g. `src/crypto.rs`) that:
   - Constructs `esp_hal::aes::Aes` (or `AesContext` / work-queue driver on the 1.2 line) from `peripherals.AES`.
   - Encrypts and decrypts with `Operation::Encrypt` / `Operation::Decrypt`; key length is taken from the key, not a mode enum.
   - Uses CBC (or the mode the spec requires) with a unique IV per blob when storing more than one block.
   - Never `unwrap()` on fallible crypto / storage in production paths.
   - Never logs key material, IVs used as secrets, or plaintext credentials (`security-baseline.md`).

4. **Key storage in `esp-storage`**, not IDF NVS C APIs and not `const` secrets in `src/`. Typical pattern:
   - Generate the key once on first boot (`esp_hal` RNG / TRNG).
   - Persist it with `esp-storage` (flash). Do not commit keys.
   - Production-targeted examples (`security-baseline.md`) must not leave the key in plaintext flash without encryption / eFuse notes in the README.

5. Do **not** generate:
   - mbedTLS hooks (`MBEDTLS_AES_ALT`, …)
   - `sdkconfig.defaults` flash-encryption / secure-boot fragments
   - `espsecure.py` / `espefuse.py` C-IDF script stubs as the primary workflow
   - Hardcoded credentials in source

6. Depend on `esp-bootloader-esp-idf` for `esp_app_desc!()` only, pinned from `rust-toolchain.md`. This skill does not write OTA slots, burn eFuses, or enroll Secure Boot (`security-baseline.md`).

7. Document the AES mode, where the key lives, and that a plaintext-flash demo is not production key storage. Enable `esp-hal` `unstable` and say so. Do not add an eFuse enrollment procedure.
