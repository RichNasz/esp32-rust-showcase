<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-full-project-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# ESP-NOW Sensor to Gateway — ESP32-C6

> One sensor deep-sleeps and sends a single unicast datagram. One gateway stays awake and logs it. The directory name `esp-now-low-power-mesh` is historical. This is not a mesh.

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/` are in this folder. A clean host compile is not a two-board test.

`specs/showcase-master-spec.md` still lists this example as **Specs only**. Humans own that catalog. The FunctionalSpec in this folder repeats that older note.

`esp-hal` feature **`unstable`** is on because ESP-NOW, sleep, RTC, and RMT need it on the `esp-hal-v1.2.1` pin. Those APIs can change. **`esp-radio` is an unstable beta**, and the `esp-now` feature is unstable.

## Overview

One firmware image, two mutually exclusive Cargo features:

- `role-sensor` (default, with `board-devkitc-c6`) wakes on an RTC timer of about 15 seconds, and on RTC GPIO 0. It sends one ESP-NOW datagram to a compile-time gateway MAC, blinks, tears the radio down, and deep-sleeps. The active window is capped at about 3 seconds. The boot count lives in persistent RAM (magic plus checksum) and is lost on power-cycle.
- `role-gateway` never sleeps. It logs the station MAC at boot. Each datagram whose magic matches is logged (sender MAC and boot count) and blinks the LED once. Other frames are dropped with a warning.

Neither role associates to an access point. There is no relay, no beacon, no hop count, and no TCP stack. Both images must use the same `ESPNOW_CHANNEL` (default `1`). A channel mismatch looks like a dead gateway.

The placeholder gateway MAC `00:00:00:00:00:00` means "not configured". The sensor logs that, shows the failure indication, and sleeps. It does not register that address as a peer and it does not hang.

GPIO 9 (XIAO BOOT) cannot wake the sensor. GPIO 0 can: DevKitC BOOT, or an external button on XIAO D0.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Two ESP32-C6 boards for the on-air test (DevKitC-C6, XIAO C6, or one of each). One board can still build either role.

ESP-IDF is not required. Do not install `espup` or the `esp` rustup channel. No access point is required.

## Build & Flash

From this directory. Default features are DevKitC-C6 plus `role-sensor`:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo clippy --release -- -D warnings
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

Set the same `ESPNOW_CHANNEL` on both images. Copy the gateway's logged station MAC into the sensor's `ESPNOW_GATEWAY_MAC`. A shell export overrides `[env]` in `.cargo/config.toml`. `build.rs` reruns when either variable changes.

```sh
ESPNOW_CHANNEL=1 \
ESPNOW_GATEWAY_MAC=AA:BB:CC:DD:EE:FF \
cargo run --release
```

Twelve hex digits without colons are also accepted. The all-zero MAC is the unconfigured sensor. Enabling both role features, or a board feature with neither role, fails at compile time. Enabling both board features, or neither board, fails at compile time.

DevKitC sensor logs and flash go through the **left** USB-C UART-C port so the monitor stays up across sleep. The gateway does not sleep, so either USB port can monitor it. XIAO and the DevKitC right port are USB Serial/JTAG: the sensor monitor exits on the first sleep. That is expected.

> **Never edit generated source files directly.** To change behaviour, update the specs and regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/esp-now-low-power-mesh/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

Sensor, placeholder MAC (default build), DevKitC left USB-C:

```
INFO ESP-NOW sensor — channel 1 — WS2812 RGB LED GPIO 8
INFO First boot — counter starts at 1 (resets on power-cycle)
INFO gateway MAC 00:00:00:00:00:00
WARN ESPNOW_GATEWAY_MAC is unset; skipping send
INFO Sleeping for 15 s
INFO Wake #2 — timer
```

Sensor, after a real gateway MAC, and the gateway on the same channel:

```
INFO ESP-NOW sensor — channel 1 — WS2812 RGB LED GPIO 8
INFO Wake #4 — timer
INFO gateway MAC AA:BB:CC:DD:EE:FF
INFO send ok  boot=4
INFO radio teardown
INFO Sleeping for 15 s
```

Gateway:

```
INFO ESP-NOW gateway — channel 1 — WS2812 RGB LED GPIO 8
INFO station MAC AA:BB:CC:DD:EE:FF
INFO set ESPNOW_GATEWAY_MAC on the sensor to that address
INFO rx src=11:22:33:44:55:66 boot=4
```

A frame without the payload magic logs `drop frame from … (magic mismatch)` and does not panic. The gateway does not print a sleep line.

## Key Concepts

- **Unicast pair** — one sender, one receiver, fixed channel, compile-time peer MAC. Not a routing protocol. The directory name is historical.
- **Radio teardown** — the sensor drops the ESP-NOW driver and the Wi-Fi controller before `sleep_deep`. ESP-NOW shares the 802.11 radio. There is no `embassy-net` and no AP association. Leaving the radio up keeps sleep current in milliamps.
- **Persistent boot count** — `#[esp_hal::ram(persistent)]` on the sensor only. Lost on power-cycle. Guarded by magic and a checksum.
- **Payload** — four-byte magic plus a little-endian boot count. Eight bytes, under the 250-byte ESP-NOW maximum.
- **Application descriptor** — `esp_app_desc!()` is required for current `espflash`. This is not an ESP-IDF install.
- **WS2812** — DevKitC GPIO 8 is RMT, GRB, 800 kHz NRZ, brightness cap 64/255. Green is a successful send, red is failure or a skipped MAC, blue is an accepted gateway frame. XIAO GPIO 15 is active-LOW: one blink or a double blink. A GPIO write on the DevKitC data pin stays dark.

