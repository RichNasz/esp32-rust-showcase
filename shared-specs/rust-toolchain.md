# Rust Toolchain and Crate Pins

**Classification:** Human-authored shared spec
**Applies to:** All examples in this repository

Looked up on crates.io and GitHub on **2026-09-08**. Generators must copy these pins; do not float to "latest" at generation time.

---

## Toolchain

| Constraint | Value |
|---|---|
| **Channel** | `stable` (rustup). **Not** channel `esp`. |
| **MSRV** | 1.95.0 |
| **Edition** | 2024 |
| **Target** | `riscv32imac-unknown-none-elf` |
| **Components** | `rust-src`, `clippy`, `rustfmt` |
| **Flash / monitor** | `espflash` / `cargo-espflash` (`runner = "espflash flash --monitor"`) |
| **ESP-IDF** | **Not used** in v1 |
| **espup** | **Not used** in v1 |

Firmware is `no_std` rust-on-metal via `esp-hal`. Do not install ESP-IDF, do not invoke `idf.py`, and do not generate `sdkconfig`.

### Why ESP-IDF / `espup` are unused (v1)

Verified against Espressif / esp-rs docs (2026-09):

- **`esp-hal` does not use ESP-IDF.** It is a bare-metal (`no_std`) HAL. The other Rust path (`esp-idf-sys` / `esp-idf-hal` / `esp-idf-svc`) wraps the C SDK and *would* need ESP-IDF; this repository does not take that path. Espressif’s official, vendor-backed SDK for Rust is `esp-hal`.
- **ESP32-C6 is RISC-V.** Upstream rustup **stable** already ships `riscv32imac-unknown-none-elf` (Tier 2). The *Rust on ESP Book* says: if you are not targeting ESP32 / ESP32-S2 / ESP32-S3 (Xtensa), skip `espup`. `espup` and the `esp` rustup channel exist for the Xtensa LLVM/Rust fork.
- **`esp-bootloader-esp-idf` is image metadata only.** It emits `esp_app_desc!()` so `espflash` and the second-stage bootloader accept the ELF. It is not an ESP-IDF checkout.

Generated reader-facing explanation: `docs/guides/why-esp-idf-is-not-required.md` (driven by `specs/documentation-generation-spec.md`).

---

## Supported Target SoCs

| SoC | Status | Notes |
|---|---|---|
| ESP32-C6 | **Required** | RISC-V HP + LP cores; chip feature `esp32c6`; target `riscv32imac-unknown-none-elf`. RTC/LP GPIO **0–7 only**. LP-core (`esp-lp-hal`). No capacitive touch. |
| ESP32 | Not in v1 | Xtensa — needs `espup` / `esp` channel. Different RTC map, no C6 LP-core. Do not port a C6 spec by renaming the chip feature. |
| ESP32-S3 | Not in v1 | Xtensa — needs `espup`. USB OTG CDC, capacitive touch, ULP-RISC-V — **not** C6 APIs. |
| ESP32-S2 | Not in v1 | Xtensa — needs `espup`. No BLE. Not a C6 drop-in. |
| ESP32-C3 | Not in v1 | RISC-V (`riscv32imc-unknown-none-elf`, not `imac`). No LP-core. Single-core. |
| ESP32-C5 | Not in v1 | RISC-V; dual-band Wi-Fi. No v1 chip feature. Do not add a board-spec. |
| ESP32-H2 | Not in v1 | RISC-V; 802.15.4 / BLE, **no Wi-Fi**. Cannot run Wi-Fi / ESP-NOW examples. |

Hardware bar: **ESP32-C6-DevKitC-1-N8 required**; **XIAO ESP32-C6 also supported**. Other SoCs are not firmware targets and have no board-specs. Expanding the bar means a new chip feature, a row in this table, new board-specs, and new example specs — not `idf.py set-target`.

Chip (SoC) ≠ board. `esp32c6` selects the HAL. `board-devkitc-c6` / `board-xiao-c6` select pins and USB. Each FunctionalSpec names **one** SoC.

Each example's `FunctionalSpec.md` must declare C6 (and which boards). Generated crates compile only for `esp32c6`.

---

## Pinned Crates

