<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# 07 — Secure Your Project

Edit the example FunctionalSpec so it states the observable crypto behavior, ask the agent to follow `esp32-hardware-crypto-configurer`, then regenerate the crate with `esp32-sdd-full-project-generator`. Phrases to type to the agent, not in the shell: `Activate esp32-hardware-crypto-configurer skill`, then `Activate esp32-sdd-full-project-generator`. A **spec** is Markdown a person writes. **Generate** means the agent writes the crate from that spec. **Regenerate** overwrites the previous crate output and leaves `specs/` in place. How specs work: [Using With AI Agents](using-with-ai-agents.md).

A **skill** is a Markdown workflow in `skills/<name>/SKILL.md`.

## What this does

- Hardware AES through `esp-hal`, not an ESP-IDF mbedTLS menu and not Kconfig.
- Keys through `esp-storage` (flash-backed), not a hard-coded constant in `main.rs`.
- Alignment with [`shared-specs/security-baseline.md`](../../shared-specs/security-baseline.md).

`hardware-aes-secure-storage` is a plaintext-key demo, not production key storage. It is **Firmware generated**. That label is not a hardware pass.

## What this does not do

Secure Boot, eFuse burn, and flash-encryption enrollment are out of scope. Do not spec `CONFIG_BOOTLOADER_APP_ROLLBACK_ENABLE`, `esp_https_ota`, or an eFuse key ceremony. There is no `sdkconfig`.

## OTA

`secure-ota-https` is **Firmware generated** as of 2026-10-20. The crate is in the tree. It downloads one application image over HTTPS into the inactive OTA slot, checks a CRC32, and reboots onto that slot only when the check matches. TLS is 1.3 (`embedded-tls` 0.19.0, one embedded CA). Partition profile `dual-ota` is `partitions.csv` / `espflash.toml`, not sdkconfig. Stock `espflash` does not auto-rollback: the bootloader shipped with current `espflash` will not revert a bad image by itself. A CRC failure must not activate the new slot. This is not Secure Boot and not flash-encryption enrollment. CRC32 is a transfer check, not a signature.

`esp-radio` is an unstable beta. Radio crates, including the Wi-Fi path this example uses, git-pin the `esp-hal` monorepo at tag `esp-hal-v1.2.1` ([`shared-specs/rust-toolchain.md`](../../shared-specs/rust-toolchain.md)). A host build is not a hardware result.
