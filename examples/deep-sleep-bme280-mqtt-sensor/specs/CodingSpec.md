<!-- See shared-specs/AIGenLessonsLearned.md — read before generating. -->

# Coding Specification — Deep Sleep BME280 + MQTT Sensor Node (ESP32-C6)

## Architecture

Single-shot Embassy firmware. Entry is async main from `esp-rtos`; after `esp_hal::init`,
start the Embassy executor on TIMG0 + `FROM_CPU_INTR0` (C6), initialize a heap (justified
below), then run one linear pipeline: persist boot count, log wakeup cause, read BME280,
bring up Wi-Fi STA + `embassy-net`, publish one MQTT message, tear the radio down, blink
the LED, arm wakeup sources, enter deep sleep. Deep sleep does not return; the chip resets
on wake.

Each pipeline stage has an independent timeout. Failure skips remaining network stages and
still sleeps. A 15-second wall-clock deadline covers the whole active window.

Top-level wiring lives in `src/main.rs`. Board GPIO, LED kind, polarity, I2C pins, and the
RTC wake pin live only in generated `src/board.rs`. Additional modules for the sensor,
MQTT framing, and status LED are allowed so no file exceeds ~500 lines.

## Board Abstraction

`src/board.rs` is generated from `board-specs/`. It exposes LED GPIO / kind / polarity,
I2C SDA and SCL, and the RTC wake GPIO (GPIO 0 on both boards). `compile_error!` if both
`board-devkitc-c6` and `board-xiao-c6` are enabled, or if neither is. Application code must
not hardcode pin numbers. Chip feature is `esp32c6` only.

Two compile-time LED paths:

- **WS2812** (default `board-devkitc-c6`): RMT 800 kHz NRZ, GRB, cap 64/255. Three green
  blinks on MQTT success, three red blinks on failure, black before sleep.
- **GPIO LED** (`board-xiao-c6`): active-LOW binary blinks. Three blinks on success; remain
  off (or a shorter distinct pattern) on failure. Do not run the RMT encoder on this board.

## Persistence

Store the boot counter in RTC-backed RAM that survives deep sleep and is lost on
power-cycle. Use `#[esp_hal::ram(persistent)]` or the equivalent
`#[ram(unstable(...))]` form required by the pinned `esp-hal` 1.2.1 proc-macro
(`persistent` is an unstable option and, on C6, belongs in RTC fast memory). Types must be
persistable bit-patterns.

Treat uninitialized or corrupted persistent RAM as first boot: distinctive magic plus a
checksum. Increment as the first application action after HAL/Embassy/heap init. Do not
use NVS or flash for this integer.

## Sleep and wakeup

Enable `esp-hal` feature `unstable` — sleep, RTC, RMT, and radio need it on this pin.

Arm an RTC timer wakeup for about 15 seconds from sleep entry (absolute deadline so work
between arming and sleep does not shorten sleep). Also arm RTC GPIO 0 as a low-level wake
with a pull-up and the low-power pad path (deep sleep powers down the HP GPIO peripheral).
GPIO 9 is not an RTC wakeup pin.

Enter deep sleep via the 1.2-line `LowPower` / `sleep_deep` API. Panic only if no wakeup
source is armed. Unrecognised wakeup causes log a warning and still run the pipeline.

**Tear down `esp-radio` before sleep.** Stop STA (disconnect) and drop the controller so
the radio is not left powered. Skipping teardown leaves sleep current in milliamps.

Drive the LED dark before sleep. Allow a brief UART drain after the last log line.

## Sensor

Talk to the BME280 with `esp-hal` I2C master at 400 kHz on the board-spec pins. Use
**forced** mode so the sensor is not converting during sleep. Prefer a small in-tree
driver (Bosch compensation formulas) over a heavyweight registry component; the old
ESP-IDF `bosch/bme280` component does not apply. Probe I2C 0x76 then 0x77. If the chip
id is missing, skip MQTT and sleep — do not hang on NAK.

## Radio, network, and MQTT

This is a **radio** example. Git-pin `esp-hal`, `esp-radio` (features `esp32c6`,
`unstable`, `wifi`), `esp-rtos` (features `esp32c6`, `embassy`, `esp-radio`), and
`esp-alloc` from tag `esp-hal-v1.2.1`. Never mix crates.io `esp-rtos = "0.4.0"` with git
`esp-hal`.

Wi-Fi STA uses `esp-radio` + `embassy-net` DHCPv4, matching the `esp-hal` 1.2
`embassy_dhcp` example (Interface + WifiController + net runner task). Heap is required
for the Wi-Fi blob and `embassy-net` (about 72 KiB internal DRAM). Do not use
`#[ram(reclaimed)]` unless the crate also emits an ESP-IDF-compatible app descriptor —
that attribute is rejected without it. `build-std` must include `alloc`. Do not allocate
after init except as the radio / net stack does internally.

