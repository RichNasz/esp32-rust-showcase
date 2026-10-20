# HTTPS OTA Slot Flip (ESP32-C6)

## Overview

Rust-on-metal OTA example for ESP32-C6 (`no_std` `esp-hal`). The device joins Wi-Fi,
downloads one application image over HTTPS into the inactive OTA slot, checks a CRC,
points the IDF-compatible `otadata` partition at that slot, and reboots. The same
firmware reports which slot it booted from, so one image flipped between `ota_0` and
`ota_1` is the whole demo. Chip is ESP32-C6 only.

This is a transfer-and-slot-flip demo. It does not enroll Secure Boot, burn eFuses, or
turn on flash encryption. The stock `espflash` bootloader does not auto-rollback.

## Implementation status

**Specs only** as of 2026-10-20. These Functional, Coding, and Test specs are the current design (HTTPS slot flip, CRC32, no Secure Boot). There is no `Cargo.toml` and no `src/` until `esp32-sdd-full-project-generator` runs. The catalog table in `specs/showcase-master-spec.md` is the source of truth across examples.

## Target Hardware

v1 chip: **ESP32-C6** only (`esp32c6`). Other SoCs are not in v1.

| Item | v1 value |
| --- | --- |
| **SoC** | ESP32-C6 only |
| **Chip Cargo feature** | `esp32c6` |
| **rustc target** | `riscv32imac-unknown-none-elf` |

### Board Compatibility Checklist

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [x] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add non-C6 boards. Adafruit HUZZAH32, Seeed XIAO ESP32S3, Seeed XIAO ESP32-C5, and
YEJMKJ ESP32-S3 boards are **not in v1**. There are no external LEDs and no external button.

RTC / LP GPIOs on C6 are **0–7 only**. This example does not sleep, so that limit does not
constrain the trigger button. The trigger is a press **after** boot.

## Features

- Boot, log the active app slot (`ota_0` or `ota_1`, including its flash offset), and wait
  for the board button. Do not start a download at boot.
- **DevKitC-C6:** trigger is the BOOT button, GPIO 0, active LOW, internal pull-up. A level
  that is already low at the start of the wait is not a press. The firmware arms only after
  it has seen the line released, then a new press. GPIO 0 held low across reset is the
  serial-download strap, not an OTA request.
- **XIAO ESP32-C6:** trigger is the BOOT button, GPIO 9, active LOW. Same release-then-press
  rule. GPIO 9 is not an RTC pin; this example never sleeps, so that is fine.
- On a press: associate to Wi-Fi (WPA2-Personal) with a bounded timeout of about 15 seconds.
  On timeout, show the error indication and return to the wait. Do not reboot.
- HTTPS GET of one URL. TLS 1.3, server certificate checked against a CA embedded at build
  time. A certificate that does not chain to that CA, a hostname mismatch, or a handshake
  failure aborts the transfer, does not write a new boot slot, and returns to the wait.
- The body is an integrity envelope, not a raw image:
  - 4-byte little-endian image length
  - 4-byte little-endian IEEE CRC32 of the image bytes that follow
  - the application image produced by `espflash save-image` for this chip and this
    partition table
- Stream the image into the **inactive** OTA slot. Do not buffer the whole image in RAM.
  If the byte count or the CRC does not match the prefix, erase nothing the bootloader
  will select: do **not** activate the slot. Return to the wait.
- On a match: activate that slot and reboot. The next boot logs the other slot.
- Status LED uses generated `src/board.rs` pins. No external LEDs.
  - **DevKitC-C6:** WS2812 on GPIO 8 via RMT, GRB, cap 64/255. Solid blue while running
    from `ota_0`, solid amber while running from `ota_1`. Blink during the transfer.
    Red blinks on failure, then return to the slot colour. A GPIO level produces no light.
  - **XIAO ESP32-C6:** GPIO 15, active LOW. One short blink after boot for `ota_0`, two
    short blinks for `ota_1`, rapid blink during transfer, a slower distinct pattern on
    failure. Do not run the RMT encoder on this board.
- Wi-Fi SSID, Wi-Fi password, OTA URL, and the path to the CA PEM are compile-time
  environment variables. Placeholder values must let `cargo build --release` succeed.
  Never put secrets in `src/`. Never log the PSK.
- Partition table: profile `dual-ota` from `shared-specs/partition-table-profiles.md`.
  The same CSV fits 4 MB (XIAO) and 8 MB (DevKitC-N8). Flash size follows the board.
  The first `cargo run` image must boot from `ota_0`. There is no `sdkconfig`.
- Serial log at INFO: booted slot, Wi-Fi result (no password), download progress in
  coarse steps, CRC result, activate-or-reject, and reboot. Certificate failure names
  the reason class (untrusted, handshake, timeout) and does not print key material.

