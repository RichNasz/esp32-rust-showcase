# Security Baseline

**Classification:** Human-authored shared spec
**Applies to:** All examples; mandatory for any example marked `production-targeted` in its FunctionalSpec.md

---

## Threat Model Scope

This baseline addresses the threat model relevant to ESP32-C6 rust-on-metal firmware examples in this showcase:

- **Firmware confidentiality** — prevent extraction of proprietary algorithms or credentials stored in flash.
- **Firmware integrity** — prevent unauthorized firmware modification (physical or OTA).
- **Credential protection** — prevent extraction of Wi-Fi PSKs, API keys, TLS private keys stored in flash / NVS-like storage.
- **Communication security** — prevent eavesdropping or tampering of OTA updates and cloud connections.

Out of scope for this baseline: physical decapping attacks, RF-layer attacks, denial-of-service, cloud backend security.

---

## Minimum Requirements (All Examples)

These apply regardless of whether an example is production-targeted.

| Requirement | Implementation |
|---|---|
| No hardcoded credentials | Wi-Fi PSK, API keys, and tokens must not appear in committed source. Provision via gitignored config, compile-time env, or `esp-storage` / NVS-like flash — never `const` secrets in `src/` |
| No log leakage of secrets | Credentials must never be printed via `log::*`, `esp_println`, or `defmt` at any level |
| Input validation at system boundaries | All data received over UART, BLE, or network must be length-checked before use |
| No `unwrap()` on fallible HAL/radio in production paths | See `coding-conventions.md`; init may panic if the process cannot continue |

ESP-IDF Kconfig symbols (`CONFIG_FREERTOS_CHECK_STACKOVERFLOW_*`, `CONFIG_ESP_TASK_WDT_EN`, `CONFIG_SECURE_*`) **do not apply**. There is no sdkconfig.

---

## Production-Targeted Requirements

Examples with `Production Target: yes` in their FunctionalSpec.md must also satisfy:

### Flash encryption and Secure Boot (eFuse, not Kconfig)

IDF `CONFIG_SECURE_FLASH_ENC_ENABLED` / `CONFIG_SECURE_BOOT*` Kconfig **does not apply**.

- Use Espressif eFuse-based flash encryption and Secure Boot v2 as documented for ESP32-C6.
- Use `esp-bootloader-esp-idf` for IDF-compatible application descriptors and partition metadata so a standard ESP-IDF bootloader can verify and select the image. This crate is **not** ESP-IDF itself.
- Encryption and signing keys are generated or burned on-device / offline. Never commit private keys or eFuse key material.
- Once Secure Boot is enabled in production, it cannot be disabled without reflashing eFuses (irreversible).

### Storage encryption

- Production-targeted examples that persist credentials must encrypt that storage (flash encryption + encrypted NVS-like partition, or equivalent `esp-storage` usage).
- Encryption keys live in eFuse or a dedicated key partition — never in source.

### TLS/HTTPS Requirements

- All outbound HTTP connections must use HTTPS with server certificate verification enabled.
- Do not disable certificate verification to "make it work".
- Minimum TLS version: TLS 1.2.
- Certificate expiry must be handled gracefully — log and alert rather than silently failing.
- Heap (`esp-alloc`) for TLS buffers is allowed only when the CodingSpec justifies it.

### OTA Update Security

- OTA updates must be fetched over HTTPS with certificate verification.
- Verify the incoming firmware signature before committing the new image (Secure Boot app signing / `esp-bootloader-esp-idf` image metadata).
- Keep a rollback slot. Mark the new image valid only after confirming the device can still connect.

---

## Credentials Management

| Credential Type | Storage | Notes |
|---|---|---|
| Wi-Fi PSK | Encrypted flash / gitignored provision file | Never in committed `src/` or Cargo features |
| API keys / tokens | Encrypted flash namespace | Injected at provisioning |
| TLS private key | Dedicated storage or hardware-backed key when available | Private key never logged |
| OTA signing public key | Embedded in the signed image metadata | Part of the signed binary |

---

## Common Vulnerabilities to Avoid

The full-project-generator must not emit code that exhibits these patterns:

| Vulnerability | Pattern to Avoid |
|---|---|
| Buffer overflow | Unbounded copies from BLE / network / UART into fixed buffers |
| Format string | Passing untrusted input as a `log!` / format string |
| Integer overflow | Unchecked arithmetic on lengths before allocation or indexing |
| Unwrap of fallible I/O | `unwrap()` / `expect()` on HAL, radio, TLS, or OTA in production paths |
| Hardcoded IV/nonce | Using a fixed IV with AES-CBC — always use a random IV per message |
| Secret logging | Printing PSK, tokens, or private keys |

---

## Security Review Checklist

Before marking an example as production-ready, verify:

- [ ] Flash encryption / Secure Boot plan uses eFuse + `esp-bootloader-esp-idf`, not IDF Kconfig.
- [ ] Signing keys stored offline; never committed.
- [ ] Credentials stored outside committed source.
- [ ] TLS certificate verification enabled for all outbound connections.
- [ ] OTA update path uses HTTPS and signature verification.
- [ ] No debug logs expose credentials.
- [ ] BLE / network input is length-checked.
- [ ] No `unwrap()` on fallible HAL/radio in production paths.
