# Rust Toolchain and Crate Pins

**Classification:** Human-authored shared spec
**Applies to:** All examples in this repository

Looked up on crates.io on **2026-09-08**. Generators must copy these pins; do not float to "latest" at generation time.

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

---

## Supported Target SoCs

| SoC | Status | Notes |
|---|---|---|
| ESP32-C6 | **Required** | RISC-V HP + LP cores; chip feature `esp32c6`; target `riscv32imac-unknown-none-elf` |
| ESP32 | Not in v1 | — |
| ESP32-S3 | Not in v1 | — |
| ESP32-S2 | Not in v1 | — |
| ESP32-C3 | Not in v1 | — |
| ESP32-C5 | Not in v1 | — |
| ESP32-H2 | Not in v1 | — |

Hardware bar: **ESP32-C6-DevKitC-1-N8 required**; **XIAO ESP32-C6 also supported**. Other chips from the C-language showcase are not in v1.

Each example's `FunctionalSpec.md` must declare C6 (and which boards). Generated crates compile only for `esp32c6`.

---

## Pinned Crates

| Crate | Version | Notes |
|---|---|---|
| `esp-hal` | **1.2.1** | crates.io `default_version` on 2026-09-08 (plan listed 1.2.0; 1.2.1 is a same-day patch, MSRV still 1.95.0). Feature `esp32c6`. Radio / sleep need feature `unstable`. |
| `esp-rtos` | **0.4.0** | Embassy integration via feature `embassy`. |
| `esp-radio` | **1.0.0-beta.0** | crates.io newest on 2026-09-08 (`default_version` is still 0.18.0). **Unstable beta** — pin it and document instability in every radio example README. Requires `esp-hal` `unstable`. |
| `esp-println` | **0.18.0** | Use with the `log` crate (`log-04` feature). |
| `esp-backtrace` | **0.20.0** | Panic handler + println. |
| `esp-alloc` | **0.11.0** | Heap only when the CodingSpec justifies it. |
| `esp-bootloader-esp-idf` | **0.6.0** | IDF-compatible image / partition metadata. This is **not** ESP-IDF. |
| `esp-storage` | **0.10.0** | Flash access. |

Related (not a substitute for the table above): `embassy_time` for delays; Embassy tasks via `esp-rtos`.

---

## Version Locking Rationale

- `esp-hal` 1.2.x is the current stable HAL line with MSRV 1.95.0 and a first-class `esp32c6` feature, including LP-core support.
- Radio and several RTC / sleep APIs are gated on `unstable`. v1 documents that honestly rather than pretending those APIs are frozen.
- `esp-radio` 1.0.0-beta.0 is the newest crates.io release compatible with this HAL generation. It is a beta: APIs may change; READMEs must say so.
- rustup **stable** (not the `esp` channel / `espup`) is sufficient for RISC-V ESP32-C6 `no_std` as of this HAL line.

---

## Upgrading

When the project moves to a new `esp-hal` minor or a new MSRV:

1. Update the tables in this file from crates.io.
2. Rebuild every example (`cargo build --release` and `cargo clippy -- -D warnings` on the C6 target).
3. Record breaking changes in `AIGenLessonsLearned.md`.

Do not upgrade mid-example-development without running the full validation suite first.
