<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# ESP-NOW Sensor to Gateway — ESP32-C6

> ESP-NOW sensor-to-gateway unicast. The directory name is historical. This is not a multi-hop mesh.

## Implementation status

**Firmware generated.** Crate generated 2026-10-20 (sensor-to-gateway unicast). Directory name is historical. A host build is not an on-air pass.

A spec is Markdown a person writes; the agent generates the crate. See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

`esp-radio` is an unstable beta (in-tree `1.0.0-beta.0` at git tag `esp-hal-v1.2.1`). `esp-hal` feature `unstable` is required for ESP-NOW, sleep, RTC, and RMT. The `esp-now` feature is unstable. These APIs can change with the next `esp-hal` minor.

## Overview

One firmware image, two mutually exclusive Cargo features. The directory name `esp-now-low-power-mesh` is historical. There is no relay, no hop count, no beacon, and no multi-hop mesh.

- `role-sensor` (default, with `board-devkitc-c6`) wakes on an RTC timer of about 15 seconds, and on RTC GPIO 0. It sends one unicast ESP-NOW datagram to a compile-time gateway MAC, blinks, tears the radio down, and deep-sleeps. The active window is capped at about 3 seconds. The boot count lives in persistent RAM (magic plus checksum) and is lost on power-cycle.
- `role-gateway` never sleeps. It logs the station MAC at boot. Each datagram whose magic matches is logged (sender MAC and boot count) and blinks the LED once. Other frames are dropped with a warning. There is no forwarding and no scan.

Neither role associates to an access point. There is no TCP stack. Both images must use the same `ESPNOW_CHANNEL` (default `1`). A channel mismatch looks like a dead gateway: the radio fails silently.

The placeholder gateway MAC `00:00:00:00:00:00` means "not configured". The sensor logs that, shows the failure indication, and sleeps. It does not register that address as a peer and it does not hang.

Payload is a four-byte magic plus a little-endian boot count: eight bytes, under the 250-byte ESP-NOW maximum. The sensor waits about 200 ms for send status.

GPIO 9 (XIAO BOOT) cannot wake the sensor. GPIO 0 can: DevKitC BOOT, or an external button on XIAO D0. Do not hold DevKitC BOOT across reset (download strap).

DevKitC status is an RMT WS2812 on GPIO 8 (GRB, 800 kHz NRZ, brightness cap 64/255): one green flash on a successful send, red on failure or a skipped MAC, one blue flash when the gateway accepts a frame, black before sleep. A GPIO level on GPIO 8 stays dark. XIAO status is GPIO 15, active LOW: one blink on success or an accepted frame, a short double-blink on sensor failure, off during sleep.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Two ESP32-C6 boards for the on-air check (DevKitC-C6, XIAO C6, or one of each). One board can still build either role.
- Sensor logs that must survive sleep: DevKitC **left** USB-C (UART bridge). See [Toolchain setup](../../docs/guides/01-toolchain-setup.md).

ESP-IDF is not required. Do not install `espup` or the `esp` rustup channel. There is no `idf.py` and no `sdkconfig`. No access point is required.

## Supported Boards

The chip feature `esp32c6` is the SoC. Board features select pins. This is not ESP-IDF `idf.py set-target`.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

## Per-Board Behavior

The sensor sleeps. The gateway does not, so either USB port can monitor a gateway.

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected (left USB-C UART bridge) | WS2812: green = successful send, red = failure or skipped MAC, blue = accepted gateway frame, off = sleep | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW: one blink on success or accept, short double-blink on sensor failure, off during sleep | ~15 µA |

The first-time difference that matters is the sensor's serial port. On DevKitC-C6, use the **left** USB-C UART bridge: the monitor stays connected and is silent while the sensor sleeps. On XIAO, and on the DevKitC **right** USB-C port, USB Serial/JTAG drops when the chip sleeps and `cargo run` exits; the port comes back on wake. That is expected, not a crash. The floors above are after the radio is torn down. Leaving ESP-NOW up keeps sleep current in milliamps. GPIO 9 is not a wake source.