**MQTT choice:** MQTT 3.1.1 QoS 0, single CONNECT → PUBLISH → DISCONNECT over an
`embassy-net` TCP socket. Do **not** pull a long-lived Embassy MQTT client (those assume
a forever-running session, which is a poor fit for duty-cycled deep sleep). `rust-mqtt`
is MQTT 5 only. A small in-crate encoder/decoder over TCP is the stack that actually
matches the FunctionalSpec and the `esp-hal` 1.2 embassy-net examples. No TLS. Document
this choice in the README.

JSON is formatted into a stack buffer (no serde). Payload shape is fixed: temp, hum,
pres, boot. Pressure in hPa.

## Credentials

Wi-Fi SSID/password, MQTT broker host, MQTT broker port, and MQTT client id are
**compile-time environment variables** (`env!` / Cargo `[env]` in `.cargo/config.toml`),
not string literals in `src/` and not sdkconfig/Kconfig. Placeholder `[env]` values exist
so `cargo build --release` succeeds without a lab AP; operators override them. Never log
the password or other secrets at any log level. A placeholder SSID such as `unset` must
skip association and still sleep.

## Key Constraints

- Sleep duration: 15 seconds from sleep entry.
- Hard active-window deadline: 15 seconds. Wi-Fi associate timeout: about 10 seconds.
- MQTT: QoS 0, retain false, one publish per cycle, MQTT 3.1.1, no TLS, no auth required
  (optional username/password are out of scope).
- Heap is justified for Wi-Fi + embassy-net only. No `std`. No FreeRTOS. No `sdkconfig`.
- Embassy entry: `#[esp_rtos::main] async fn main(spawner: Spawner) -> !`. Do not use
  blocking `#[esp_hal::main]`.
- `build.rs` emits `-Tlinkall.x` last.
- Zero `unwrap()` / `expect()` on fallible HAL / radio / net in the wake path. Init may
  halt if clocks, heap, or RMT cannot start.

## Preferred Crates and APIs

- Git-pinned `esp-hal` / `esp-radio` / `esp-rtos` / `esp-alloc` at `esp-hal-v1.2.1`
  (reason: radio pin; crates.io `esp-radio` does not resolve with HAL 1.2.1).
- `embassy-net` 0.9 with DHCPv4 + TCP (+ DNS only if the broker host is not a literal
  IPv4 address) (reason: same stack as the HAL 1.2 Wi-Fi examples).
- `esp-hal` I2C master for BME280 (reason: metal I2C; no IDF driver).
- `esp-hal` RTC / `LowPower` for timer + GPIO wakeup and deep sleep.
- `esp-hal` ram persistent attribute for the boot counter.
- `esp-hal` RMT for WS2812; `embassy-time` for blinks and timeouts.
- `log` + `esp-println` (`log-04`) for INFO operational events.

## Non-Functional Requirements

- Logging: INFO for first boot / timer / GPIO wake + count, sensor values, MQTT success
  (topic only), radio teardown, and sleep entry. WARN for timeouts and unexpected wakeup
  causes. ERROR for init failures. Never print SSID+password together; never print the PSK.
- Concurrency: one net-stack runner task plus main. Do not spawn reconnect loops that
  outlive the active window.
- Error recovery: any failed stage logs and continues to teardown + sleep.
- CPU clock: max is acceptable for the short Wi-Fi window (matches HAL radio examples).

## Gotchas

- Persistent RAM is retained across deep sleep but cleared on power-on. Checksum/magic is
  required because a reset can skip zero-init.
- GPIO 9 cannot wake C6 from deep sleep. GPIO 0 is the RTC wake pin on both boards; on
  XIAO that is D0 (external button), not the silkscreen BOOT key.
- A GPIO level on the DevKitC WS2812 produces no light. Cap 64/255; three blinks.
- XIAO LED is active LOW.
- DevKitC serial for persistence tests is the **left** USB-C UART-C port. Native USB
  Serial/JTAG drops the host CDC port on true deep sleep; `espflash monitor` exiting is
  expected.
- Sleep, RTC, RMT, and `esp-radio` live behind `unstable`. `esp-radio` is an unstable beta.
- Do not mix crates.io `esp-rtos` 0.4.0 with git `esp-hal`.
- `sleep_deep` panics if no wakeup source is enabled or if the timer deadline is already
  too near. Arm the 15 s deadline immediately before sleep, after radio teardown.
- Tear down Wi-Fi before sleep or the sleep floor is milliamps, not microamps.
- `env!` credentials: changing `.cargo/config.toml` `[env]` may require a clean rebuild
  (Cargo env-cache bug). Document that in the README.
- BME280 forced mode needs a short conversion wait; skipping it yields zeros that look
  like a wiring fault.