| Crate | Version | Notes |
|---|---|---|
| `esp-hal` | **1.2.1** | crates.io `default_version` on 2026-09-08 (plan listed 1.2.0; 1.2.1 is a 2026-09-08 patch, MSRV still 1.95.0). Git tag `esp-hal-v1.2.1` (rev `3da4493d8fda1b1dc066b09b8d1db0c66f48906e`). Feature `esp32c6`. Radio / sleep need feature `unstable`. **Non-radio** examples use crates.io. **Radio** examples git-pin `esp-hal` from that tag with the rest of the monorepo stack (see [Radio pin](#radio-pin-esp-radio)). |
| `esp-rtos` | **0.4.0** (non-radio, crates.io) / **git** tag **`esp-hal-v1.2.1`** (radio) | In-tree version at the tag is still `0.4.0` and path-depends on `esp-hal ~1.2.0`. Embassy integration via feature `embassy`. Radio examples also enable feature `esp-radio`. **Do not** mix crates.io `esp-rtos = "0.4.0"` with git `esp-hal`. |
| `esp-radio` | **git** tag **`esp-hal-v1.2.1`** | **No crates.io release matches `esp-hal` 1.2.x.** crates.io `esp-radio` `1.0.0-beta.0` (2026-06-03) depends on `esp-hal ~1.1.0` and **cannot** resolve with `esp-hal` 1.2.1. Do **not** write `esp-radio = "1.0.0-beta.0"` next to `esp-hal = "1.2.1"`. At git tag `esp-hal-v1.2.1` the in-tree crate is still versioned `1.0.0-beta.0` but depends on `esp-hal ~1.2.0` via path — that tree is the 1.2.1-compatible pin. See [Radio pin](#radio-pin-esp-radio). |
| `esp-println` | **0.18.0** | Use with the `log` crate (`log-04` feature). |
| `esp-backtrace` | **0.20.0** | Panic handler + println. |
| `esp-alloc` | **0.11.0** (non-radio, crates.io) / **git** tag **`esp-hal-v1.2.1`** (radio, when heap is justified) | Heap only when the CodingSpec justifies it. Radio examples that need a heap git-pin `esp-alloc` from the same tag (in-tree `0.11.0`). |
| `esp-bootloader-esp-idf` | **0.6.0** (non-radio, crates.io) / **git** tag **`esp-hal-v1.2.1`** (radio) | **Required on every example.** Emits the ESP-IDF-compatible application descriptor current `espflash` requires (`esp_app_desc!()`). Feature `esp32c6`. This crate is **not** ESP-IDF. Radio examples git-pin it with the rest of the monorepo stack so Cargo does not build two `esp-hal` packages. |
| `esp-storage` | **0.10.0** (crates.io, non-radio) / **git** tag **`esp-hal-v1.2.1`** (radio, in-tree `0.10.0`) | Flash access. Radio examples that write partitions (OTA) git-pin it with the rest of the monorepo stack. Mixing crates.io `esp-storage` with git `esp-hal` builds two HALs. |
| `esp-lp-hal` | **git** tag **`esp-hal-v1.2.1`** (in-tree **0.3.0**) | LP-core firmware only. Feature `esp32c6`. Nested `lp-core/` crate. Does not force the HP crate off crates.io when that crate has no radio. |
| `embedded-tls` | **0.19.0** | `secure-ota-https` only. TLS 1.3 client, `no_std` (disable the default `std` feature). No allocator required by the crate. Not a TLS 1.2 stack. Looked up on crates.io 2026-10-20. |

Related (not a substitute for the table above): `embassy-executor` **0.10**, `embassy-time` **0.5**, `embassy-sync` **0.8** (as used by the `esp-hal` 1.2.1 examples). Time is `embassy_time`. Embassy tasks via `esp-rtos` feature `embassy`.

---

## Radio pin (`esp-radio`)

Looked up 2026-09-08:

- crates.io `esp-radio` versions: `1.0.0-beta.0` (newest, 2026-06-03), `0.18.0` (`max_stable`). **None** of these declare `esp-hal ~1.2`.
- crates.io `esp-radio 1.0.0-beta.0` → `esp-hal = "~1.1.0"` → Cargo will not select `esp-hal` 1.2.1.
- Git tags: `esp-hal-v1.2.1` (`3da4493d8fda1b1dc066b09b8d1db0c66f48906e`), `esp-hal-v1.2.0` (`dbd951f78c1276461ea3050c2ba71f552a3508ef`). There is **no** crates.io `esp-radio` tag/release for either.
- At `esp-hal-v1.2.1`, `esp-radio/Cargo.toml` is still `version = "1.0.0-beta.0"` and has `esp-hal = { version = "~1.2.0", path = "../esp-hal" }`. `esp-rtos` is in-tree `0.4.0` with the same path dep. `esp-alloc` is in-tree `0.11.0`. Those path deps are conceptually compatible with HAL 1.2.1.

Radio examples **must** git-pin **all** monorepo crates they use from `esp-rs/esp-hal` at tag **`esp-hal-v1.2.1`** (rev `3da4493d8fda1b1dc066b09b8d1db0c66f48906e`): at least `esp-hal`, `esp-radio`, `esp-rtos`, `esp-bootloader-esp-idf`, and `esp-alloc` when heap is justified. OTA examples also git-pin `esp-storage` from that tag. Mixing crates.io `esp-rtos = "0.4.0"` (or crates.io `esp-alloc` / `esp-bootloader-esp-idf` / `esp-storage`) with git `esp-hal` / `esp-radio` produces two `esp-hal` packages. This is still HAL 1.2.1, not a downgrade.

```toml
esp-hal = { git = "https://github.com/esp-rs/esp-hal", tag = "esp-hal-v1.2.1", features = ["esp32c6", "unstable"] }
esp-radio = { git = "https://github.com/esp-rs/esp-hal", tag = "esp-hal-v1.2.1", features = ["esp32c6", "unstable"] }
# plus wifi and/or ble and/or esp-now and/or ieee802154 on esp-radio, matching the example
esp-rtos = { git = "https://github.com/esp-rs/esp-hal", tag = "esp-hal-v1.2.1", features = ["esp32c6", "embassy", "esp-radio"] }
esp-bootloader-esp-idf = { git = "https://github.com/esp-rs/esp-hal", tag = "esp-hal-v1.2.1", features = ["esp32c6"] }
# when the CodingSpec justifies a heap (typical for radio blobs / TrouBLE):
esp-alloc = { git = "https://github.com/esp-rs/esp-hal", tag = "esp-hal-v1.2.1" }
```

The radio crate remains an **unstable beta** (in-tree version string `1.0.0-beta.0`). Document that in every radio example README. Requires `esp-hal` `unstable`.

**Non-radio** examples keep crates.io `esp-hal` 1.2.1 + `esp-rtos` 0.4.0 (and crates.io `esp-alloc` 0.11.0 if a heap is justified).

If a later crates.io `esp-radio` appears that depends on `esp-hal ~1.2`, switch the pin table to that crates.io version and drop the git source.

---

## Version Locking Rationale

- `esp-hal` 1.2.x is the current stable HAL line with MSRV 1.95.0 and a first-class `esp32c6` feature, including LP-core support.
- Radio and several RTC / sleep APIs are gated on `unstable`. v1 documents that honestly rather than pretending those APIs are frozen.
- `esp-rtos` 0.4.0 is the crates.io Embassy / scheduler crate for **non-radio** examples. Radio examples take `esp-rtos` from the same git tag as `esp-hal` / `esp-radio`.
- `esp-radio` matching 1.2.x is **not on crates.io**; it lives in the `esp-hal` monorepo at tag `esp-hal-v1.2.1`. Git-pin the whole monorepo stack used by radio examples from that tag.
- rustup **stable** (not the `esp` channel / `espup`) is sufficient for RISC-V ESP32-C6 `no_std` as of this HAL line.

---

## Upgrading

When the project moves to a new `esp-hal` minor or a new MSRV:

1. Update the tables in this file from crates.io **and** (for radio) from the matching `esp-hal-v*` git tag.
2. Rebuild every example (`cargo build --release` and `cargo clippy -- -D warnings` on the C6 target).
3. Record breaking changes in `AIGenLessonsLearned.md`.

Do not upgrade mid-example-development without running the full validation suite first.