## Build & Flash

From this folder. Default features are DevKitC-C6 plus `role-sensor`:

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

Set the same `ESPNOW_CHANNEL` on both images. Copy the gateway's logged station MAC into the sensor's `ESPNOW_GATEWAY_MAC`. A shell export overrides `[env]` in `.cargo/config.toml`. `build.rs` reruns when either variable changes.

```sh
ESPNOW_CHANNEL=1 \
ESPNOW_GATEWAY_MAC=AA:BB:CC:DD:EE:FF \
cargo run --release
```

Twelve hex digits without colons are also accepted. The all-zero MAC is the unconfigured sensor. Enabling both role features, or a board feature with neither role, fails at compile time. Enabling both board features, or neither board, fails at compile time.

> **Never edit generated source files directly.** Change behaviour in the spec files below, then regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder** (not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/esp-now-low-power-mesh/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target + `espflash` runner). Do not open the top-level repository folder as your active workspace while developing this example.

## Expected behavior

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

A frame without the payload magic logs `drop frame from … (magic mismatch)` and does not panic. The gateway does not print a sleep line. These lines are the spec's observable shape. They are not a recorded on-air pass.

## Key Concepts

- **Unicast pair** — one sender, one receiver, fixed channel, compile-time peer MAC. Not a routing protocol. The directory name is historical.
- **Radio teardown** — the sensor drops the ESP-NOW driver and the Wi-Fi controller before `sleep_deep`. ESP-NOW shares the 802.11 radio. There is no `embassy-net` and no AP association. Leaving the radio up keeps sleep current in milliamps.
- **Persistent boot count** — `#[esp_hal::ram(persistent)]` (or the equivalent `#[ram(unstable(...))]` form) on the sensor only. Lost on power-cycle. Guarded by magic and a checksum.
- **Payload** — four-byte magic plus a little-endian boot count. Eight bytes, under the 250-byte ESP-NOW maximum. Send wait is about 200 ms.
- **Git pin** — `esp-hal`, `esp-radio`, `esp-rtos`, `esp-alloc`, and `esp-bootloader-esp-idf` from tag `esp-hal-v1.2.1`. Do not mix crates.io `esp-rtos` 0.4.0 with git `esp-hal`. Heap for the radio blob is about 72 KiB (`esp-alloc`). `esp-radio` is an unstable beta. `esp-hal` feature `unstable` and the `esp-now` feature are unstable.
- **Application descriptor** — `esp_app_desc!()` is required for current `espflash`. This is not an ESP-IDF install.
- **WS2812** — DevKitC GPIO 8 is RMT, GRB, 800 kHz NRZ, brightness cap 64/255. XIAO GPIO 15 is active-LOW. A GPIO write on the DevKitC data pin stays dark.

## Power budget

Sensor only. Gateway current is continuous and is not a battery budget. Planning figures from the FunctionalSpec. C6 sleep floor is about 7–15 µA on DevKitC with the RTC domain, and about 15 µA on XIAO, after the radio is torn down. ESP-NOW transmit is a short Wi-Fi-radio burst; ~80 mA average over the active window is the planning figure (peaks higher).

| Mode | Typical current | Duration | Energy per cycle |
| --- | --- | --- | --- |
| Active (CPU + ESP-NOW, no AP association) | ~80 mA | ~0.5 s typical (≤ 3 s cap) | ~11 µAh |
| Radio teardown | included in active | ~5–10 ms | — |
| Deep sleep (RTC timer + GPIO 0 + persistent RAM) | ~7–15 µA | 15 s | ~0.06 µAh |
| **Total per duty cycle** | — | ~15.5 s | ~11 µAh |

