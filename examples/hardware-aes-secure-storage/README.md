<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-full-project-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Hardware AES Secure Storage — ESP32-C6

> On-chip AES-256-CBC via `esp-hal` plus flash-backed ciphertext in `esp-storage`.

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/main.rs` are in this folder (generated 2026-09-08 from the specs). The build commands below apply to this crate. The firmware stores the AES key in plaintext flash on purpose. This is not a record of a hardware test on 2026-10-20.

This example enables `esp-hal` feature **`unstable`** because the AES accelerator, FLASH singleton, and RMT (WS2812) APIs are unstable on the 1.2.1 pin. Those APIs may change with `esp-hal` minor releases.

This example does **not** use ESP-IDF mbedTLS, Kconfig, sdkconfig, or IDF NVS.

## Overview

Security foundation example. Demonstrates the ESP32-C6 AES hardware engine through `esp-hal`. Encrypts a plaintext payload with AES-256-CBC, stores the unique IV plus ciphertext in a dedicated flash partition via `esp-storage`, retrieves and decrypts it, then verifies the round-trip. A 64 KiB throughput benchmark logs hardware AES speed in MB/s.

LED: three slow green blinks on pass; three fast red blinks on self-test failure (WS2812). AES key is generated once on first boot from the hardware RNG and stored via `esp-storage`. This example demonstrates the hardware crypto API, **not** full key lifecycle management — the key is stored in plaintext flash. Production systems should add flash encryption / eFuse protection as noted in `shared-specs/security-baseline.md`.

Pin numbers live in generated `src/board.rs`. Never hard-code GPIOs in `main.rs`.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6

ESP-IDF is **not** required. Do not install `espup` or the `esp` rustup channel.

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

The runner passes `--partition-table partitions.csv` so the dedicated `storage` partition (offset 0x110000) is programmed.

> **Never edit generated source files directly.** To change board configuration, update `specs/FunctionalSpec.md` or `specs/CodingSpec.md` and regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/hardware-aes-secure-storage/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

### First boot

```
hardware_aes_secure_storage: AES storage — WS2812 RGB LED, GPIO 8
hardware_aes_secure_storage: key generated, saved to storage
hardware_aes_secure_storage: ciphertext differs from plaintext
hardware_aes_secure_storage: Self-test: PASS
hardware_aes_secure_storage: hardware AES throughput 6.553 MB/s
hardware_aes_secure_storage: WS2812 RMT init on GPIO 8 (GRB order, max brightness 64/255)
```

(The throughput number is representative; C6 hardware AES should be ≥ 2 MB/s.)

### After reset

```
hardware_aes_secure_storage: key loaded from storage
hardware_aes_secure_storage: Self-test: PASS
```

The AES key is never printed.

### Seeed XIAO ESP32-C6 (active LOW, GPIO 15)

```
hardware_aes_secure_storage: AES storage — GPIO LED (active-LOW), GPIO 15
hardware_aes_secure_storage: key generated, saved to storage
hardware_aes_secure_storage: Self-test: PASS
hardware_aes_secure_storage: GPIO LED on GPIO 15 (GPIO LED (active-LOW))
```

## Key Concepts

- **Standalone Cargo crate** — `#![no_std]` `#![no_main]`; Embassy entry via `esp-rtos`; no workspace at the repo root
- **`esp-hal` AES** — hardware accelerator (feature `unstable`); AES-256-CBC; key length from the 32-byte key
- **`esp-storage`** — flash-backed key and ciphertext in a dedicated partition; no NVS Kconfig
- **Unique IV per write** — hardware RNG; IV stored with the blob
- **Round-trip self-test** on boot; LED pass/fail (three blinks, cap 64/255)
- **Throughput log** — 64 KiB encrypt; RISC-V C6 target ≥ 2 MB/s
- **Security model** — example key is stored as generated plaintext in flash; not a full HSM

## Limitations

| This example is NOT | Why |
|---|---|
| Production key management | Key is plaintext in flash; no eFuse wrapping |
| Authenticated encryption | AES-CBC without HMAC / GCM; ciphertext is not integrity-protected |
| IDF NVS or mbedTLS | rust-on-metal `esp-hal` + `esp-storage` only |
| Flash encryption / Secure Boot enablement | Irreversible eFuse operations are out of scope |

Extension path: wrap the stored key with flash encryption / eFuse; add an HMAC or switch to an AEAD; keep using hardware AES.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (DevKitC-C6, default features)**

   From `examples/hardware-aes-secure-storage/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings.

2. **T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings.

3. **T-A3 — Flash via espflash runner**

   ```sh
   cargo run --release
   ```

   Pass: image flashes; serial log shows `Self-test: PASS` and a throughput figure. Use the DevKitC **left** USB-C UART-C port.

### Manual Tests — hardware required

**T-M1 — Self-test and LED on real hardware (ESP32-C6-DevKitC-1-N8)**

Why manual: confirms the hardware AES accelerator is invoked on C6 silicon; simulators may use a software path.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8, USB-C on the **left** UART-C port.

1. `cargo run --release`
2. Confirm serial shows key generated or loaded, `Self-test: PASS`, ciphertext differs from plaintext.
3. Confirm a hardware AES throughput line of ≥ 2 MB/s.
4. Observe three slow green blinks on the WS2812, then dark.

Pass: PASS logged, throughput ≥ 2 MB/s, green 3-blink visible.

---

**T-M2 — Key persistence across resets**

Why manual: flash persistence requires physical write/read cycles.

1. First boot: key generated and saved.
2. Press RST.
3. Confirm key loaded from storage; self-test still passes.

Pass: key loads after reset; self-test passes.

---

**T-M3 — GPIO LED pass pattern (Seeed XIAO ESP32-C6)**

Why manual: active-LOW polarity and blink timing require human observation.

1. `cargo run --release --no-default-features --features board-xiao-c6`
2. Observe the blue user LED on GPIO 15: three slow blinks, then off.

Pass: correct polarity, three blinks, PASS in the log.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)
