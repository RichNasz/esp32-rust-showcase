# ESP-NOW Sensor to Gateway (ESP32-C6)

## Overview

Two-role rust-on-metal ESP-NOW link for ESP32-C6 (`no_std` `esp-hal`). One firmware image,
two mutually exclusive Cargo features. The **sensor** wakes on an RTC timer, sends one
unicast datagram to a compile-time gateway MAC, tears the radio down, and deep-sleeps.
The **gateway** stays awake, logs its own MAC and every accepted datagram, and blinks the
onboard LED. Chip is ESP32-C6 only.

The directory name `esp-now-low-power-mesh` is historical. This example is **not** a mesh.
There is no relay, no hop count, no beacon discovery, and no Wi-Fi RSSI scan.

## Implementation status

**Specs only** as of 2026-10-20. These Functional, Coding, and Test specs are the current design (sensor-to-gateway unicast; the directory name is historical). There is no `Cargo.toml` and no `src/` until `esp32-sdd-full-project-generator` runs. The catalog table in `specs/showcase-master-spec.md` is the source of truth across examples.

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
YEJMKJ ESP32-S3 boards are **not in v1**.

RTC / LP GPIOs on C6 are **0–7 only**. GPIO 9 (XIAO boot button) is **not** RTC-capable and
**cannot** wake the chip from deep sleep. The sensor may also wake from GPIO 0. On XIAO
that is D0 (an external button), not the silkscreen BOOT key.

## Features

- Two compile-time roles, not a runtime menu and not Kconfig:
  - `role-sensor` (default, together with `board-devkitc-c6`)
  - `role-gateway`
  - Enabling both, or neither, is a compile error.
  - Gateway on DevKitC: `cargo run --release --no-default-features --features board-devkitc-c6,role-gateway`
  - XIAO sensor: `cargo run --release --no-default-features --features board-xiao-c6,role-sensor`
  - XIAO gateway: `cargo run --release --no-default-features --features board-xiao-c6,role-gateway`
- Both roles use the same fixed 802.11 channel, from compile-time environment
  `ESPNOW_CHANNEL` (placeholder default `1`). A channel mismatch fails silently at the
  radio; the README must say that.
- Neither role associates to an access point. Neither role runs a TCP/IP stack.

### Sensor

- Wake from deep sleep on an RTC timer of about **15 seconds**, plus optional RTC GPIO 0
  (active LOW, internal pull-up): DevKitC BOOT, or an external button on XIAO D0.
- Increment a boot counter in `#[esp_hal::ram(persistent)]` (or the equivalent
  `#[ram(unstable(...))]` form), guarded by magic and a checksum. Lost on power-cycle.
- Send **one** unicast ESP-NOW datagram to `ESPNOW_GATEWAY_MAC` (compile-time hex MAC).
  The placeholder `00:00:00:00:00:00` means "not configured": log that, blink the failure
  pattern, and sleep. Do not hang.
- Payload: the boot count plus a fixed magic so the gateway can ignore unrelated frames.
  The payload must fit in one ESP-NOW frame (250 bytes maximum).
- Wait about **200 ms** for send status. On success, one LED blink. On timeout or send
  error, a distinct failure blink. Then tear the radio down and sleep. The active window
  must not exceed about **3 seconds**.
- LED off during sleep.
  - **DevKitC-C6:** WS2812 GPIO 8, RMT, GRB, cap 64/255. Green one-shot on success, red
    on failure, black before sleep.
  - **XIAO:** GPIO 15 active LOW. One blink on success, a short double-blink on failure.
    No RMT.

### Gateway

- Never sleep. Bring up ESP-NOW on the same channel and stay in receive.
- Log the station MAC at boot in a form the operator can paste into `ESPNOW_GATEWAY_MAC`.
- On each datagram whose magic matches, log the sender MAC and the boot count, and blink
  the LED once (DevKitC: one blue flash at 64/255; XIAO: one active-LOW blink). Drop
  other frames with a debug or warn line, not a panic.
- No forwarding, no beacon, no scan.

### Shared

- Pin numbers live only in generated `src/board.rs`.
- Serial log at INFO (`log` + `esp-println`): role, channel, gateway MAC on the gateway,
  boot count and send result on the sensor, receive line on the gateway.
- USB console: DevKitC **left** USB-C UART-C stays up across sensor sleep. Native USB
  Serial/JTAG (DevKitC right port; XIAO) drops on sensor sleep. The gateway does not
  sleep, so either USB port can monitor it. Use a UART-bridge DevKitC for the sensor
  when you need more than one cycle of logs.

## Wakeup Strategy

**Sensor:** RTC timer (~15 s from sleep entry) plus RTC GPIO 0. GPIO 9 is not a wakeup source.

**Gateway:** not applicable. The gateway does not sleep.

Deep sleep does not return; the sensor resets on wake. Sleep, RTC, RMT, and radio APIs
require `esp-hal` feature `unstable`.

