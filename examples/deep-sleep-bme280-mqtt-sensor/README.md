<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Deep Sleep BME280 + MQTT — Real IoT Sensor Node

> Duty-cycled IoT node: wake → I2C sense → Wi-Fi STA → MQTT publish → deep sleep.

This example enables `esp-hal` feature **`unstable`** (radio + sleep / RTC). **`esp-radio` is an unstable beta** (git-pinned with the rest of the `esp-hal` monorepo stack at tag `esp-hal-v1.2.1`). Those APIs may change with `esp-hal` minor releases.

## Overview

Production IoT pattern on ESP32-C6. Wakes from deep sleep, reads temperature, humidity, and pressure from a BME280 over I2C, connects to Wi-Fi, publishes a JSON payload via MQTT, then returns to deep sleep inside a bounded active window. Demonstrates the minimal wake → sense → transmit → sleep loop used in battery-powered sensor networks.

On DevKitC-C6, GPIO 0 (boot button, RTC-capable) can trigger an immediate wakeup for on-demand readings. On XIAO ESP32-C6 the boot button is GPIO 9, which is **not** an RTC GPIO — use timer-only wakeup or an external button on LP GPIO 0–7.

Tear down `esp-radio` before deep sleep so sleep current stays in microamps.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- External: BME280 breakout (3.3 V) on the board's default I2C pins
- Wi-Fi AP + MQTT broker

ESP-IDF is **not** required. Radio crates are git-pinned — never mix crates.io `esp-rtos = "0.4.0"` with git `esp-hal`.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6. Wi-Fi, deep-sleep timer wakeup, and a user LED are required.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

I2C (from board-specs): DevKitC default SDA/SCL = GPIO 6 / 7. XIAO D4/D5 = GPIO 22 / 23.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | Color: green = success, red = failure, off = sleep (WS2812) | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW | ~15 µA |

UART-bridge boards keep the serial monitor connected through deep sleep; native USB boards drop and reconnect — this is expected, not a crash. Both C6 boards scan **2.4 GHz Wi-Fi 6** only.

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

Wi-Fi SSID/password and MQTT broker URI are crate configuration (not sdkconfig). After generation, set them in the example's documented config path and regenerate if the spec changes.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/deep-sleep-bme280-mqtt-sensor/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

```
bme280-mqtt: Wake #42 — timer
bme280-mqtt: T=22.5 C  H=48.2 %  P=1013.1 hPa
bme280-mqtt: MQTT publish ok  topic=sdd/sensor/bme280
bme280-mqtt: radio teardown; sleeping
```

JSON payload shape: `{"temp":22.5,"hum":48.2,"pres":1013.1,"boot":42}` on topic `sdd/sensor/bme280`.

## Key Concepts

- **Duty-cycled STA + MQTT** — bounded active window; disconnect cleanly before sleep
- **`esp-radio` Wi-Fi** — unstable beta; git-pin the monorepo stack
- **Radio teardown** — required or deep-sleep current stays in milliamps
- **BME280 over I2C** — pins from `src/board.rs`
- **Persistent boot counter** — `#[esp_hal::ram(persistent)]`
- **RTC GPIO wakeup** — C6 GPIO 0–7 only

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build**

   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   ```

   Pass: exit 0, zero warnings. Also confirms radio git-pin resolves.

2. **T-A2 — XIAO feature build**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   ```

   Pass: exit 0.

### Manual Tests — hardware required

**T-M1 — BME280 reads valid data**

Why manual: real I2C and sensor coefficients require physical hardware.

1. Wire BME280: VCC 3.3 V, GND, SDA/SCL to the board-spec I2C pins.
2. `cargo run --release`
3. Confirm temperature 10–40 °C, humidity 10–90 %, pressure 900–1100 hPa.
4. Breathe on the sensor — humidity should increase.

Pass: all three readings in range; humidity responds.

---

**T-M2 — MQTT publish reaches broker**

Why manual: end-to-end MQTT needs a live broker and AP.

1. Run a broker (e.g. `mosquitto`) and subscribe to `sdd/sensor/bme280`.
2. Flash and confirm well-formed JSON each cycle; `boot` increments.

Pass: broker receives JSON every cycle.

---

**T-M3 — Button wakeup (DevKitC GPIO 0)**

Why manual: physical RTC GPIO wakeup requires silicon.

1. During deep sleep, press the DevKitC BOOT button (GPIO 0).
2. Confirm immediate wakeup and a full sense+publish cycle.

Pass: button wakes the device. (XIAO boot GPIO 9 cannot do this — timer-only or external LP GPIO.)

---

**T-M4 — Active window timeout**

Why manual: requires deliberately invalid Wi-Fi credentials.

1. Configure a non-existent SSID.
2. Confirm the device logs a timeout and sleeps within the active window — no hang.

Pass: sleeps on network failure.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
