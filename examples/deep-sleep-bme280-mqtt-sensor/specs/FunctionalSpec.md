# Deep Sleep BME280 + MQTT Sensor Node (ESP32-C6)

## Overview

Duty-cycled rust-on-metal IoT node for ESP32-C6 (`no_std` `esp-hal`). Each wake reads
temperature, humidity, and pressure from a BME280 over I2C, joins Wi-Fi as a station via
`esp-radio`, publishes one JSON MQTT message, tears the radio down, and returns to deep
sleep. Demonstrates the wake → sense → transmit → sleep loop used in battery-powered
sensor networks. Chip is ESP32-C6 only.

On DevKitC-C6, GPIO 0 (boot button, RTC-capable) can trigger an on-demand reading. On
XIAO ESP32-C6 the onboard boot button is GPIO 9, which is **not** an RTC GPIO and cannot
wake from deep sleep; use the RTC timer or an external button on LP GPIO 0–7 (this
example arms GPIO 0 / D0 as that external wake pin).

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/main.rs` are present (generated 2026-09-08). This line is not a hardware-test result. The catalog table in `specs/showcase-master-spec.md` is the source of truth across examples.

## Target Hardware

v1 chip: **ESP32-C6** only (`esp32c6`). Other SoCs are not in v1.

### Board Compatibility Checklist

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [x] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add boards for any SoC other than ESP32-C6.

RTC / LP GPIOs on C6 are **0–7 only**. GPIO 9 (XIAO boot button) is **not** RTC-capable
and **cannot** wake the chip from deep sleep. Do not invent a GPIO-9 wakeup workaround.

## Features

- Wake from deep sleep on an RTC timer of about **15 seconds**, measured from sleep entry.
- Optional RTC GPIO wakeup on **GPIO 0** (active LOW, internal pull-up): DevKitC boot
  button; XIAO external button on D0. Same sense+publish pipeline as a timer wake.
- On every wake: increment a boot counter stored with `#[esp_hal::ram(persistent)]` (or
  the equivalent `#[ram(unstable(...))]` form). Guard with magic + checksum so random
  bytes after a raced reset are first boot, not a huge count. Lost on power-cycle.
- Read BME280 over I2C in **forced** mode (one-shot, then standby). Pins from generated
  `src/board.rs` (DevKitC default SDA/SCL GPIO 6/7; XIAO D4/D5 GPIO 22/23). Probe 0x76
  then 0x77. Temperature °C, humidity %RH, pressure **hPa**.
- Serial INFO for a sample uses the **same units and one-decimal scale** as the MQTT JSON
  (`temp` °C, `hum` %, `pres` hPa). Sea-level pressure is about **`1013.1` hPa** on serial
  (`P=1013.1 hPa`) and in `"pres":1013.1`. Compensation stores Pascal internally; do **not**
  log Pascal, and do **not** log a 100×-smaller hPa figure (that would show ~10.1 at sea
  level).
- Connect Wi-Fi STA (WPA2-Personal) via `esp-radio` + `embassy-net` DHCPv4. Association is
  capped at about 10 s **or** whatever remains of the active window, whichever is shorter.
  Hard wall-clock deadline **15 s from wake** covers the **whole** cycle (sense + STA +
  MQTT + LED). If that deadline expires, tear the radio down if it is up and sleep. Do not
  let independent stage timeouts stack past 15 s. Never block indefinitely on network or
  I2C (a stuck I2C bus must time out so it cannot consume the window).
- Publish a single MQTT 3.1.1 QoS 0 message (retain false, no TLS) to topic
  `sdd/sensor/bme280` with JSON
  `{"temp":22.5,"hum":48.2,"pres":1013.1,"boot":42}`.
- Wi-Fi SSID/password, MQTT broker host/port, and MQTT client id come from **compile-time
  environment variables** (documented in the README / `.cargo/config.toml` `[env]`).
  **Never** string-literal secrets in `src/`. **Never** log the PSK or other credentials.
- Tear down STA / drop `esp-radio` before deep sleep so sleep current stays in microamps.
- LED confirmation after a successful publish (three visible blinks), then dark for sleep.
  Failure uses a distinct colour / pattern (DevKitC red vs green; XIAO blink vs no
  success blink) and still sleeps. No hang.
  - **DevKitC-C6**: WS2812 on GPIO 8 via RMT, GRB, ≤64/255, 800 kHz NRZ.
  - **XIAO ESP32-C6**: GPIO 15 active-LOW binary blinks. No RMT on this board.
- Pin numbers live only in generated `src/board.rs`. Board selection is a Cargo feature
  (`board-devkitc-c6` default; XIAO: `--no-default-features --features board-xiao-c6`).
- USB console: DevKitC **left** USB-C UART-bridge stays connected through sleep. Native
  USB Serial/JTAG (DevKitC right port; XIAO) drops `/dev/cu.usbmodem*` on sleep entry.

## Wakeup Strategy

