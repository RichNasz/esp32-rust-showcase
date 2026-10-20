# Hardware AES Secure Storage (ESP32-C6)

## Overview

Security-foundation example for rust-on-metal ESP32-C6 (`no_std` `esp-hal`). Demonstrates
the on-chip AES-256 accelerator through `esp-hal` (not ESP-IDF mbedTLS) and flash-backed
blobs through `esp-storage` (not IDF NVS). Encrypts a plaintext payload with AES-256-CBC,
stores ciphertext plus a unique IV, retrieves and decrypts it, and verifies the round-trip.
A 64 KiB hardware-AES throughput figure is logged. Chip is ESP32-C6 only.

This example demonstrates the hardware crypto API and flash persistence, **not** production
key lifecycle management. The AES key is stored in plaintext flash. Production systems must
add flash encryption / eFuse key protection as noted in `shared-specs/security-baseline.md`.

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/main.rs` are present (generated 2026-09-08). The crate demonstrates the AES accelerator and plaintext key storage, not production key management. This line is not a hardware-test result. The catalog table in `specs/showcase-master-spec.md` is the source of truth across examples.

## Target Hardware

v1 chip: **ESP32-C6** only (`esp32c6`). Other SoCs are not in v1.

### Board Compatibility Checklist

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [x] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add non-C6 boards. Adafruit HUZZAH32, Seeed XIAO ESP32S3, Seeed XIAO ESP32-C5, and
YEJMKJ ESP32-S3 boards are **not in v1**.

RTC / LP GPIOs on C6 are **0–7 only**; GPIO 9 (XIAO boot button) is **not** RTC-capable.
This example does not sleep, so that constraint is informational only.

## Features

- Encrypt and decrypt byte payloads with **AES-256-CBC** on the ESP32-C6 AES hardware
  accelerator (`esp_hal::aes`, feature `unstable`). Key length is taken from the 32-byte
  key. Prefer the hardware engine over a pure-software AES crate.
- Persist encrypted blobs (unique IV prepended to ciphertext) with **`esp-storage`**
  `FlashStorage` in a dedicated flash data partition. Do **not** use IDF NVS C APIs,
  `nvs_flash_init`, or Kconfig NVS.
- Partition table: a `storage` data partition of 64 KiB immediately after a 1 MiB factory
  app (flash offset **0x110000**). One flash sector holds the AES key; the next sector
  holds the self-test blob. Offsets match `partitions.csv`; `espflash` is pointed at that
  table.
- Generate the AES-256 key once on first boot from the hardware RNG. Persist it. On later
  resets, load the existing key rather than regenerate. Never commit a key and never log
  key material, IVs used as secrets, or credentials.
- Each write uses a **fresh random IV**. IV reuse with the same key is forbidden.
- Pad plaintext to the AES block size (PKCS#7) so any length is valid.
- On every boot, run a round-trip self-test: encrypt a known string, store it, retrieve it,
  decrypt it, verify it matches. Log `Self-test: PASS` or `Self-test: FAIL`. Confirm the
  stored ciphertext is not human-readable (it differs from the plaintext).
- Run a throughput benchmark: encrypt 64 KiB with hardware AES-256-CBC and log MB/s.
  Target **≥ 2 MB/s** on RISC-V C6.
- Status LED after the self-test (three visible blinks, then dark):
  - **DevKitC-C6 (default, `board-devkitc-c6`)**: WS2812 RGB LED on GPIO 8, driven by the
    `esp-hal` RMT peripheral (not a GPIO level). Green (g=64) for pass, red (r=64) for fail.
    GRB byte order, 800 kHz NRZ. Pass: ~500 ms on/off; fail: ~100 ms on/off.
  - **XIAO ESP32-C6 (`board-xiao-c6`)**: simple user LED on GPIO 15, **active LOW**. Same
    3-blink timing; no RMT / WS2812 on this board.
- Pin numbers live in generated `src/board.rs`. Application code must not hardcode GPIOs.
- Board selection is a Cargo feature, not menuconfig. Default features select DevKitC-C6.
  XIAO: `cargo run --release --no-default-features --features board-xiao-c6`.
- Serial log at INFO (`log` + `esp-println`): key generated vs loaded, self-test result,
  throughput. Never log the AES key.
- After the LED indication, idle in an Embassy loop. Self-test failure must not abort
  (no panic loop on a test bench).

## Wakeup Strategy

Not applicable. This example runs continuously; it does not enter deep sleep.

## Connectivity

None. Non-radio crate pin: crates.io `esp-hal` 1.2.1 + `esp-rtos` 0.4.0.

## Security Requirements

Follow `shared-specs/security-baseline.md`. **Production Target: no.**

- Algorithm: AES-256-CBC via `esp_hal::aes` (hardware). Unique IV per blob.
- Key: generated on first boot (hardware RNG), stored in `esp-storage`. Not a `const` in
  `src/`. Not IDF NVS.
- Do not log key material at any log level.
- This example stores the key in **plaintext flash**. That is intentional for a hardware-API
  demo and is **not** production key protection. Production must use eFuse-backed flash
  encryption (not IDF `CONFIG_SECURE_*` Kconfig).
- No hardcoded credentials.

## Hardware Dependencies

- Board-spec: `board-specs/espressif/esp32-c6-devkitc-1-n8.md` (WS2812 RGB on GPIO 8;
  flash/monitor on the **left** USB-C UART-C port).
- Board-spec: `board-specs/seeed/xiao-esp32c6.md` (user LED GPIO 15, active LOW;
  USB Serial/JTAG console).
- No external components required.

The generator auto-invokes:

- `esp32-hardware-crypto-configurer` — AES-256-CBC + `esp-storage` key/blob persistence
- `esp32-ws2812-led-engineer` — DevKitC WS2812 RMT path

## Partition Table

Custom table so the ciphertext and key do not share the factory app image. Profile is a
single-app layout plus a dedicated `storage` data partition (not IDF NVS format):

| Name | Type | SubType | Offset | Size |
|---|---|---|---|---|
| nvs | data | nvs | 0x9000 | 0x6000 |
| phy_init | data | phy | 0xF000 | 0x1000 |
| factory | app | factory | 0x10000 | 1 MiB |
| storage | data | 0x40 | 0x110000 | 64 KiB |

Flash via `espflash` with `--partition-table partitions.csv`. Fits both 4 MB (XIAO) and
8 MB (DevKitC-N8) parts.

## Limitations

| This example is NOT | Why |
|---|---|
| Production key management | Key is plaintext in flash; no eFuse wrapping |
| Authenticated encryption | AES-CBC without HMAC / GCM; ciphertext is not integrity-protected |
| IDF NVS or mbedTLS | rust-on-metal `esp-hal` + `esp-storage` only |
| Flash encryption / Secure Boot enablement | Irreversible eFuse operations are out of scope |

Extension path: wrap the stored key with flash encryption / eFuse; add an HMAC or switch
to an AEAD; keep using hardware AES.

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf` (default DevKitC-C6 features).
- `cargo clippy --release -- -D warnings` is clean for the default features.
- `cargo build --release --no-default-features --features board-xiao-c6` and the matching clippy invocation are clean.
- **`cargo run --release`** flashes via `espflash` on DevKitC-C6 (left USB-C / UART-C). Serial shows key generated or loaded, `Self-test: PASS`, and a hardware AES throughput line. The WS2812 blinks green three times (slow) on pass.
- Optional XIAO: `cargo run --release --no-default-features --features board-xiao-c6`. GPIO 15 blinks three times on pass (active-LOW).
- Round-trip: decrypted plaintext matches the original; stored blob is not the plaintext.
- Hardware AES throughput **≥ 2 MB/s** on C6.
- No heap. Self-test failure logs FAIL, blinks the fail LED pattern, and idles (does not panic).
