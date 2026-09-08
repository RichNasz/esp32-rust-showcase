<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Secure OTA via HTTPS

> Dual-bank OTA over HTTPS with TLS certificate validation and bootloader rollback.

This example enables `esp-hal` feature **`unstable`** (Wi-Fi via `esp-radio`). **`esp-radio` is an unstable beta** (git-pinned with the `esp-hal` monorepo stack at tag `esp-hal-v1.2.1`). Those APIs may change with `esp-hal` minor releases.

Partition tables are **CSV for espflash** (`partitions.csv` / `espflash.toml`) — there is no sdkconfig `CONFIG_PARTITION_TABLE_*`. See `shared-specs/partition-table-profiles.md`.

## Overview

Minimal visual demonstration of ESP32-C6 OTA using dual partitions.

The device maintains two firmware images in flash. A button press triggers an HTTPS download from a fixed server URL. The new binary is written to the **inactive** OTA partition. On success the device reboots into the newly written image. A status LED indicates activity during transfer and a distinct pattern on failure.

On first boot after OTA the new image must mark itself valid; otherwise rollback occurs. The server CA is embedded at build time. Wi-Fi STA, WPA2-Personal, HTTPS only.

DevKitC-C6 has 8 MB flash (N8) — comfortable dual-OTA headroom. XIAO C6 has 4 MB — use a matching partition profile.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- Wi-Fi AP; HTTPS host serving the firmware image
- Optional external LEDs/button as described in the FunctionalSpec (onboard LED is used when no externals are wired)

ESP-IDF is **not** required.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6. Wi-Fi + user LED required.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only.

## Per-Board Behavior

| Board | LED feedback | Deep sleep floor |
| --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Color: green = success, red = failure, off = idle (WS2812) | ~7–15 µA |
| Seeed XIAO ESP32-C6 | On / off, active LOW | ~15 µA |

Flash size differs (8 MB N8 vs 4 MB XIAO) — partition CSV must match. Use DevKitC **left** USB-C for `espflash`.

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

Wi-Fi credentials and OTA URL are crate configuration (not sdkconfig / menuconfig).

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/secure-ota-https/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

```
ota: running ota_0
ota: Wi-Fi associated
ota: HTTPS GET … TLS ok
ota: writing ota_1
ota: reboot into new image
```

After a good image marks itself valid, the next boot stays on the new partition. An image that never marks valid rolls back.

## Key Concepts

- **Dual OTA partitions** — `partitions.csv` + espflash, not Kconfig
- **TLS CA embed** — invalid cert must refuse the download
- **Rollback** — new image must confirm health
- **`esp-radio` Wi-Fi** — unstable beta; git-pin the monorepo stack
- **Visual program flip** — FunctionalSpec uses left/right LEDs on a DevKitC header; onboard WS2812 can stand in for status

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build**

   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   ```

   Pass: exit 0, zero warnings.

2. **T-A2 — Partition table present**

   After generation, confirm `partitions.csv` (or `espflash.toml` pointing at it) lists `ota_0` and `ota_1`.

   ```sh
   grep -E "ota_0|ota_1" partitions.csv
   ```

   Pass: both OTA slots listed with sizes matching the chosen profile in `shared-specs/partition-table-profiles.md`.

3. **T-A3 — XIAO feature build**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   ```

   Pass: exit 0.

### Manual Tests — hardware required

**T-M1 — Full OTA download and flash**

Why manual: requires a live Wi-Fi network, HTTPS host, and target hardware.

1. Configure SSID/password and OTA URL in the generated crate config.
2. `cargo run --release`
3. Trigger OTA (button).
4. Confirm TLS success, write to inactive slot, reboot, opposite/new status indication.
5. Confirm the new image marks itself valid (no rollback).

Pass: visual/serial flip; no rollback on a healthy image.

---

**T-M2 — Invalid certificate rejected**

Why manual: needs a host whose cert does not match the embedded CA.

Pass: download aborted; error LED pattern; running slot unchanged.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