**RTC timer** (~15 s from sleep entry) **plus RTC GPIO 0** (active LOW) on both boards.
GPIO 9 is not used as a wakeup source.

Deep sleep does not return; the chip resets on wake. Sleep / RTC / RMT / radio APIs
require `esp-hal` feature `unstable`.

## Connectivity

Wi-Fi STA + MQTT 3.1.1 over TCP, no TLS (TLS is the `secure-ota-https` example).

This is a **radio** example: git-pin `esp-hal`, `esp-radio`, `esp-rtos`, and `esp-alloc`
from tag `esp-hal-v1.2.1`. Never mix crates.io `esp-rtos = "0.4.0"` with git `esp-hal`.
`esp-radio` is an unstable beta.

## Security Requirements

Not production-targeted. Minimum baseline:

- No hardcoded Wi-Fi / MQTT secrets in committed `src/`.
- Never log PSKs, broker passwords, or tokens.
- MQTT is plaintext QoS 0 for a local lab broker. Do not treat this as a secure cloud path.

## Hardware Dependencies

- Board-spec: `board-specs/espressif/esp32-c6-devkitc-1-n8.md` (WS2812 GPIO 8; boot GPIO 0;
  default I2C GPIO 6/7; flash/monitor on the **left** USB-C UART-C port).
- Board-spec: `board-specs/seeed/xiao-esp32c6.md` (user LED GPIO 15 active LOW; boot GPIO 9
  is not RTC; I2C D4/D5 = GPIO 22/23; USB Serial/JTAG).
- External: BME280 (or BMP280 for T/P only) breakout at 3.3 V on the board I2C pins.
- Wi-Fi AP (2.4 GHz, WPA2-Personal) and an MQTT 3.1.1 broker (e.g. mosquitto, no TLS).

The generator auto-invokes:

- `esp32-deep-sleep-engineer` — RTC timer + GPIO 0 wakeup, persistent RAM, radio teardown, sleep
- `esp32-ws2812-led-engineer` — DevKitC WS2812 RMT path

## Power Budget

C6 with Wi-Fi. Sleep floor from board-specs (~7–15 µA DevKitC with RTC domain; ~15 µA XIAO).
Typical STA + MQTT active current ~80 mA average (peaks higher during TX). Radio teardown
(~5–10 ms) is part of the active window. Target assumes a **typical** ~1 s active window;
the 15 s figure is a hard timeout, not the expected duration.

| Mode | Typical current | Duration | Energy per cycle |
|---|---|---|---|
| Active (CPU + Wi-Fi STA + MQTT) | ~80 mA | ~1 s typical (≤ 15 s timeout) | ~22 µAh |
| Radio teardown | included in active | ~5–10 ms | — |
| Deep sleep (RTC timer + GPIO 0 + persistent RAM) | ~7–15 µA | 15 s | ~0.06 µAh |
| **Total per duty cycle** | — | ~16 s | ~22 µAh |

**Average current (typical 1 s active):** `(80 mA × 1 s + 0.015 mA × 15 s) / 16 s ≈ 5.0 mA` — target **< 5 mA** when association is fast. A full 15 s timeout on a missing AP raises the average substantially; that path is for failure recovery, not the happy-path budget.

**Sleep wakeup source:** RTC timer + RTC GPIO 0  
**Wakeup interval:** 15 seconds

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf` (default DevKitC-C6 features) without requiring real Wi-Fi secrets in source.
- `cargo clippy --release -- -D warnings` is clean for the default features.
- `cargo build --release --no-default-features --features board-xiao-c6` and the matching clippy invocation are clean.
- **`cargo run --release`** flashes via `espflash` on DevKitC-C6 (**left** USB-C / UART-C). With valid compile-time credentials, a live AP, broker, and BME280: serial shows wake + readings; broker receives well-formed JSON; `boot` increments; device sleeps. Monitor stays connected across sleep on UART-C.
- Optional XIAO: `cargo run --release --no-default-features --features board-xiao-c6`. USB monitor dropping on first sleep is expected.
- BME280 readings in plausible ranges when the sensor is present (about 10–40 °C, 10–90 %, 900–1100 hPa).
- Unreachable AP / broker / missing sensor / stuck I2C: log a recoverable error (no secrets) and sleep within the 15 s active window — no hang. Serial pressure and MQTT `pres` agree in hPa.
- GPIO 0 (DevKitC BOOT, or XIAO external on D0) wakes the device. GPIO 9 is not used for wakeup.
- No stack overflow or watchdog across 10+ consecutive cycles in stable RF conditions.

## Limitations

- MQTT is unencrypted QoS 0. Use `secure-ota-https` patterns for TLS.
- Wi-Fi is 2.4 GHz only on these C6 boards.
- XIAO onboard BOOT (GPIO 9) cannot wake from deep sleep.
- Dummy compile-time `[env]` values let the crate **build**; they will not associate with a real AP until the operator overrides them (README).
