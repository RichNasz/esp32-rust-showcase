<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Deep Sleep BME280 + MQTT — ESP32-C6

> BME280 readings over Wi-Fi MQTT, then deep sleep

## Implementation status

**Firmware generated.** Crate present since the 2026-09-08 generation. This label is not a hardware pass.

A spec is Markdown a person writes. The agent reads that Markdown and generates the crate from it. See [Using an AI coding agent](../../docs/guides/using-with-ai-agents.md).

## Overview

Each wake reads temperature, humidity, and pressure from a BME280 over I2C, joins Wi-Fi as a station, publishes one JSON MQTT 3.1.1 QoS 0 message, tears down `esp-radio`, blinks the LED, and returns to deep sleep. The whole cycle (sense + STA + MQTT + LED) shares one Embassy `Instant` deadline of **15 seconds from wake**. Stage waits use time remaining, so associate, DHCP, and MQTT cannot stack past 15 s. If the deadline expires, the firmware tears the radio down if it is up and sleeps. It does not hang.

Serial pressure is **hPa** at the same scale as MQTT `pres` (sea-level ≈ 1013.1, not ≈ 10.1). Compensation stores Pascal internally. Do not log Pascal, and do not log a 100×-smaller hPa figure.

The RTC timer (~15 s from sleep entry) is always armed. On DevKitC-C6, GPIO 0 (boot button, RTC-capable, active LOW) can wake immediately and runs the same pipeline. On XIAO ESP32-C6 the onboard boot button is GPIO 9, which is **not** an RTC GPIO. This firmware arms GPIO 0 (D0) for an external active-LOW button instead.

After a successful publish the DevKitC WS2812 blinks green three times (GRB, ≤64/255, RMT, 800 kHz NRZ). Failure blinks red three times, then the LED is dark for sleep. The XIAO LED (GPIO 15, active LOW) blinks three times on success and stays off, or uses a shorter distinct pattern, on failure. Both paths still sleep.

Pin numbers live in generated `src/board.rs`. Do not hard-code GPIOs in `main.rs`.

This example enables `esp-hal` feature **`unstable`** (radio, sleep / RTC, and RMT). Those APIs may change with `esp-hal` minor releases. **`esp-radio` is an unstable beta** (crates.io `=1.0.0-beta.1` with `esp-hal` `=1.2.1`, `esp-rtos` `=0.4.0`, and `esp-alloc` `=0.11.0`). Do not git-pin those HP crates.

## Prerequisites

- **ESP32-C6-DevKitC-1-N8** (default). Use the **left** USB-C port (UART bridge, UART-C) so the monitor stays connected through deep sleep. The right USB-C port is USB Serial/JTAG and drops on sleep.
- **Seeed XIAO ESP32-C6** (optional). USB-C, USB Serial/JTAG. The port drops when the chip sleeps.
- BME280 breakout at 3.3 V on the board's default I2C pins (BMP280 covers temperature and pressure only).
- A 2.4 GHz WPA2-Personal access point and an MQTT 3.1.1 broker (for example mosquitto, no TLS). A compile-only check does not need them.
- rustup **stable** (MSRV 1.95.0), component `rust-src`, target `riscv32imac-unknown-none-elf`, and `espflash` or `cargo-espflash`. Install steps: [Toolchain setup](../../docs/guides/01-toolchain-setup.md).

ESP-IDF is not a prerequisite. Do not install `espup` or the `esp` rustup channel.

## Supported Boards

Chip ≠ board: the v1 chip feature is `esp32c6` (the SoC), while boards are pins and USB via Cargo features, not `idf.py set-target`.

Wi-Fi, deep-sleep timer wakeup, I2C, and a user LED are required. Both of these boards are 2.4 GHz Wi-Fi 6 only.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

I2C (from the board specs): DevKitC default SDA/SCL = GPIO 6 / 7. XIAO D4/D5 = GPIO 22 / 23. The driver probes address 0x76, then 0x77.

