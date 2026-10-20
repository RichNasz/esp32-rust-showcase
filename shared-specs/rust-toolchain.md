# Rust Toolchain and Crate Pins

**Classification:** Human-authored shared spec
**Applies to:** All examples in this repository

Looked up on crates.io and GitHub on **2026-09-08**, with the radio pin rechecked on **2026-10-20**. Generators must copy these pins; do not float to "latest" at generation time.

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
| `esp-hal` | **=1.2.1** | crates.io. Feature `esp32c6`. Radio / sleep need feature `unstable`. Exact `=` so Cargo does not select 1.2.2. Radio and non-radio examples both use this crates.io pin. |
| `esp-rtos` | **=0.4.0** | crates.io. Embassy integration via feature `embassy`. Radio examples also enable feature `esp-radio`. Same pin for radio and non-radio. |
| `esp-radio` | **=1.0.0-beta.1** | crates.io, published 2026-09-16. Depends on `esp-hal ~1.2.0`, so it resolves with `esp-hal` 1.2.1. **Do not** use `1.0.0-beta.0` (depends on `esp-hal ~1.1.0`). **Do not** use `1.0.0-beta.2` (yanked 2026-09-17). Unstable beta. See [Radio pin](#radio-pin-esp-radio). |
| `esp-println` | **0.18.0** | Use with the `log` crate (`log-04` feature). |
| `esp-backtrace` | **0.20.0** | Panic handler + println. |
| `esp-alloc` | **=0.11.0** | crates.io. Heap only when the CodingSpec justifies it. Same pin for radio and non-radio. |
| `esp-bootloader-esp-idf` | **=0.6.0** | **Required on every example.** Emits the ESP-IDF-compatible application descriptor current `espflash` requires (`esp_app_desc!()`). Feature `esp32c6`. This crate is **not** ESP-IDF. crates.io for radio and non-radio. |
| `esp-storage` | **=0.10.0** | crates.io. Flash access when the example writes partitions. Feature `esp32c6`. |
| `esp-lp-hal` | **git** tag **`esp-hal-v1.2.1`** (in-tree **0.3.0**) | LP-core firmware only. Feature `esp32c6`. Nested `lp-core/` crate. Does not pull the HP radio crates onto git. |
| `embedded-tls` | **0.19.0** | `secure-ota-https` only. TLS 1.3 client, `no_std` (disable the default `std` feature). No allocator required by the crate. Not a TLS 1.2 stack. Looked up on crates.io 2026-10-20. |
| `openthread` | **0.4.0** | `thread-udp-pair` only. Rust bindings over the OpenThread C library. Default features are the prebuilt `matter` profile (MTD). Feature `esp-radio`. Feature `ftd` is the leader only and is not the prebuilt profile. |
| `tinyrlibc` | **0.5** | `thread-udp-pair` and `matter-wifi-onoff`. Provides the C `str*` symbols the prebuilt OpenThread library calls. Disable default features. Thread enables `utoa` and `strtoul`. Matter also enables `memchr`. |
| `rs-matter-embassy` | git rev **`2a806b0b1b521293b34a773618499e87402c843c`** | `matter-wifi-onoff` only. `https://github.com/sysgrok/rs-matter-embassy` (2026-09-28, "Update to esp-radio beta 1"). crates.io `0.0.0` is an empty placeholder. Disable default features. Enable `esp`, `log`, `trouble`, `embassy-net`, `rustcrypto`. |

Related (not a substitute for the table above): `embassy-executor` **0.10**, `embassy-time` **0.5**, `embassy-sync` **0.8** (as used by the `esp-hal` 1.2.1 examples). Time is `embassy_time`. Embassy tasks via `esp-rtos` feature `embassy`.

---

## Radio pin (`esp-radio`)

Rechecked 2026-10-20:

- crates.io `esp-radio` `1.0.0-beta.1` (2026-09-16) depends on `esp-hal ~1.2.0` and is the radio pin.
- crates.io `esp-radio` `1.0.0-beta.0` (2026-06-03) depends on `esp-hal ~1.1.0` and must not be used next to `esp-hal` 1.2.1.
- crates.io `esp-radio` `1.0.0-beta.2` was yanked on 2026-09-17. Do not select it.
- Git tag `esp-hal-v1.2.1` still vendors `esp-radio` as version string `1.0.0-beta.0`. Do not git-pin that tag for HP radio crates. The nested LP crate still git-pins `esp-lp-hal` at that tag.

Radio examples use **crates.io exact versions** for every monorepo crate they touch: `esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1`, `esp-rtos` `=0.4.0`, `esp-bootloader-esp-idf` `=0.6.0`, `esp-alloc` `=0.11.0` when a heap is justified, and `esp-storage` `=0.10.0` when the example writes flash. Do not mix any of those with a git `esp-hal`. IEEE 802.15.4 does not enable `wifi`. BLE coexistence with 802.15.4 is part of beta.1. Wi-Fi coexistence with 802.15.4 is not.

```toml
esp-hal = { version = "=1.2.1", features = ["esp32c6", "unstable"] }
esp-radio = { version = "=1.0.0-beta.1", features = ["esp32c6", "unstable"] }
# plus wifi and/or ble and/or esp-now and/or ieee802154 and/or coex on esp-radio, matching the example
esp-rtos = { version = "=0.4.0", features = ["esp32c6", "embassy", "esp-radio"] }
esp-bootloader-esp-idf = { version = "=0.6.0", features = ["esp32c6"] }
# when the CodingSpec justifies a heap (typical for radio blobs / TrouBLE):
esp-alloc = { version = "=0.11.0" }
```

The radio crate remains an **unstable beta** (`1.0.0-beta.1`). Document that in every radio example README. Requires `esp-hal` `unstable`.

**Non-radio** examples keep crates.io `esp-hal` `=1.2.1` and `esp-rtos` `=0.4.0` (and crates.io `esp-alloc` `=0.11.0` if a heap is justified).

### Thread and Matter pins

`thread-udp-pair` adds crates.io `openthread` 0.4.0 (features `esp-radio`, `isupper`, `log`, and `udp`) and `tinyrlibc` 0.5. The child role keeps the default prebuilt MTD archive, so that build is cargo-only. The leader role enables `openthread` feature `ftd`. That feature is not the prebuilt profile, so `openthread-sys` compiles OpenThread on the host and needs Clang, CMake, and Ninja. The example `build.rs` does not invoke CMake itself. Wi-Fi stays off.

`matter-wifi-onoff` adds git `rs-matter-embassy` at rev `2a806b0b1b521293b34a773618499e87402c843c` and `tinyrlibc` 0.5. Enable `esp-radio` features `wifi`, `ble`, and `coex`. Do not enable `ieee802154`. This example is Matter over Wi-Fi. Matter over Thread is not a catalog example.

---

## Version Locking Rationale

- `esp-hal` 1.2.x is the current stable HAL line with MSRV 1.95.0 and a first-class `esp32c6` feature, including LP-core support.
- Radio and several RTC / sleep APIs are gated on `unstable`. v1 documents that honestly rather than pretending those APIs are frozen.
- `esp-rtos` 0.4.0 is the crates.io Embassy / scheduler crate for radio and non-radio examples.
- `esp-radio` 1.0.0-beta.1 on crates.io depends on `esp-hal ~1.2.0`. Radio examples use that crates.io pin with exact `esp-hal` 1.2.1. The nested LP crate still git-pins `esp-lp-hal`.
- rustup **stable** (not the `esp` channel / `espup`) is sufficient for RISC-V ESP32-C6 `no_std` as of this HAL line. The Thread leader role also needs Clang, CMake, and Ninja because `openthread` feature `ftd` rebuilds the C library.

---

## Upgrading

When the project moves to a new `esp-hal` minor or a new MSRV:

1. Update the tables in this file from crates.io **and** (for radio) from the matching `esp-hal-v*` git tag.
2. Rebuild every example (`cargo build --release` and `cargo clippy -- -D warnings` on the C6 target).
3. Record breaking changes in `AIGenLessonsLearned.md`.

Do not upgrade mid-example-development without running the full validation suite first.
