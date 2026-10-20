# Security Baseline

**Classification:** Human-authored shared spec
**Applies to:** All examples. v1 examples are not production-targeted.

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

These apply to every example.

| Requirement | Implementation |
|---|---|
| No hardcoded credentials | Wi-Fi PSK, API keys, and tokens must not appear in committed source. Provision via gitignored config, compile-time env, or `esp-storage` / NVS-like flash — never `const` secrets in `src/` |
| No log leakage of secrets | Credentials must never be printed via `log::*`, `esp_println`, or `defmt` at any level |
| Input validation at system boundaries | All data received over UART, BLE, or network must be length-checked before use |
| No `unwrap()` on fallible HAL/radio in production paths | See `coding-conventions.md`; init may panic if the process cannot continue |

ESP-IDF Kconfig symbols (`CONFIG_FREERTOS_CHECK_STACKOVERFLOW_*`, `CONFIG_ESP_TASK_WDT_EN`, `CONFIG_SECURE_*`) **do not apply**. There is no sdkconfig.

---

## Out of scope for v1

No example sets `Production Target: yes`. Secure Boot eFuse enrollment and flash-encryption enablement are not part of this repository. Do not add them as examples, and do not emit `CONFIG_SECURE_*` or any sdkconfig.

`esp-bootloader-esp-idf` writes the application descriptor `espflash` requires. It does not enroll Secure Boot.

`secure-ota-https` downloads over HTTPS, checks a CRC, and activates the inactive OTA slot. It does not verify a signature. Stock `espflash` does not auto-rollback.

### TLS when an example speaks HTTPS

- Verify the server certificate. Do not disable verification to make a demo connect.
- `secure-ota-https` uses TLS 1.3 via `embedded-tls` (one embedded CA).
- Heap (`esp-alloc`) for TLS buffers is allowed only when the CodingSpec justifies it.

---

## Credentials Management

| Credential Type | Storage | Notes |
|---|---|---|
| Wi-Fi PSK | Encrypted flash / gitignored provision file | Never in committed `src/` or Cargo features |
| API keys / tokens | Encrypted flash namespace | Injected at provisioning |
| TLS private key | Dedicated storage or hardware-backed key when available | Private key never logged |
| OTA image check | CRC before the inactive slot is activated (`secure-ota-https`) | Signature checks and eFuse enrollment are out of scope |

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

Before generating an example, verify:

- [ ] Credentials stored outside committed source.
- [ ] TLS certificate verification enabled for outbound HTTPS.
- [ ] OTA, when present, uses HTTPS and a CRC check before slot activation.
- [ ] No debug logs expose credentials.
- [ ] BLE / network input is length-checked.
- [ ] No `unwrap()` on fallible HAL/radio on paths that must keep running.
