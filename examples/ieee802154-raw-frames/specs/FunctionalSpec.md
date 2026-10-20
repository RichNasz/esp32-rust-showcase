# Raw IEEE 802.15.4 Frames (ESP32-C6)

## Overview

Two-role rust-on-metal IEEE 802.15.4 link for ESP32-C6 (`no_std` `esp-hal`). One firmware
image, two mutually exclusive Cargo features. The **node** sends one short data frame
about every 2 seconds to a fixed peer short address and reports whether a MAC ACK came
back. The **peer** stays in receive, auto-acknowledges frames addressed to it, logs
frames that carry this example's magic, echoes that payload, and blinks. Chip is
ESP32-C6 only.

This example is raw MAC data frames. It is **not** Thread, **not** Zigbee, and **not**
a mesh. There is no network layer, no association handshake, and no Wi-Fi.

## Implementation status

**Firmware generated** as of 2026-10-20. These specs are the current design (node/peer
raw frames). The crate is produced by `esp32-sdd-full-project-generator`. A clean host
build is not a two-board on-air pass. The catalog table in `specs/showcase-master-spec.md`
is the source of truth across examples.

## Target Hardware

v1 chip: **ESP32-C6** only (`esp32c6`). Other SoCs are not in v1.

| Item | v1 value |
| --- | --- |
| **SoC** | ESP32-C6 only |
| **Chip Cargo feature** | `esp32c6` |
| **rustc target** | `riscv32imac-unknown-none-elf` |

### Board Compatibility Checklist

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [x] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add boards for any SoC other than ESP32-C6.

## Features

- Two compile-time roles, not a runtime menu and not Kconfig:
  - `role-node` (default, together with `board-devkitc-c6`)
  - `role-peer`
  - Enabling both, or neither, is a compile error.
  - Peer on DevKitC: `cargo run --release --no-default-features --features board-devkitc-c6,role-peer`
  - XIAO node: `cargo run --release --no-default-features --features board-xiao-c6,role-node`
  - XIAO peer: `cargo run --release --no-default-features --features board-xiao-c6,role-peer`
- Both roles use the same PAN and the same 802.15.4 channel (11 through 26), from
  compile-time environment `IEEE802154_CHANNEL` (placeholder default `15`) and
  `IEEE802154_PAN_ID` (placeholder default `0x4242`). A channel or PAN mismatch fails
  silently at the radio; the README must say that.
- Short addresses are fixed by role so two boards need no MAC paste:
  - node `0x2222`, peer of the node `0x2323`
  - peer `0x2323`, peer of the peer `0x2222`
- Neither role brings up Wi-Fi, BLE, or ESP-NOW. The 802.15.4 driver does not coexist
  with Wi-Fi.

### Node

- Stay awake. About every **2 seconds**, transmit one data frame to the peer short
  address with MAC ACK requested.
- Payload is a fixed magic plus one sequence byte. It must fit in one 802.15.4 data
  frame. The sequence increments and wraps.
- Wait about **200 ms** for an ACK. On ACK, three success blinks. On no ACK, three
  failure blinks. Do not hang.
- If an echoed payload with the same magic is queued, log it (sequence, RSSI when the
  driver provides it). An echo is not a second blink.
- A transmit error is logged and shown as the failure blink. The loop continues.

### Peer

- Stay awake in receive, with hardware auto-ACK for frames addressed to this short
  address.
- On a frame whose payload magic matches, log the sequence and RSSI, blink three times,
  and echo that payload back to the node short address. The echo does not request ACK,
  so the two boards do not ping-pong acknowledgements.
- Drop other frames with a warning, not a panic. A decode error is a warning. The
  receive loop continues.

### Shared

- Pin numbers live only in generated `src/board.rs`.
- Serial log at INFO (`log` + `esp-println`): role, channel, PAN, local short address,
  transmit or receive result.
- LED:
  - **DevKitC-C6:** WS2812 GPIO 8, RMT, GRB, cap 64/255. Node success is three green
    flashes. Node failure is three red flashes. Peer accept is three blue flashes.
    The LED starts dark.
  - **XIAO:** GPIO 15 active LOW. Three blinks for node success and for peer accept.
    Node failure is three slower blinks so it is distinct without colour. No RMT.
- Neither role sleeps, so USB Serial/JTAG (DevKitC right port; XIAO) and the DevKitC
  left UART-C port both stay usable for the monitor.

## Wakeup Strategy

Not applicable. Neither role enters deep sleep.

Radio and RMT APIs require `esp-hal` feature `unstable`.

## Connectivity

IEEE 802.15.4 raw data frames only. One PAN, one channel, two short addresses. No
Thread, no Zigbee, no Wi-Fi association, no MQTT, no BLE, no ESP-NOW.

This is a **radio** example: git-pin `esp-hal`, `esp-radio`, `esp-rtos`, `esp-alloc`,
and `esp-bootloader-esp-idf` from tag `esp-hal-v1.2.1`. Enable the `ieee802154` feature
on `esp-radio` (unstable). Do not enable `wifi`. Never mix crates.io `esp-rtos = "0.4.0"`
with git `esp-hal`. `esp-radio` is an unstable beta.

## Security Requirements

Not production-targeted. Frames are not encrypted and not authenticated beyond the MAC
checksum the radio already applies. Do not put secrets in the payload. The PAN id and
short addresses are lab constants, not credentials.

## Hardware Dependencies

- Board-spec: `board-specs/espressif/esp32-c6-devkitc-1-n8.md` (WS2812 GPIO 8).
- Board-spec: `board-specs/seeed/xiao-esp32c6.md` (user LED GPIO 15 active LOW).
- Two C6 boards for the on-air test (one node, one peer). Any mix of DevKitC-C6 and
  XIAO C6 is fine if both images use the same channel and PAN. A single board can still
  build either role.
- No sensor breakout and no access point.

The generator auto-invokes `esp32-ws2812-led-engineer` for the DevKitC WS2812 RMT path.

## Power Budget

Not a battery example. Both roles keep the 802.15.4 radio in receive-when-idle. Expect
tens of milliamps continuously. There is no deep-sleep target.

## Limitations

| This example is NOT | Why |
|---|---|
| Thread or Zigbee | No network layer and no stack above raw MAC data frames |
| A multi-hop mesh | One node, one peer, no relay and no routing |
| Encrypted 802.15.4 | No auxiliary security header. Lab plaintext |
| A Wi-Fi client | The pinned driver does not coexist with Wi-Fi |
| A sleepy sensor | Both roles stay awake so the link is easy to watch on a serial monitor |
| A sniffer | Promiscuous capture of unrelated PAN traffic is out of scope |

## Success Criteria

- `cargo build --release` (default: DevKitC-C6 + `role-node`) completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf`.
- `cargo clippy --release -- -D warnings` is clean for that default.
- `cargo build --release --no-default-features --features board-devkitc-c6,role-peer` and the matching clippy invocation are clean.
- The two XIAO feature sets (`board-xiao-c6,role-node` and `board-xiao-c6,role-peer`, each with `--no-default-features`) build and clippy clean.
- Building with both role features, or with neither, fails at compile time.
- **`cargo run --release`** on a DevKitC-C6 node, with a peer running the same channel and PAN: the node logs an incrementing sequence, the peer logs that sequence, and the node shows the success indication when an ACK arrives. The peer LED blinks on each accepted frame.
- With no peer powered, the node keeps looping, logs the missing ACK, shows the failure indication, and does not hang.
- No stack overflow across 10 node periods with the peer powered.
