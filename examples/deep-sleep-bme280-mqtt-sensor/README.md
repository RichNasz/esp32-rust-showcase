<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-full-project-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Deep Sleep BME280 + MQTT — Real IoT Sensor Node

> Duty-cycled IoT node: wake → I2C sense → Wi-Fi STA → MQTT publish → deep sleep.

This example enables `esp-hal` feature **`unstable`** (radio + sleep / RTC + RMT). **`esp-radio` is an unstable beta** (git-pinned with the rest of the `esp-hal` monorepo stack at tag `esp-hal-v1.2.1`). Those APIs may change with `esp-hal` minor releases.

## Overview

Production IoT pattern on ESP32-C6. Wakes from deep sleep, reads temperature, humidity, and pressure from a BME280 over I2C, connects to Wi-Fi, publishes a JSON payload via MQTT 3.1.1 QoS 0, tears down `esp-radio`, then returns to deep sleep inside a **15-second Instant deadline from wake** covering sense + STA + MQTT + LED. Serial pressure is **hPa** at the same scale as MQTT `pres` (sea-level ≈ 1013.1, not ≈ 10.1).

On DevKitC-C6, GPIO 0 (boot button, RTC-capable) can trigger an immediate wakeup. On XIAO ESP32-C6 the boot button is GPIO 9, which is **not** an RTC GPIO — this firmware arms GPIO 0 (D0) for an external active-LOW button instead. Timer wakeup (~15 s) is always armed.

Pin numbers live in generated `src/board.rs`. Never hard-code GPIOs in `main.rs`.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- External: BME280 breakout (3.3 V) on the board's default I2C pins
- Wi-Fi AP (2.4 GHz, WPA2-Personal) + MQTT 3.1.1 broker (optional for a compile-only check)

ESP-IDF is **not** required. Do not install `espup` or the `esp` rustup channel. Radio crates are git-pinned — never mix crates.io `esp-rtos = "0.4.0"` with git `esp-hal`.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6. Wi-Fi, deep-sleep timer wakeup, I2C, and a user LED are required.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

I2C (from board-specs): DevKitC default SDA/SCL = GPIO 6 / 7. XIAO D4/D5 = GPIO 22 / 23.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only.

GPIO 9 (XIAO boot button) is **not** an RTC GPIO and cannot wake the chip from deep sleep.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | Color: green = success, red = failure, off = sleep (WS2812) | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW | ~15 µA |

UART-bridge boards (DevKitC **left** USB-C) keep `espflash monitor` connected through deep sleep; native USB boards drop `/dev/cu.usbmodem*` and the monitor exits. That is expected, not a crash. Use the DevKitC UART-C port for counter-persistence tests; treat XIAO as LED-only observation unless you accept a single-cycle capture. Both C6 boards scan **2.4 GHz Wi-Fi 6** only.

## Build & Flash

```sh
cargo build --release
cargo clippy --release -- -D warnings
cargo run --release
```

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

### Credentials (compile-time env — not source literals)

Wi-Fi PSK and MQTT broker settings are **not** string literals in `src/`. They come from environment variables (Cargo `[env]` in `.cargo/config.toml` ships placeholders so the crate **builds** without a lab AP):

| Variable | Placeholder | Purpose |
| --- | --- | --- |
| `WIFI_SSID` | `unset` | STA SSID. The firmware **skips** association when this is empty or `unset`. |
| `WIFI_PASSWORD` | `unset` | WPA2 PSK. **Never logged.** |
| `MQTT_BROKER_HOST` | `192.168.1.1` | Broker IPv4 address (hostname DNS is not required for the lab path). |
| `MQTT_BROKER_PORT` | `1883` | MQTT TCP port |
| `MQTT_CLIENT_ID` | `c6-bme280` | MQTT 3.1.1 client id |

Override for a real AP (example):

```sh
WIFI_SSID='your-ap' WIFI_PASSWORD='your-psk' MQTT_BROKER_HOST='192.168.1.10' \
  cargo run --release
```

Or edit `.cargo/config.toml` `[env]`. Cargo may cache env values — **clean rebuild** (`cargo clean`) after changing `[env]`. Do not commit real PSKs. Serial logs never print the password.

MQTT stack: `embassy-net` TCP + an in-crate MQTT 3.1.1 QoS 0 encoder (CONNECT → PUBLISH → DISCONNECT). That matches a duty-cycled sleep cycle better than a forever-running MQTT client crate, and matches the `esp-hal` 1.2 `embassy_dhcp` networking path. No TLS.

> **Never edit generated source files directly.** To change board configuration, update `specs/FunctionalSpec.md` or `specs/CodingSpec.md` and regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/deep-sleep-bme280-mqtt-sensor/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

```
deep_sleep_bme280_mqtt_sensor: Wake #42 — timer
deep_sleep_bme280_mqtt_sensor: T=22.5 C  H=48.2 %  P=1013.1 hPa
deep_sleep_bme280_mqtt_sensor: MQTT publish ok  topic=sdd/sensor/bme280
deep_sleep_bme280_mqtt_sensor: radio teardown; sleeping
deep_sleep_bme280_mqtt_sensor: Sleeping for 15 s
```

