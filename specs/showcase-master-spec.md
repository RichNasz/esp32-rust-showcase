# Showcase Master Specification

## Examples Catalog

1. blinky – breathing LED, multi-board (ESP32-C6-DevKitC-1-N8 + XIAO ESP32-C6)
2. deep-sleep-timer-node – RTC wakeup + persistence
3. deep-sleep-bme280-mqtt-sensor – real IoT sensor node
4. hardware-aes-secure-storage – crypto accelerator
5. ble-beacon-deep-sleep – duty-cycled BLE beacon (demo window, not a multi-year budget)
6. secure-ota-https – HTTPS download into the inactive OTA slot, CRC check, reboot. Not Secure Boot or flash-encryption enrollment
7. esp-now-low-power-mesh – ESP-NOW sensor-to-gateway unicast. The directory name is historical. This is not a multi-hop mesh
8. lp-core-wakeup – LP-core GPIO wakeup on RTC GPIO 0–7 (C6 has no capacitive touch)

## Implementation status

Catalog source of truth for what is actually in the tree. Checked 2026-10-20. **Firmware generated** means `examples/<name>/Cargo.toml` and `src/main.rs` exist. **Specs only** means the three human spec files exist and the crate does not. Neither label is a hardware pass/fail result.

| # | Example | Status | Note |
|---|---|---|---|
| 1 | blinky | Firmware generated | Crate present since the 2026-09-08 generation. Specs were not rewritten on 2026-10-20. |
| 2 | deep-sleep-timer-node | Firmware generated | Crate present since the 2026-09-08 generation. |
| 3 | deep-sleep-bme280-mqtt-sensor | Firmware generated | Crate present since the 2026-09-08 generation. |
| 4 | hardware-aes-secure-storage | Firmware generated | Crate present since the 2026-09-08 generation. Plaintext key demo, not production key storage. |
| 5 | ble-beacon-deep-sleep | Firmware generated | Crate present since the 2026-09-08 generation. 10 s / 10 s demo duty cycle, not a multi-year budget. |
| 6 | secure-ota-https | Specs only | Specs rewritten 2026-10-20 (HTTPS slot flip, CRC, no Secure Boot). No `Cargo.toml` or `src/`. |
| 7 | esp-now-low-power-mesh | Specs only | Specs rewritten 2026-10-20 (sensor-to-gateway unicast). Directory name is historical. No crate. |
| 8 | lp-core-wakeup | Specs only | Specs rewritten 2026-10-20 (C6 LP-core GPIO 0). No crate. The retired S3 touch spec is gone. |

Generated docs copy this table. They must not say all eight examples already contain Rust, and they must not describe examples 6–8 as a mesh, as signed production updates, or as capacitive touch.

## Deferred

IEEE 802.15.4 raw frames (`esp-radio` feature `ieee802154`) are supported on ESP32-C6 and are the next catalog candidate. They are not one of the eight v1 examples. Do not add Thread, Zigbee, a multi-hop ESP-NOW mesh, ESP32-S3 capacitive touch, or Secure Boot eFuse enrollment as v1 example specs.

## Hardware bar

ESP32-C6-DevKitC-1-N8 required; XIAO ESP32-C6 also supported. Other SoCs (ESP32, S3, C5, …) are catalog-only until a chip feature exists. Chip feature `esp32c6` only.

## Example Directory Structure

See `specs/example-project-structure-spec.md` for the permanent, immutable layout every example must follow.

## Skills

esp32-ansi-monitor-engineer, esp32-board-spec-generator, esp32-ble-beacon-engineer, esp32-deep-sleep-engineer, esp32-hardware-crypto-configurer, esp32-sdd-full-project-generator, esp32-sdd-documentation-generator, esp32-sdd-functional-spec-creator, esp32-sdd-project-validator, esp32-ws2812-led-engineer, esp32-lp-core-engineer