## Wakeup Strategy

Not applicable. The example runs until the operator presses the button or power is removed.
It does not enter deep sleep.

## Connectivity

Wi-Fi STA + one HTTPS GET. No MQTT, no BLE, no ESP-NOW.

This is a **radio** example: git-pin `esp-hal`, `esp-radio`, `esp-rtos`, `esp-alloc`,
`esp-bootloader-esp-idf`, and `esp-storage` from tag `esp-hal-v1.2.1`. Never mix those
crates from crates.io with git `esp-hal`. `esp-radio` is an unstable beta.

TLS is `embedded-tls` **0.19.0** with the `std` feature off (see
`shared-specs/rust-toolchain.md`). That crate speaks TLS 1.3 only. The lab server must
offer TLS 1.3. Minimum policy in `shared-specs/security-baseline.md` is TLS 1.2;
TLS 1.3 satisfies it. Do not disable certificate verification to make a server work.

## Security Requirements

Follow `shared-specs/security-baseline.md`. **Production Target: no.**

- HTTPS with the embedded CA. Refuse the download when verification fails.
- CRC32 is a transfer check, not a signature and not Secure Boot.
- No eFuse burns, no flash-encryption enablement, no committed private keys.
- No hardcoded Wi-Fi secrets. The CA PEM is public trust-anchor material; it may live in
  the example tree as a placeholder the operator replaces. Do not commit a server
  private key.
- Do not log PSKs, session keys, or the CA private key (there should be no private key).

## Hardware Dependencies

- Board-spec: `board-specs/espressif/esp32-c6-devkitc-1-n8.md` (WS2812 GPIO 8; BOOT GPIO 0;
  8 MB flash; flash/monitor on the **left** USB-C UART-C port).
- Board-spec: `board-specs/seeed/xiao-esp32c6.md` (user LED GPIO 15 active LOW; BOOT GPIO 9;
  4 MB flash; USB Serial/JTAG).
- A 2.4 GHz WPA2-Personal access point.
- An HTTPS server that speaks TLS 1.3 and serves the enveloped image at the configured URL.
  The URL must be the final URL. Redirects are out of scope.
- No extra LEDs, buttons, or sensors.

The generator auto-invokes `esp32-ws2812-led-engineer` for the DevKitC RMT path.

## Partition Table

Profile: `dual-ota` from `shared-specs/partition-table-profiles.md`.

Emit `partitions.csv` and point espflash at it. Set flash size to 8 MB for
`board-devkitc-c6` and 4 MB for `board-xiao-c6`. Do not add a factory app partition.
The first flashed image must be bootable from `ota_0` (offset `0x20000` in that profile).

## Limitations

| This example is NOT | Why |
|---|---|
| Signed production updates | CRC32 is not a signature. Secure Boot v2 is an irreversible eFuse step and is out of scope |
| Automatic rollback | The bootloader shipped with current `espflash` is not built with OTA auto-rollback. A bad image that was activated stays selected until something writes `otadata` again |
| A full WebPKI client | One embedded trust anchor via `embedded-tls`. Expiry and revocation beyond what that crate checks are not a lab requirement |
| TLS 1.2 | `embedded-tls` 0.19.0 is TLS 1.3 only |
| An ESP-IDF OTA app | No `esp_https_ota`, no Kconfig, no `sdkconfig`, no FreeRTOS tasks |
| A two-program LED rig | One firmware, two slots. The slot is shown by the onboard LED and the log |

Extension path: keep the slot writer; add a real signature check only when the project
takes on a rollback-capable bootloader and a key ceremony. Do not bolt Secure Boot onto
this example in place.

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf` (default DevKitC-C6 features) without real Wi-Fi secrets in `src/`.
- `cargo clippy --release -- -D warnings` is clean for the default features.
- `cargo build --release --no-default-features --features board-xiao-c6` and the matching clippy invocation are clean.
- **`cargo run --release`** flashes via `espflash` on DevKitC-C6 (left USB-C / UART-C) using the `dual-ota` table and an 8 MB flash size. Serial shows the booted slot. The WS2812 shows the slot colour (blue `ota_0`, amber `ota_1`).
- With a reachable AP, a TLS 1.3 server, a CA that matches the embedded trust anchor, and a correctly enveloped image: a button press downloads, logs a CRC match, reboots, and the log plus the LED show the other slot.
- A server certificate that does not chain to the embedded CA does not change the boot slot. The device returns to the wait and shows the error indication.
- A body whose CRC or length does not match the prefix does not change the boot slot.
- Optional XIAO: `cargo run --release --no-default-features --features board-xiao-c6` with a 4 MB flash size. GPIO 15 blinks once for `ota_0` and twice for `ota_1`.
- No panic on a failed handshake, a short body, or a missing AP.