## Power budget (sensor)

Gateway current is continuous and is not a battery budget. Planning figures from the FunctionalSpec. C6 sleep floor is about 7–15 µA on DevKitC with the RTC domain, and about 15 µA on XIAO, after the radio is torn down.

| Mode | Typical current | Duration | Energy per cycle |
|---|---|---|---|
| Active (CPU + ESP-NOW, no AP association) | ~80 mA | ~0.5 s typical (≤ 3 s cap) | ~11 µAh |
| Radio teardown | included in active | ~5–10 ms | — |
| Deep sleep (RTC timer + GPIO 0 + persistent RAM) | ~7–15 µA | 15 s | ~0.06 µAh |
| **Total per duty cycle** | — | ~15.5 s | ~11 µAh |

Average when the send finishes promptly: about 2.6 mA, target under 3 mA. Wakeup is the RTC timer (15 s from sleep entry) plus RTC GPIO 0. GPIO 9 is not a wake source.

## Limitations

| This example is NOT | Why |
|---|---|
| A multi-hop mesh | No relay and no routing table |
| Dynamic gateway discovery | The sensor address is compile-time. The gateway prints its MAC so you can copy it |
| A Wi-Fi scanner | No SSID RSSI scan |
| A light-sleep forwarder | The gateway is fully awake. The sensor uses deep sleep, not light-sleep relay |
| Encrypted ESP-NOW | Frames are plaintext lab traffic |
| An access-point client | No association, no DHCP, no TCP |

IEEE 802.15.4 raw frames are `ieee802154-raw-frames`, not a feature of this example.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

From `examples/esp-now-low-power-mesh/`:

1. **T-A1 — DevKitC sensor (default)** — `cargo build --release` and `cargo clippy --release -- -D warnings` exit 0.
2. **T-A2 — DevKitC gateway** — the same two commands with `--no-default-features --features board-devkitc-c6,role-gateway`.
3. **T-A3 — XIAO, both roles** — `board-xiao-c6,role-sensor` and `board-xiao-c6,role-gateway`, each with `--no-default-features`. Build and clippy, four commands.
4. **T-A4 — roles are exclusive** — a build with both role features, and a build with the board feature and no role, must fail to compile. The default build still succeeds.

### Manual Tests — hardware required

Why manual: delivery and deep-sleep retention need two radios and a real RTC domain.

**T-M1 — Two DevKitC boards.** Flash the gateway first (`--no-default-features --features board-devkitc-c6,role-gateway`) and copy its station MAC. Flash the sensor with that MAC and the same channel (`cargo run --release` after exporting `ESPNOW_GATEWAY_MAC` and `ESPNOW_CHANNEL`). Use the sensor's left USB-C port. Watch at least five wakes. The gateway count matches the sensor and the counts step by 1. The sensor LED flashes once (green, cap 64/255) and is dark during the ~15 s sleep. The monitor stays connected.

**T-M2 — Unconfigured sensor.** Default MAC, no override. The log says the MAC is unset, the failure colour shows, and the device sleeps within about 3 seconds. The counter still increments. Repeat for two cycles.

**T-M3 — XIAO sensor (optional).** `cargo run --release --no-default-features --features board-xiao-c6,role-sensor` with a real gateway MAC. USB Serial/JTAG exits on the first sleep. That is expected. GPIO 15 blinks once per send the gateway accepted, and stays off during sleep.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)
