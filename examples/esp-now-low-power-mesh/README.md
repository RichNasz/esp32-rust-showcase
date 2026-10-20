<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# ESP-NOW Sensor to Gateway — ESP32-C6

> One sensor deep-sleeps and sends a single unicast datagram. One gateway stays awake and logs it. The directory name `esp-now-low-power-mesh` is historical. This is not a mesh.

## Implementation status

**Specs only** as of 2026-10-20. The Functional, Coding, and Test specs in this folder are the current design. There is no `Cargo.toml` and no `src/`. `cargo build` and `cargo run` fail in this directory until `esp32-sdd-full-project-generator` runs. This README describes the spec. It is not a record of packets on the air.

The design enables `esp-hal` feature **`unstable`** (ESP-NOW, sleep, RTC, RMT). **`esp-radio` is an unstable beta**, and the `esp-now` feature is unstable. Crates are git-pinned at tag `esp-hal-v1.2.1`. Those APIs may change with `esp-hal` minor releases.

## Overview

One firmware image, two mutually exclusive Cargo features:

- `role-sensor` (default, with `board-devkitc-c6`) — wake on an RTC timer of about 15 seconds (and optional GPIO 0), send one datagram to a compile-time gateway MAC, tear the radio down, sleep.
- `role-gateway` — never sleep. Log the station MAC at boot. Log each datagram whose magic matches, and blink once.

Neither role associates to an access point. There is no relay, no beacon discovery, no hop count, and no Wi-Fi RSSI scan. Both images must use the same `ESPNOW_CHANNEL` (default `1`). A channel mismatch looks like a dead gateway.

The placeholder gateway MAC `00:00:00:00:00:00` means "not configured". The sensor logs that, shows the failure indication, and sleeps. It must not hang.

GPIO 9 (XIAO BOOT) cannot wake the sensor. GPIO 0 can: DevKitC BOOT, or an external button on XIAO D0.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Two ESP32-C6 boards for the on-air test (DevKitC-C6, XIAO C6, or one of each). One board can still build either role after generation.

ESP-IDF is **not** required. No access point is required.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6. Wi-Fi (for ESP-NOW) and a user LED are required.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only. ESP32-H2 has no Wi-Fi, so it cannot run ESP-NOW. It is not in v1 anyway.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep (sensor on the **left** USB-C) | WS2812, cap 64/255. Sensor: green = sent, red = failed or skipped. Gateway: blue flash per accepted frame | ~7–15 µA (sensor, after radio teardown) |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW. One blink = success, double blink = failure | ~15 µA (sensor, after radio teardown) |

The gateway does not sleep, so either USB port can monitor it. Use a UART-bridge DevKitC for the sensor when you need more than one cycle of logs. Forgetting radio teardown leaves the sensor in milliamps during sleep.

## After the crate is generated

These commands are the intended workflow. **They fail today** because this folder has no `Cargo.toml`.

Sensor, DevKitC-C6 (default features `board-devkitc-c6` + `role-sensor`):

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

Gateway, DevKitC-C6. `--no-default-features` is required so `role-sensor` is not also enabled:

```sh
cargo run --release --no-default-features --features board-devkitc-c6,role-gateway
```

XIAO sensor and gateway:

```sh
cargo run --release --no-default-features --features board-xiao-c6,role-sensor
cargo run --release --no-default-features --features board-xiao-c6,role-gateway
```

Set `ESPNOW_CHANNEL` on both images. Copy the gateway's logged station MAC into the sensor's `ESPNOW_GATEWAY_MAC`. Enabling both role features, or neither, must fail at compile time.

> **Never edit generated source files directly.** To change behaviour, update the specs and regenerate.

### Opening in VS Code / Cursor

After generation this folder is a standalone Cargo package. Open **only this folder**.

**File → Open Folder… → select `examples/esp-now-low-power-mesh/`**

Until then the folder is specs plus this README.

## Key Concepts

- **Unicast pair** — one sender, one receiver, fixed channel, compile-time peer MAC. Not a routing protocol.
- **Radio teardown** — the sensor drops `esp-radio` before `sleep_deep`. ESP-NOW shares the 802.11 radio. There is no `embassy-net` and no AP association.
- **Persistent boot count** — `#[esp_hal::ram(persistent)]` on the sensor only. Lost on power-cycle. Guarded by magic and a checksum.
- **Payload** — boot count plus a fixed magic, under the 250-byte ESP-NOW maximum.
- **Application descriptor** — `esp_app_desc!()` is required for current `espflash`. This is not an ESP-IDF install.

## Limitations

| This example is NOT | Why |
|---|---|
| A multi-hop mesh | No relay and no routing table |
| Dynamic gateway discovery | The sensor address is compile-time. The gateway prints its MAC so you can copy it |
| A Wi-Fi scanner | No SSID RSSI scan |
| A light-sleep forwarder | The gateway is fully awake. The sensor uses deep sleep, not light-sleep relay |
| Encrypted ESP-NOW | Frames are plaintext lab traffic |
| An access-point client | No association, no DHCP, no TCP |

IEEE 802.15.4 raw frames are a deferred catalog item, not a feature of this example.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

Run these only after the crate exists. From `examples/esp-now-low-power-mesh/`:

1. **T-A1 — DevKitC sensor (default)** — `cargo build --release` and `cargo clippy --release -- -D warnings` exit 0.
2. **T-A2 — DevKitC gateway** — the same two commands with `--no-default-features --features board-devkitc-c6,role-gateway`.
3. **T-A3 — XIAO, both roles** — `board-xiao-c6,role-sensor` and `board-xiao-c6,role-gateway`, each with `--no-default-features`.
4. **T-A4 — roles are exclusive** — a build with both role features, and a build with the board feature and no role, must fail to compile.

### Manual Tests — hardware required

Why manual: delivery and deep-sleep retention need two radios.

**T-M1 — Two DevKitC boards.** Flash the gateway first and copy its MAC. Flash the sensor with that MAC and the same channel. Watch at least five wakes. The gateway count matches the sensor. The sensor LED flashes once and is dark during the ~15 s sleep. The left USB-C monitor stays connected.

**T-M2 — Unconfigured sensor.** Default MAC, no override. The log says the MAC is unset, the failure colour shows, and the device sleeps within about 3 seconds.

**T-M3 — XIAO sensor (optional).** USB Serial/JTAG exits on the first sleep. That is expected. GPIO 15 blinks once per send the gateway accepted.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)