RTC GPIOs on the ESP32-C6 are 0–7 only. GPIO 9 (XIAO boot button) cannot wake the chip from deep sleep. Do not use it as a wake source.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | Color: green = success, red = failure, off = sleep (WS2812) | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW | ~15 µA |

The UART bridge on the DevKitC **left** USB-C port stays connected through deep sleep. Native USB Serial/JTAG on the XIAO drops `/dev/cu.usbmodem*` and `espflash monitor` exits. That drop is expected, not a crash. Use the DevKitC UART-C port when the log must span sleep. Treat the XIAO as LED observation unless a single-cycle capture is enough. Skipping radio teardown before sleep leaves the current in milliamps, not the microamp floor above.

## Build & Flash

From `examples/deep-sleep-bme280-mqtt-sensor/`:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

Do not hand-edit `src/`. Behavior changes go in the spec files (see [Spec Files](#spec-files)).

### Credentials (compile-time env — not source literals)

Wi-Fi PSK and MQTT broker settings are not string literals in `src/`. They come from environment variables. Cargo `[env]` in `.cargo/config.toml` ships placeholders so the crate **builds** without a lab AP.

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

Add `--no-default-features --features board-xiao-c6` to that same command when the board is a XIAO. Or edit `.cargo/config.toml` `[env]`. Cargo may cache env values — **clean rebuild** (`cargo clean`) after changing `[env]`. Do not commit real PSKs. Serial logs never print the password.

MQTT stack: `embassy-net` TCP plus an in-crate MQTT 3.1.1 QoS 0 encoder (CONNECT → PUBLISH → DISCONNECT). That matches a duty-cycled sleep cycle better than a forever-running MQTT client crate, and it matches the `esp-hal` 1.2 `embassy_dhcp` networking path. No TLS. Heap (about 72 KiB internal DRAM) is for the Wi-Fi blob and `embassy-net` only. `build-std` includes `alloc`.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only** `examples/deep-sleep-bme280-mqtt-sensor/`, not the repository root, so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/deep-sleep-bme280-mqtt-sensor/`**

That loads this crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target and `espflash` runner). Do not open the repository root as the active workspace while developing this example.

## Expected behavior

Log shape (the numbers are an example of scale, not a recorded run):

```
deep_sleep_bme280_mqtt_sensor: Wake #42 — timer
deep_sleep_bme280_mqtt_sensor: T=22.5 C  H=48.2 %  P=1013.1 hPa
deep_sleep_bme280_mqtt_sensor: MQTT publish ok  topic=sdd/sensor/bme280
deep_sleep_bme280_mqtt_sensor: radio teardown; sleeping
deep_sleep_bme280_mqtt_sensor: Sleeping for 15 s
```

JSON payload shape: `{"temp":22.5,"hum":48.2,"pres":1013.1,"boot":42}` on topic `sdd/sensor/bme280`. Retain is false. Serial `P=` is hPa with one decimal, from the same conversion as `pres` (integer hPa = `Pa/100`, tenth = `(Pa/10)%10`).

If `WIFI_SSID` is the placeholder `unset`, the log skips MQTT and still sleeps. A missing sensor, stuck I2C bus, unreachable AP, or broker failure logs a recoverable error with no secrets and still sleeps inside the 15 s window. Association is capped at about 10 s or whatever remains of that window, whichever is shorter.

Persistent RAM (boot counter, magic + checksum) is retained across deep sleep and cleared on power-on. The counter increments on every wake, including a GPIO 0 wake.

## Key Concepts

- **Duty-cycled STA + MQTT** — one Instant, 15 s from wake (sense+STA+MQTT+LED); stage waits use remaining time so they cannot stack past 15 s; disconnect and drop `esp-radio` before sleep (including on deadline expiry)
- **Serial and MQTT pressure** — hPa, same conversion from Pascal (`Pa/100` and `(Pa/10)%10`). Logging `pressure_pa / 10000` is about 100× too small (sea-level ~10.1)
- **`esp-radio` Wi-Fi** — unstable beta; crates.io `esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1`, `esp-rtos` `=0.4.0`, and `esp-alloc` `=0.11.0`. Do not git-pin those HP crates. Features include `unstable` and `wifi`. APIs may change with `esp-hal` minor releases
- **`embassy-net` + MQTT 3.1.1 QoS 0** — one TCP publish (CONNECT → PUBLISH → DISCONNECT), no long-lived MQTT session crate, no TLS. `rust-mqtt` is MQTT 5 only and is not the stack here
- **Compile-time credentials** — `env!` / Cargo `[env]`; never log the PSK. Placeholder `unset` skips association and still sleeps
- **BME280 forced mode over I2C** — 400 kHz, pins from `src/board.rs`; probe 0x76 then 0x77; one-shot then standby. A stuck bus must time out (software transaction timeout and hardware `BusTimeout`) so it cannot consume the 15 s window. Skipping the conversion wait yields zeros that look like a wiring fault
- **Persistent boot counter** — `#[esp_hal::ram(unstable(rtc_fast, persistent))]` with magic + checksum
- **RTC GPIO wakeup** — C6 GPIO 0–7 only; GPIO 0 active LOW with pull-up on both boards; GPIO 9 is not RTC
- **WS2812 heartbeat** — three green blinks on success, three red blinks on failure, ≤64/255 via RMT NRZ; GPIO boards blink active LOW. RMT, sleep, and radio are **unstable** APIs
- **USB vs UART console** — UART bridge stays up; USB Serial/JTAG drops on sleep
- **No sdkconfig** — board differences live in Cargo features + `src/board.rs`. The radio pin is crates.io (`esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1`, `esp-rtos` `=0.4.0`, `esp-alloc` `=0.11.0`). Do not git-pin those HP crates.

## Power Budget

Sleep floor from the board specs: ~7–15 µA on DevKitC with the RTC domain, ~15 µA on XIAO. Typical STA + MQTT active current is ~80 mA average (peaks higher during TX). Radio teardown (~5–10 ms) is part of the active window. The ~1 s figure is the typical active window. The 15 s figure is a hard timeout, not the expected duration.

| Mode | Typical current | Duration | Energy per cycle |
| --- | --- | --- | --- |
| Active (CPU + Wi-Fi STA + MQTT) | ~80 mA | ~1 s typical (≤ 15 s timeout) | ~22 µAh |
| Radio teardown | included in active | ~5–10 ms | — |
| Deep sleep (RTC timer + GPIO 0 + persistent RAM) | ~7–15 µA | 15 s | ~0.06 µAh |
| **Total per duty cycle** | — | ~16 s | ~22 µAh |

**Average current (typical 1 s active):** `(80 mA × 1 s + 0.015 mA × 15 s) / 16 s ≈ 5.0 mA` — target **< 5 mA** when association is fast. A full 15 s timeout on a missing AP raises the average substantially. That path is failure recovery, not the happy-path budget.

**Sleep wakeup source:** RTC timer + RTC GPIO 0. **Wakeup interval:** 15 seconds from sleep entry.

## Limitations

- MQTT is unencrypted QoS 0. Use `secure-ota-https` patterns for TLS.
- Wi-Fi is 2.4 GHz only on these C6 boards.
- XIAO onboard BOOT (GPIO 9) cannot wake from deep sleep.
- Dummy compile-time `[env]` values let the crate **build**; they will not associate with a real AP until the operator overrides them.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (DevKitC-C6, default features)**

   From `examples/deep-sleep-bme280-mqtt-sensor/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This also confirms the crates.io radio pin resolves. Real Wi-Fi secrets are not required in source.

2. **T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This checks the GPIO LED, I2C, radio, and RTC sleep path.

3. **T-A3 — Flash via espflash runner**

   ```sh
   cargo run --release
   ```

   Pass criterion: the image flashes, and the serial log shows first boot or a wake, then either a sensor/MQTT cycle or a skipped-network path if credentials are placeholders, then sleep entry. Use the DevKitC **left** USB-C UART-C port so the monitor stays up through sleep. Native USB Serial/JTAG drops the port on the first sleep; that is not a firmware failure. A board is required for the flash half. T-A1 covers the build without a board. These commands are the check, not a recorded hardware pass.

### Manual — hardware required

**T-M1 — BME280 reads valid data**

Why automation is not practical: real I2C and the sensor's coefficients need the physical part.

Hardware required: ESP32-C6-DevKitC-1-N8 or XIAO ESP32-C6, and a BME280 breakout at 3.3 V on the board-spec I2C pins.

1. Wire the BME280: VCC 3.3 V, GND, SDA/SCL to the board-spec I2C pins (DevKitC GPIO 6/7; XIAO GPIO 22/23).
2. From `examples/deep-sleep-bme280-mqtt-sensor/`: `cargo run --release` (add `--no-default-features --features board-xiao-c6` for XIAO).
3. Confirm temperature about 10–40 °C, humidity 10–90 %, and pressure **900–1100 hPa**. Serial `P=` and MQTT `pres` must match at that hPa scale (sea-level ≈ **1013.1**, not ≈ 10.1).
4. Breathe on the sensor. Humidity should increase.

Pass criterion: all three readings are in range, humidity responds, and serial hPa matches MQTT `pres`.

**T-M2 — MQTT publish reaches broker**

Why automation is not practical: end-to-end MQTT needs a live broker and a real access point.

Hardware required: the same board and sensor as T-M1, plus a 2.4 GHz WPA2 AP and an MQTT 3.1.1 broker.

1. Override compile-time env (see [Credentials](#credentials-compile-time-env--not-source-literals)): `WIFI_SSID`, `WIFI_PASSWORD`, `MQTT_BROKER_HOST`, `MQTT_BROKER_PORT`. Clean-rebuild if `[env]` changed.
2. Run a broker (for example `mosquitto`) and subscribe to `sdd/sensor/bme280`.
3. Flash and confirm well-formed JSON each cycle. `boot` increments. The PSK is not printed on serial.

Pass criterion: the broker receives JSON every cycle, and the log contains no secrets.

**T-M3 — Button wakeup (DevKitC GPIO 0 / XIAO external GPIO 0)**

Why automation is not practical: a physical RTC GPIO wake needs the silicon and a button.

Hardware required: DevKitC-C6 (BOOT = GPIO 0), or a XIAO with an external active-LOW button on D0 (GPIO 0). Do **not** use the XIAO BOOT key (GPIO 9).

1. During deep sleep, press the RTC wake button.
2. Confirm an immediate wakeup and a full sense-and-publish cycle, or the documented skip path if credentials are still placeholders.

Pass criterion: GPIO 0 wakes the device. GPIO 9 must not be required.

**T-M4 — Active window timeout**

Why automation is not practical: the check needs a deliberately bad Wi-Fi setup (or a stuck I2C bus) and a wall clock. A host build cannot show that the chip returned to sleep.

1. Set `WIFI_SSID` to a network that does not exist. Use the documented `unset` value only when you want the skip path; `unset` is also a valid skip, but it is not an associate timeout.
2. Confirm the device logs a timeout or skip, does not print the PSK, and **sleeps within 15 seconds of wake**. Sense + STA + MQTT + LED must not stay awake past that Instant. Stacked associate, DHCP, and MQTT waits must not add up past 15 s.

Pass criterion: the device sleeps on network failure (or an I2C timeout) inside the 15 s whole-cycle window. No hang.

## Spec Files

- [FunctionalSpec.md](specs/FunctionalSpec.md) states observable behavior.
- [CodingSpec.md](specs/CodingSpec.md) states architecture and constraints, with no code.
- [TestSpec.md](specs/TestSpec.md) states how to check the example.

Edits go in those files. Then ask the agent **Activate esp32-sdd-full-project-generator**. Type that phrase to the agent, not in the shell. See [Using an AI coding agent](../../docs/guides/using-with-ai-agents.md).
