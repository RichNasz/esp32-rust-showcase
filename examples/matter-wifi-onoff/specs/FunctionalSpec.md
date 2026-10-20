# Matter Wi-Fi On/Off (ESP32-C6)

## Overview

A rust-on-metal Matter on/off light for ESP32-C6 (`no_std` `esp-hal`). The device
commissions over BLE, then joins the Wi-Fi network the commissioner provides. An
on command lights the status LED. An off command turns it off. Chip is ESP32-C6
only.

This example is Matter over Wi-Fi with the public test attestation credentials
from `rs-matter`. It is not Matter over Thread, not a CSA-certified product, and
not raw IEEE 802.15.4. Thread stays in `thread-udp-pair`. The 802.15.4 radio stays
off.

## Implementation status

**Firmware generated** as of 2026-10-20. The catalog row in
`specs/showcase-master-spec.md` is the source of truth. A clean host build is not
a commissioning pass. The key store is RAM. Every reset requires commissioning
again.

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

## Features

- One image. Default board feature `board-devkitc-c6`. XIAO is
  `board-xiao-c6`. Selecting both boards, or neither, fails at compile time.
- BLE commissioning and Wi-Fi together (`esp-radio` features `wifi`, `ble`, and
  `coex`). The commissioner supplies the SSID and passphrase. The firmware does
  not embed a Wi-Fi password.
- On/off cluster on endpoint 1. The root endpoint carries the stack's system
  clusters. The status LED follows the on/off attribute. On is lit. Off is dark.
  DevKitC uses the WS2812 (white at the brightness cap). XIAO uses the
  active-LOW GPIO LED.
- At boot the firmware logs the Matter manual pairing code and the QR payload
  from the test commissioning data, so a commissioner can pair without a second
  channel.
- Fabrics and operational credentials live in a RAM key store. A reset clears
  them.

## Wakeup Strategy

Not applicable. The device stays awake while commissioned.

## Connectivity

Matter over Wi-Fi, with BLE for commissioning. Copy the radio pin from
`shared-specs/rust-toolchain.md`. The Matter integration crate is git
`rs-matter-embassy` at the rev in that file. Do not enable `ieee802154`. Do not
depend on ESP-IDF or `esp-matter`.

## Security Requirements

Test device attestation only, the credentials published with `rs-matter` for
examples. They do not authenticate a production product. No private key is
committed beyond those public test constants, which ship inside the dependency.
Production Target: no. The Wi-Fi passphrase is not compiled into the image.

## Partition

The image may be larger than the 1 MB default factory app. When the linked app
exceeds 1 MB, use one custom single-app `partitions.csv` that still fits the
XIAO's 4 MB flash, and point the espflash runner at it. Do not use the `dual-ota`
profile. This example does not implement Matter OTA.

## Limitations

| Topic | What this example does |
| --- | --- |
| Matter over Thread | Not this example. Wi-Fi and 802.15.4 do not coexist on this pin |
| CSA certification | Test attestation only |
| Persistence | RAM store. Recommission after every reset |
| Alexa non-concurrent commissioning | This example uses concurrent BLE and Wi-Fi commissioning |
| Border router | Not used |

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on
  `riscv32imac-unknown-none-elf` for DevKitC and for XIAO.
- `cargo clippy -- -D warnings` is clean for both board features.
- **`cargo run --release`** flashes via `espflash` and the log shows the test
  pairing code.
- A Matter commissioner (Home Assistant or a phone) commissions the board over
  BLE onto a Wi-Fi network. An on command lights the LED. An off command turns
  it off.
- Optional XIAO: `cargo run --release --no-default-features --features board-xiao-c6`.