JSON payload shape: `{"temp":22.5,"hum":48.2,"pres":1013.1,"boot":42}` on topic `sdd/sensor/bme280`. Serial `P=` is hPa with one decimal, matching `pres`.

If `WIFI_SSID` is the placeholder `unset`, the log skips MQTT and still sleeps.

Persistent RAM is retained across deep sleep but cleared on power-on.

## Key Concepts

- **Duty-cycled STA + MQTT** — one Instant, 15 s from wake (sense+STA+MQTT+LED); stage waits use remaining time so they cannot stack past 15 s; disconnect and drop `esp-radio` before sleep (including on deadline expiry)
- **Serial and MQTT pressure** — hPa, same conversion from Pascal (`Pa/100` and `(Pa/10)%10`)
- **`esp-radio` Wi-Fi** — unstable beta; git-pin `esp-hal` / `esp-radio` / `esp-rtos` / `esp-alloc` at tag `esp-hal-v1.2.1`
- **`embassy-net` + MQTT 3.1.1 QoS 0** — TCP publish, no long-lived MQTT session crate
- **Compile-time credentials** — `env!` / Cargo `[env]`; never log the PSK
- **BME280 forced mode over I2C** — pins from `src/board.rs`; probe 0x76 then 0x77
- **Persistent boot counter** — `#[esp_hal::ram(unstable(rtc_fast, persistent))]` with magic + checksum
- **RTC GPIO wakeup** — C6 GPIO 0–7 only; GPIO 9 is not RTC
- **WS2812 heartbeat** — three green/red blinks at ≤64/255 via RMT NRZ; GPIO boards blink active-LOW
- **USB vs UART console** — UART bridge stays up; USB Serial/JTAG drops on sleep
- **No sdkconfig** — board differences live in Cargo features + `src/board.rs`

## Limitations

- MQTT is unencrypted QoS 0. Use `secure-ota-https` patterns for TLS.
- Wi-Fi is 2.4 GHz only on these C6 boards.
- XIAO onboard BOOT (GPIO 9) cannot wake from deep sleep.
- Placeholder `[env]` values let the crate build; they will not associate with a real AP until overridden.

## Power Budget

| Mode | Typical current | Duration |
| --- | --- | --- |
| Active (CPU + Wi-Fi STA + MQTT) | ~80 mA | ~1 s typical (≤ 15 s timeout) |
| Deep sleep (RTC timer + GPIO 0 + persistent RAM) | ~7–15 µA | 15 s |
| **Target average (fast associate)** | **< 5 mA** | ~16 s period |

`(80 mA × 1 s + 0.015 mA × 15 s) / 16 s ≈ 5.0 mA`. A full 15 s AP timeout raises the average; that path is failure recovery.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (DevKitC-C6, default features)**

   From `examples/deep-sleep-bme280-mqtt-sensor/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings. Also confirms the radio git-pin resolves.

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

   Pass: image flashes; serial log shows first boot or a wake, then sleep entry. Use the DevKitC **left** USB-C UART-C port.

### Manual Tests — hardware required

**T-M1 — BME280 reads valid data**

Why manual: real I2C and sensor coefficients require physical hardware.

1. Wire BME280: VCC 3.3 V, GND, SDA/SCL to the board-spec I2C pins.
2. `cargo run --release`
3. Confirm temperature 10–40 °C, humidity 10–90 %, pressure **900–1100 hPa**. Serial `P=` and MQTT `pres` match at that scale (sea-level ≈ **1013.1**, not ≈10.1).
4. Breathe on the sensor — humidity should increase.

Pass: all three readings in range; humidity responds; serial hPa matches MQTT `pres`.

---

**T-M2 — MQTT publish reaches broker**

Why manual: end-to-end MQTT needs a live broker and AP.

1. Override compile-time env (see Credentials) and subscribe to `sdd/sensor/bme280`.
2. Flash and confirm well-formed JSON each cycle; `boot` increments; PSK is not printed.

Pass: broker receives JSON every cycle.

---

**T-M3 — Button wakeup (DevKitC GPIO 0)**

Why manual: physical RTC GPIO wakeup requires silicon.

1. During deep sleep, press the DevKitC BOOT button (GPIO 0), or an external active-LOW button on XIAO D0 (GPIO 0).
2. Confirm immediate wakeup and a full sense+publish cycle.

Pass: GPIO 0 wakes the device. (XIAO boot GPIO 9 cannot do this.)

---

**T-M4 — Active window timeout**

Why manual: requires deliberately invalid Wi-Fi credentials.

1. Configure a non-existent SSID (or leave `WIFI_SSID=unset` to skip).
2. Confirm the device logs a timeout/skip (no PSK) and **sleeps within 15 seconds of wake** — no hang. Stacked associate + DHCP + MQTT waits must not keep the chip awake past that Instant.

Pass: sleeps on network failure (or I2C timeout) within the 15 s whole-cycle window.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