Average when the send finishes promptly: `(80 mA × 0.5 s + 0.015 mA × 15 s) / 15.5 s ≈ 2.6 mA`, target under 3 mA. Wakeup is the RTC timer (15 s from sleep entry) plus RTC GPIO 0. GPIO 9 is not a wake source. These are planning figures, not a measured on-air result.

## Limitations

| This example is NOT | Why |
| --- | --- |
| A multi-hop mesh | One sender, one receiver, no relay and no routing table |
| Dynamic gateway discovery | The sensor MAC is a compile-time address. The gateway prints its MAC so the operator can copy it |
| A Wi-Fi scanner | RSSI-of-nearby-SSIDs was the retired mesh demo. ESP-NOW and a scan fight over one radio, and this example does not scan |
| A light-sleep forwarder | Light-sleep power management of an always-on relay is not part of this `esp-rtos` design |
| Encrypted ESP-NOW | Frames are plaintext lab traffic |
| An access-point client | No association, no DHCP, no TCP |

Extension path: add a third role only after this pair is reliable on two boards. Raw IEEE 802.15.4 is `ieee802154-raw-frames`, not a feature of this example.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

From `examples/esp-now-low-power-mesh/`:

1. **T-A1 — DevKitC sensor (default).** `cargo build --release` and `cargo clippy --release -- -D warnings` exit 0 with no warnings. Default features are `board-devkitc-c6` and `role-sensor`.
2. **T-A2 — DevKitC gateway.** The same two commands with `--no-default-features --features board-devkitc-c6,role-gateway`. `--no-default-features` is required so `role-sensor` is not also enabled. Both exit 0 with no warnings.
3. **T-A3 — XIAO, both roles.** `board-xiao-c6,role-sensor` and `board-xiao-c6,role-gateway`, each with `--no-default-features`. Build and clippy, four commands. All exit 0 with no warnings.
4. **T-A4 — Roles are exclusive.** `cargo build --release --no-default-features --features board-devkitc-c6,role-sensor,role-gateway` fails to compile, and `cargo build --release --no-default-features --features board-devkitc-c6` fails to compile. The default build (T-A1) still succeeds.

### Manual — hardware required

1. **T-M1 — Two DevKitC boards.** Why automation is impractical: ESP-NOW delivery and deep-sleep retention need two radios and a real RTC domain. A host compile cannot deliver a frame.
   - Flash the gateway: `cargo run --release --no-default-features --features board-devkitc-c6,role-gateway`. Copy the logged station MAC.
   - Flash the sensor with that MAC and the same channel (`cargo run --release` after exporting `ESPNOW_GATEWAY_MAC` and `ESPNOW_CHANNEL`). Use the sensor's left USB-C port.
   - Watch at least five wakes. The gateway count matches the sensor and the counts step by 1. The sensor LED flashes once (green, cap 64/255) and is dark during the ~15 s sleep. The monitor stays connected.
2. **T-M2 — Unconfigured sensor.** Why automation is impractical: the skip path is a timed sleep on the board. The placeholder MAC is the default build, and the failure colour is visual.
   - Flash the default sensor image without overriding `ESPNOW_GATEWAY_MAC`.
   - The log says the MAC is unset, the failure colour shows, and the device sleeps within about 3 seconds. The counter still increments. Repeat for two cycles.
3. **T-M3 — XIAO sensor (optional).** Why automation is impractical: active-LOW polarity and the USB drop on sleep are visual and host-port behaviour.
   - `cargo run --release --no-default-features --features board-xiao-c6,role-sensor` with a real gateway MAC, same channel as T-M1.
   - USB Serial/JTAG exits on the first sleep. That is expected. GPIO 15 blinks once per send the gateway accepted, and stays off during sleep.

## Spec Files

Edit these, then ask the agent `Activate esp32-sdd-full-project-generator` (typed to the agent, not a shell command). See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md) — observable behavior.
- [specs/CodingSpec.md](specs/CodingSpec.md) — architecture and constraints, with no code.
- [specs/TestSpec.md](specs/TestSpec.md) — how to check.
