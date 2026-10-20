# Testing Specification — Deep Sleep BME280 + MQTT Sensor Node (ESP32-C6)

## Testing Philosophy

Automated testing using native Cargo / rustc tools is strongly preferred.
Additional tools are only allowed with explicit justification and documented benefits.
Manual testing is used only when no practical automated alternative exists.

Do not specify `idf.py`, ESP-IDF Unity, or CMake-based harnesses.

---

## Automated Tests

### T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)

**Tool**: `cargo` (native; rustup stable + `riscv32imac-unknown-none-elf` + `rust-src`)
**What it verifies**: The crate compiles cleanly for the default board (DevKitC-C6, WS2812 / RMT + RTC sleep + crates.io `esp-radio` `=1.0.0-beta.1` Wi-Fi) with no `rustc` or clippy warnings. Confirms the crates.io radio pin resolves.

```sh
cd examples/deep-sleep-bme280-mqtt-sensor
cargo build --release
cargo clippy --release -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)

**Tool**: `cargo` (native)
**What it verifies**: The GPIO LED + I2C + radio + RTC sleep path compiles when the XIAO board feature is selected.

```sh
cd examples/deep-sleep-bme280-mqtt-sensor
cargo build --release --no-default-features --features board-xiao-c6
cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A3 — Flash via espflash runner (DevKitC-C6)

**Tool**: `cargo run` with the crate runner (`espflash flash --monitor`)
**What it verifies**: The image flashes; serial log shows first boot or a wake, then either a sensor/MQTT cycle or a skipped-network path if credentials are placeholders, then sleep entry.

```sh
cd examples/deep-sleep-bme280-mqtt-sensor
cargo run --release
```

Pass criterion: flash succeeds; monitor shows boot/wake log and sleep entry (or a clean skip if `WIFI_SSID` is the documented placeholder).

Hardware required for the flash step. The *build* half of this crate is covered by T-A1 without a board.

Use the **left** USB-C UART-C port so the monitor stays connected through deep sleep. Native USB Serial/JTAG will drop the port on the first sleep; that is not a firmware failure.

---

## Manual Tests

*Manual tests are used only when no practical automated alternative exists. Each step explains
why automation is not applicable.*

### T-M1 — BME280 reads valid data

**Why manual**: Real I2C and sensor coefficients require physical hardware.

Hardware required: ESP32-C6-DevKitC-1-N8 or XIAO ESP32-C6, BME280 breakout at 3.3 V on the board-spec I2C pins.

Steps:

1. Wire BME280: VCC 3.3 V, GND, SDA/SCL to the board-spec I2C pins (DevKitC GPIO 6/7; XIAO GPIO 22/23).
2. From `examples/deep-sleep-bme280-mqtt-sensor/`: `cargo run --release` (add `--no-default-features --features board-xiao-c6` for XIAO).
3. Confirm temperature about 10–40 °C, humidity 10–90 %, pressure **900–1100 hPa**.
   Serial `P=` and MQTT `pres` must match at that hPa scale (sea-level ≈ **1013.1**, not
   ≈10.1).
4. Breathe on the sensor — humidity should increase.

Pass criterion: all three readings in range; humidity responds; serial hPa matches MQTT `pres`.

---

### T-M2 — MQTT publish reaches broker

**Why manual**: End-to-end MQTT needs a live broker and AP.

Hardware required: as T-M1 plus 2.4 GHz WPA2 AP and an MQTT 3.1.1 broker.

Steps:

1. Override compile-time env (see README): `WIFI_SSID`, `WIFI_PASSWORD`, `MQTT_BROKER_HOST`, `MQTT_BROKER_PORT`. Clean rebuild if `[env]` changed.
2. Run a broker (e.g. `mosquitto`) and subscribe to `sdd/sensor/bme280`.
3. Flash and confirm well-formed JSON each cycle; `boot` increments; PSK is not printed on serial.

Pass criterion: broker receives JSON every cycle; no secrets in the log.

---

### T-M3 — Button wakeup (DevKitC GPIO 0 / XIAO external GPIO 0)

**Why manual**: Physical RTC GPIO wakeup requires silicon.

Hardware required: DevKitC-C6 (BOOT = GPIO 0) or XIAO with an external active-LOW button on D0 (GPIO 0). Do **not** use the XIAO BOOT key (GPIO 9).

Steps:

1. During deep sleep, press the RTC wake button.
2. Confirm immediate wakeup and a full sense+publish (or skip) cycle.

Pass criterion: GPIO 0 wakes the device. GPIO 9 must not be required.

---

### T-M4 — Active window timeout

**Why manual**: Requires deliberately invalid Wi-Fi credentials (or a stuck I2C bus) and a wall clock.

Steps:

1. Set `WIFI_SSID` to a non-existent network (not the documented `unset` skip path if you want a real associate timeout; `unset` is also a valid skip).
2. Confirm the device logs a timeout/skip (no PSK) and **sleeps within 15 seconds of wake** — no hang. Sense + STA + MQTT + LED together must not stay awake past that Instant; stacked stage waits (associate + DHCP + MQTT) must not add up past 15 s.

Pass criterion: sleeps on network failure (or I2C timeout) within the 15 s whole-cycle window.
