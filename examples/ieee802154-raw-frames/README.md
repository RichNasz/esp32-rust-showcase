<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-full-project-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Raw IEEE 802.15.4 Frames — ESP32-C6

> A node and a peer exchange one raw 802.15.4 data frame. Not Thread, not Zigbee, and not a mesh.

## Implementation status

**Firmware generated** as of 2026-10-20. Crate generated 2026-10-20 (raw 802.15.4 node/peer). Not Thread, Zigbee, or a mesh. A host build is not a two-board on-air pass.

`esp-hal` feature **`unstable`** is required for the radio and for RMT. `esp-radio` is an **unstable beta** (in-tree `1.0.0-beta.0` at git tag `esp-hal-v1.2.1`). These APIs can change with the next `esp-hal` minor.

## Overview

One firmware image, two compile-time roles. The **node** (default) sends a 6-byte lab payload to short address `0x2323` about every 2 seconds and blinks when a MAC ACK comes back. The **peer** receives on short address `0x2323`, auto-acknowledges, logs the payload, echoes it, and blinks. Both stay awake on one PAN and one channel.

There is no Thread stack, no Zigbee stack, no Wi-Fi, and no multi-hop routing. The 802.15.4 driver on this pin does not coexist with Wi-Fi, so the `wifi` feature is not enabled.

DevKitC status is an RMT WS2812 (GRB, 800 kHz NRZ, brightness cap 64/255): three green flashes on ACK, three red flashes when the ACK is missing, three blue flashes on the peer. XIAO status is GPIO 15, active LOW: three blinks for success or accept, three slower blinks for a missing ACK.

## Prerequisites

- rustup **stable** (MSRV 1.95.0), component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- ESP32-C6-DevKitC-1-N8 (default) or XIAO ESP32-C6
- A second C6 board for the on-air check

ESP-IDF, `espup`, and the `esp` rustup channel are not used.

## Build & Flash

From this folder:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

`cargo run --release` is the node on DevKitC-C6 (channel 15, PAN `0x4242`).

Peer on DevKitC-C6:

```sh
cargo run --release --no-default-features --features board-devkitc-c6,role-peer
```

XIAO ESP32-C6 node:

```sh
cargo run --release --no-default-features --features board-xiao-c6,role-node
```

XIAO ESP32-C6 peer:

```sh
cargo run --release --no-default-features --features board-xiao-c6,role-peer
```

Both boards must use the same `IEEE802154_CHANNEL` (11 through 26) and `IEEE802154_PAN_ID`. Defaults live in `.cargo/config.toml`. A shell export overrides them. A mismatch looks like a missing ACK: the sender keeps looping and the peer stays quiet. The short addresses are fixed (`0x2222` node, `0x2323` peer).

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/ieee802154-raw-frames/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

Node, peer powered, same channel and PAN:

```text
INFO - 802.15.4 node — WS2812 RGB LED GPIO 8
INFO - WS2812 RMT init on GPIO 8 (GRB order, max brightness 64/255)
INFO - 802.15.4 up — channel 15 pan 0x4242 short 0x2222 peer 0x2323
INFO - node tx seq 0
INFO - node seq 0 ack
INFO - node frame seq 0 rssi -40 lqi 180
```

Node alone:

```text
INFO - node tx seq 0
INFO - node seq 0 no ack
```

Peer:

```text
INFO - 802.15.4 peer — WS2812 RGB LED GPIO 8
INFO - 802.15.4 up — channel 15 pan 0x4242 short 0x2323 peer 0x2222
INFO - peer rx seq 0 rssi -40 lqi 180
INFO - peer echo seq 0
```

RSSI and LQI numbers vary. XIAO logs `GPIO LED (active-LOW)` instead of the WS2812 line.

## Key Concepts

- `esp-radio` IEEE 802.15.4 driver (`ieee802154` feature), git tag `esp-hal-v1.2.1`, with `esp-hal`, `esp-rtos`, `esp-alloc`, and `esp-bootloader-esp-idf` from that same tag
- Raw 2003 data frames and hardware auto-ACK. No Thread and no Zigbee
- Embassy on TIMG0. The radio stays up; this example does not deep-sleep
- Heap for driver payloads (`esp-alloc`, about 72 KiB). `build-std` includes `alloc`
- WS2812 via RMT on DevKitC, GPIO on XIAO, pins only in `src/board.rs`
- `esp_bootloader_esp_idf::esp_app_desc!()` so current `espflash` accepts the image

## Limitations

| This example is NOT | Why |
|---|---|
| Thread or Zigbee | No network layer above raw MAC data frames |
| A multi-hop mesh | One node, one peer, no relay |
| Encrypted 802.15.4 | No auxiliary security header |
| A Wi-Fi client | This driver does not coexist with Wi-Fi |
| A sleepy sensor | Both roles stay awake |
| A sniffer | Frames without this example's magic are dropped |

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

From `examples/ieee802154-raw-frames/`:

1. **T-A1 — DevKitC node (default)** — `cargo build --release` and `cargo clippy --release -- -D warnings` exit 0.
2. **T-A2 — DevKitC peer** — `cargo build --release --no-default-features --features board-devkitc-c6,role-peer` and the matching `cargo clippy --release --no-default-features --features board-devkitc-c6,role-peer -- -D warnings` exit 0.
3. **T-A3 — XIAO, both roles** — the same build and clippy pair for `board-xiao-c6,role-node` and for `board-xiao-c6,role-peer`, each with `--no-default-features`. All four commands exit 0.
4. **T-A4 — role guard** — `cargo build --release --no-default-features --features board-devkitc-c6,role-node,role-peer` fails, and `cargo build --release --no-default-features --features board-devkitc-c6` fails. The default build still succeeds.

### Manual — hardware required

1. **T-M1 — two boards.** Flash a node and a peer on the same channel and PAN. Over about 20 seconds the node sequence increases, the peer logs it, the node shows the success pattern when an ACK arrives, and the peer blinks. A second radio is required; the host cannot answer 802.15.4.
2. **T-M2 — node alone.** With the peer off, each period logs a missing ACK, shows the failure pattern, and the node keeps looping. The timeout is on the air.
3. **T-M3 — channel mismatch.** Different channels produce the failure pattern on the node and silence on the peer. Matching the channel again restores T-M1. The radio drops the frame without an error code.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)
