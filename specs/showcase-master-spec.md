# Showcase Master Specification

## Examples Catalog

One-liners below are copied into reader-facing tables. Write them for someone new to this repository. Name the visible behavior. Do not put SDD, skill, or spec in a one-liner. Do not claim a product mesh, a multi-year battery, Secure Boot, Zigbee, a certified Matter device, or a Thread border router. Example 10 may name a two-node Thread lab network. Example 11 may name Matter over Wi-Fi with test credentials.

1. blinky – breathing LED, multi-board (ESP32-C6-DevKitC-1-N8 + XIAO ESP32-C6)
2. deep-sleep-timer-node – RTC wakeup + persistence
3. deep-sleep-bme280-mqtt-sensor – BME280 readings over Wi-Fi MQTT, then deep sleep
4. hardware-aes-secure-storage – hardware AES and flash-backed storage (plaintext key demo)
5. ble-beacon-deep-sleep – duty-cycled BLE beacon (demo window, not a multi-year budget)
6. secure-ota-https – HTTPS download into the inactive OTA slot, CRC check, reboot. Not Secure Boot or flash-encryption enrollment
7. esp-now-low-power-mesh – ESP-NOW sensor-to-gateway unicast. The directory name is historical. This is not a multi-hop mesh
8. lp-core-wakeup – LP-core GPIO wakeup on RTC GPIO 0–7 (C6 has no capacitive touch)
9. ieee802154-raw-frames – raw IEEE 802.15.4 data frames between a node and a peer. Not Thread, not Zigbee, not a mesh
10. thread-udp-pair – two-node Thread lab network. A leader and a child exchange one UDP payload. Not a border router and not Zigbee
11. matter-wifi-onoff – Matter on/off light over Wi-Fi, commissioned over BLE with test credentials. Not Matter over Thread and not a certified product

## Implementation status

Catalog source of truth for what is actually in the tree. Rechecked 2026-10-20 after the crates for examples 6–8 landed. **Firmware generated** means `examples/<name>/Cargo.toml` and `src/main.rs` exist. **Specs only** means the three human spec files exist and the crate does not. Neither label is a hardware pass/fail result.

| # | Example | Status | Note |
|---|---|---|---|
| 1 | blinky | Firmware generated | Crate present since the 2026-09-08 generation. Specs were not rewritten on 2026-10-20. |
| 2 | deep-sleep-timer-node | Firmware generated | Crate present since the 2026-09-08 generation. |
| 3 | deep-sleep-bme280-mqtt-sensor | Firmware generated | Crate present since the 2026-09-08 generation. |
| 4 | hardware-aes-secure-storage | Firmware generated | Crate present since the 2026-09-08 generation. Plaintext key demo, not production key storage. |
| 5 | ble-beacon-deep-sleep | Firmware generated | Crate present since the 2026-09-08 generation. 10 s / 10 s demo duty cycle, not a multi-year budget. |
| 6 | secure-ota-https | Firmware generated | Crate generated 2026-10-20 (HTTPS slot flip, CRC, no Secure Boot, no stock-`espflash` auto-rollback). A host build is not a hardware pass. |
| 7 | esp-now-low-power-mesh | Firmware generated | Crate generated 2026-10-20 (sensor-to-gateway unicast). Directory name is historical. A host build is not an on-air pass. |
| 8 | lp-core-wakeup | Firmware generated | Crate generated 2026-10-20 (C6 LP-core GPIO 0–7). Not capacitive touch. A host build is not a board pass. |
| 9 | ieee802154-raw-frames | Firmware generated | Crate generated 2026-10-20 (raw 802.15.4 node/peer). Not Thread, Zigbee, or a mesh. A host build is not a two-board on-air pass. |
| 10 | thread-udp-pair | Firmware generated | Crate generated 2026-10-20 (FTD leader / MTD child, one UDP payload). Not a border router, not Zigbee, not Matter. A host build is not a two-board on-air pass. The leader role builds OpenThread from source. |
| 11 | matter-wifi-onoff | Firmware generated | Crate generated 2026-10-20 (Matter on/off over Wi-Fi, BLE commissioning, test credentials, RAM key store). Not Matter over Thread and not CSA-certified. A host build is not a commissioning pass. |

Generated docs copy this table. As of this recheck, all eleven rows are **Firmware generated** (eleven crates, zero specs-only). They must not describe examples 6–8 as a mesh, as signed production updates, or as capacitive touch. They must not describe `ieee802154-raw-frames` as Thread, Zigbee, or a mesh. They must not describe `thread-udp-pair` as a border router or as Matter. They must not describe `matter-wifi-onoff` as Matter over Thread or as a certified product. Where a row says Specs only, do not describe that example as if `cargo run` works. Where a row says Firmware generated, the crate is in the tree and that label is not a hardware pass. Each example `FunctionalSpec.md` repeats its row. If the two disagree, this table wins and the FunctionalSpec is updated to match.

## Deferred

Zigbee, a multi-hop ESP-NOW mesh, ESP32-S3 capacitive touch, Secure Boot eFuse enrollment, and Matter over Thread are not catalog examples. Raw IEEE 802.15.4 is example 9 (`ieee802154-raw-frames`). Thread is example 10 (`thread-udp-pair`), a two-node UDP lab network. Matter over Wi-Fi is example 11 (`matter-wifi-onoff`). Do not add the excluded designs as further example specs. Do not fold Thread or Matter into example 9.

## Hardware bar

ESP32-C6-DevKitC-1-N8 required; XIAO ESP32-C6 also supported. The board catalog is those two ESP32-C6 boards. Do not add other SoCs. Chip feature `esp32c6` only.

## Example Directory Structure

See `specs/example-project-structure-spec.md` for the permanent, immutable layout every example must follow.

## Skills

Generated skills tables copy this table. The purpose is one line a new reader can use. Do not invent a second purpose.

| Skill | What the agent does when you ask it to follow this skill |
|---|---|
| `esp32-sdd-functional-spec-creator` | Drafts a new `FunctionalSpec.md` with you |
| `esp32-sdd-full-project-generator` | Writes the `no_std` Cargo crate from the example specs |
| `esp32-sdd-documentation-generator` | Rewrites the Markdown docs from `specs/` |
| `esp32-board-spec-generator` | Writes a board spec from a vendor page or datasheet |
| `esp32-ws2812-led-engineer` | Applies the WS2812 timing and driver constraints |
| `esp32-ble-beacon-engineer` | Designs BLE advertising and radio teardown before sleep |
| `esp32-deep-sleep-engineer` | Designs sleep, wakeup, and the power budget |
| `esp32-hardware-crypto-configurer` | Configures hardware AES and flash-backed key storage |
| `esp32-sdd-project-validator` | Checks an example against the locked crate layout |
| `esp32-lp-core-engineer` | Writes the LP-core program and the HP-core handshake |
