<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Hardware AES Secure Storage

> On-chip AES-256 via `esp-hal` plus flash-backed ciphertext in `esp-storage`.

This example does **not** use ESP-IDF mbedTLS, Kconfig, or sdkconfig.

## Overview

Security foundation example. Demonstrates the ESP32-C6 AES hardware engine through `esp-hal`. Encrypts a plaintext payload, stores the ciphertext in flash-backed storage, retrieves and decrypts it, then verifies the round-trip. Exposes a clean secure-write / secure-read API as a template for production secure storage. A throughput benchmark logs hardware AES speed.

LED: one blink on pass; a distinct failure pattern on self-test failure. AES key is generated once on first boot and stored via `esp-storage`. This example demonstrates the hardware crypto API, not full key lifecycle management — production systems should add stronger key protection as noted in `shared-specs/security-baseline.md`.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6

ESP-IDF is **not** required.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only.

## Per-Board Behavior

| Board | LED feedback |
| --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Color: green = success, red = failure, off = idle (WS2812) |
| Seeed XIAO ESP32-C6 | On / off, active LOW |

Flash DevKitC via the **left** USB-C UART-C port. XIAO uses native USB Serial/JTAG.

## Build & Flash

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/hardware-aes-secure-storage/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

```
aes-storage: key generated, saved to storage
aes-storage: Self-test: PASS
aes-storage: hardware AES throughput … MB/s
```

After reset:

```
aes-storage: key loaded from storage
aes-storage: Self-test: PASS
```

## Key Concepts

- **`esp-hal` AES** — hardware accelerator, not a software-only crate
- **`esp-storage`** — flash-backed key and ciphertext; no NVS Kconfig
- **Round-trip self-test** on boot; LED pass/fail
- **Throughput log** — RISC-V C6 target; FunctionalSpec historically cited ≥ 2 MB/s on RISC-V
- **Security model** — example key is stored as generated; not a full HSM

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build**

   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   ```

   Pass: exit 0, zero warnings.

2. **T-A2 — XIAO feature build**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   ```

   Pass: exit 0.

3. **T-A3 — Self-test via serial (Wokwi)** — additional tool, justified in TestSpec.md: AES-CBC round-trip is verifiable from serial output.

   ```sh
   wokwi-cli --timeout 15000 --expect "Self-test: PASS" .
   ```

   Pass: `Self-test: PASS` appears within 15 seconds when Wokwi files are generated.

### Manual Tests — hardware required

**T-M1 — Self-test on real hardware**

Why manual: confirms the hardware AES accelerator is invoked on C6 silicon; simulators may use a software path.

1. `cargo run --release` on ESP32-C6-DevKitC-1-N8.
2. Confirm serial shows `Self-test: PASS` and the LED indicates pass.
3. Confirm a throughput figure is logged.

Pass: PASS logged, pass LED, throughput line present.

---

**T-M2 — Key persistence across resets**

Why manual: flash persistence requires physical write/read cycles.

1. First boot: key generated and saved.
2. Press RST.
3. Confirm key loaded from storage; self-test still passes.

Pass: key loads after reset; self-test passes.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