## Connectivity

ESP-NOW only, one unicast peer, one fixed channel. No Wi-Fi association, no MQTT, no BLE.

This is a **radio** example: git-pin `esp-hal`, `esp-radio`, `esp-rtos`, `esp-alloc`, and
`esp-bootloader-esp-idf` from tag `esp-hal-v1.2.1`. Enable the `esp-now` feature (unstable).
Never mix crates.io `esp-rtos = "0.4.0"` with git `esp-hal`. `esp-radio` is an unstable beta.

## Security Requirements

Not production-targeted. ESP-NOW payloads are not encrypted by this example. Do not put
secrets in the datagram. The gateway MAC is an address, not a credential. No Wi-Fi PSK
is used.

## Hardware Dependencies

- Board-spec: `board-specs/espressif/esp32-c6-devkitc-1-n8.md` (WS2812 GPIO 8; BOOT GPIO 0;
  flash/monitor on the **left** USB-C UART-C port).
- Board-spec: `board-specs/seeed/xiao-esp32c6.md` (user LED GPIO 15 active LOW; BOOT GPIO 9
  is not RTC; D0 / GPIO 0 is the external wake pin).
- Two C6 boards for the on-air test (one sensor, one gateway). Any mix of DevKitC-C6 and
  XIAO C6 is fine if both images use the same channel. A single board can still build
  either role.
- No sensor breakout and no access point.

The generator auto-invokes:

- `esp32-deep-sleep-engineer` — sensor RTC timer + GPIO 0, persistent RAM, radio teardown
- `esp32-ws2812-led-engineer` — DevKitC WS2812 RMT path

## Power Budget

Sensor only. Gateway current is continuous USB/active and is not a battery budget.
C6 sleep floor from board-specs (~7–15 µA DevKitC with the RTC domain; ~15 µA XIAO).
ESP-NOW transmit is a short Wi-Fi-radio burst; ~80 mA average over the active window is
the planning figure (peaks higher). Radio left powered reads milliamps in sleep.

| Mode | Typical current | Duration | Energy per cycle |
|---|---|---|---|
| Active (CPU + ESP-NOW, no AP association) | ~80 mA | ~0.5 s typical (≤ 3 s cap) | ~11 µAh |
| Radio teardown | included in active | ~5–10 ms | — |
| Deep sleep (RTC timer + GPIO 0 + persistent RAM) | ~7–15 µA | 15 s | ~0.06 µAh |
| **Total per duty cycle** | — | ~15.5 s | ~11 µAh |

**Average current (typical):** `(80 mA × 0.5 s + 0.015 mA × 15 s) / 15.5 s ≈ 2.6 mA` — target **< 3 mA** when send finishes promptly.

**Sleep wakeup source:** RTC timer + RTC GPIO 0 (sensor only)
**Wakeup interval:** 15 seconds

## Limitations

| This example is NOT | Why |
|---|---|
| A multi-hop mesh | One sender, one receiver, no relay and no routing table |
| Dynamic gateway discovery | The sensor MAC is a compile-time address. The gateway prints its MAC so the operator can copy it |
| A Wi-Fi scanner | RSSI-of-nearby-SSIDs was the retired mesh demo. ESP-NOW and a scan fight over one radio, and this example does not scan |
| A light-sleep forwarder | Light-sleep power management of an always-on relay is not part of this `esp-rtos` design |
| Encrypted ESP-NOW | Frames are plaintext lab traffic |
| An access-point client | No association, no DHCP, no TCP |

Extension path: add a third role only after this pair is reliable on two boards. Raw
IEEE 802.15.4 is `ieee802154-raw-frames`, not a feature flag here.

## Success Criteria

- `cargo build --release` (default: DevKitC-C6 + `role-sensor`) completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf`.
- `cargo clippy --release -- -D warnings` is clean for that default.
- `cargo build --release --no-default-features --features board-devkitc-c6,role-gateway` and the matching clippy invocation are clean.
- The two XIAO feature sets (`board-xiao-c6,role-sensor` and `board-xiao-c6,role-gateway`, each with `--no-default-features`) build and clippy clean.
- Building with both role features, or with neither, fails at compile time.
- **`cargo run --release`** on a DevKitC-C6 sensor (left USB-C) with a real gateway MAC and a gateway running the same channel: each wake logs a boot count that increments by 1, the gateway logs that count and the sender MAC, and the sensor sleeps. The sensor LED flashes once per successful send and is dark during sleep. The monitor on UART-C stays connected across sleep.
- A placeholder gateway MAC does not hang. The sensor logs the skip, shows the failure indication, and sleeps within the 3 second cap.
- GPIO 0 wakes the sensor into the same send path. GPIO 9 does not.
- Gateway: `cargo run --release --no-default-features --features board-devkitc-c6,role-gateway` logs a MAC and does not enter deep sleep.
- No stack overflow across 10 sensor cycles with the gateway powered.
