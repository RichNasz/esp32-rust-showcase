<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Raw IEEE 802.15.4 Frames — ESP32-C6

> Raw IEEE 802.15.4 data frames between a node and a peer. Not Thread, not Zigbee, not a mesh.

## Implementation status

**Firmware generated.** Crate generated 2026-10-20 (raw 802.15.4 node/peer). Not Thread, Zigbee, or a mesh. A host build is not a two-board on-air pass.

A spec is Markdown a person writes; the agent generates the crate. See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

`esp-radio` is an unstable beta (in-tree `1.0.0-beta.0` at git tag `esp-hal-v1.2.1`). `esp-hal` feature `unstable` is required for the radio and for RMT. The `ieee802154` feature is unstable. These APIs can change with the next `esp-hal` minor.

## Overview

One firmware image, two compile-time roles. This is raw MAC data frames. There is no Thread stack, no Zigbee stack, no network layer, no association handshake, no Wi-Fi, and no multi-hop routing. The 802.15.4 driver on this pin does not coexist with Wi-Fi, so the `wifi` feature is not enabled.

- `role-node` (default, with `board-devkitc-c6`) stays awake. About every 2 seconds it sends one 2003 data frame to short address `0x2323` and waits about 200 ms for a MAC ACK. The payload is 6 bytes: a fixed magic plus one sequence byte. The sequence increments and wraps. Three success blinks on ACK, three failure blinks when the ACK is missing. An echoed payload with the same magic is logged (sequence, and RSSI when the driver provides it) and is not a second blink. A transmit error is the failure blink. The loop does not hang.
- `role-peer` stays awake on short address `0x2323`, with hardware auto-ACK for frames addressed to it. A frame whose magic matches is logged, blinked, and echoed to `0x2222`. The echo does not request ACK, so the two boards do not ping-pong acknowledgements. Other frames are dropped with a warning, not a panic.

Short addresses are fixed by role (`0x2222` node, `0x2323` peer) so two boards need no MAC paste. Both stay on one PAN and one channel (`IEEE802154_CHANNEL` default `15`, range 11 through 26; `IEEE802154_PAN_ID` default `0x4242`). A channel or PAN mismatch fails silently at the radio and looks like a missing ACK.

Neither role sleeps.

DevKitC status is an RMT WS2812 on GPIO 8 (GRB, 800 kHz NRZ, brightness cap 64/255): three green flashes on node ACK, three red flashes when the ACK is missing, three blue flashes on the peer. The LED starts dark. A GPIO level on GPIO 8 stays dark. XIAO status is GPIO 15, active LOW: three blinks for node success or peer accept, three slower blinks for a missing ACK. No RMT.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- ESP32-C6-DevKitC-1-N8 (default) or XIAO ESP32-C6
- A second C6 board for the on-air check. One board can still build either role.
- See [Toolchain setup](../../docs/guides/01-toolchain-setup.md).

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

Neither role sleeps, so this table is the LED only. The DevKitC UART bridge and USB Serial/JTAG (DevKitC right port, and XIAO) both stay usable for the monitor.

| Board | LED feedback |
| --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | WS2812: three green flashes on node ACK, three red flashes when the ACK is missing, three blue flashes on the peer. Off at start |
| Seeed XIAO ESP32-C6 | On / off, active LOW: three blinks for node success or peer accept, three slower blinks for a missing ACK |

The first-time difference that matters is the status LED. DevKitC GPIO 8 is a WS2812: a GPIO level alone stays dark, and colour is the pass, fail, or peer signal. XIAO GPIO 15 is a simple active-LOW LED with no colour, so a missing ACK is three slower blinks rather than red.

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

Both boards must use the same `IEEE802154_CHANNEL` (11 through 26) and `IEEE802154_PAN_ID`. Defaults live in `.cargo/config.toml`. A shell export overrides them. A mismatch looks like a missing ACK: the sender keeps looping and the peer stays quiet. The short addresses are fixed (`0x2222` node, `0x2323` peer). Enabling both role features, or a board feature with neither role, fails at compile time.

> **Never edit generated source files directly.** Change behaviour in the spec files below, then regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder** (not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/ieee802154-raw-frames/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target + `espflash` runner). Do not open the top-level repository folder as your active workspace while developing this example.

## Expected behavior

Node, peer powered, same channel and PAN. RSSI and LQI numbers vary:

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

XIAO logs `GPIO LED (active-LOW)` instead of the WS2812 line. These lines are the spec's observable shape. They are not a recorded two-board on-air pass.

## Key Concepts

- `esp-radio` IEEE 802.15.4 driver (`ieee802154` feature), git tag `esp-hal-v1.2.1`, with `esp-hal`, `esp-rtos`, `esp-alloc`, and `esp-bootloader-esp-idf` from that same tag. Do not enable `wifi`. Do not mix crates.io `esp-rtos` 0.4.0 with git `esp-hal`.
- `esp-radio` is an unstable beta. `esp-hal` feature `unstable` is required for the radio and for RMT.
- Raw 2003 data frames and hardware auto-ACK. Payload is 6 bytes (fixed magic plus one sequence byte). No Thread, no Zigbee, and no mesh.
- Embassy on TIMG0. The radio stays up (`rx_when_idle`). This example does not deep-sleep.
- Heap for driver payloads (`esp-alloc`, about 72 KiB). `build-std` includes `alloc`.
- WS2812 via RMT on DevKitC, GPIO on XIAO, pins only in `src/board.rs`.
- `esp_bootloader_esp_idf::esp_app_desc!()` so current `espflash` accepts the image. This is image metadata, not an ESP-IDF install.

## Power budget

Not a battery example. Both roles keep the 802.15.4 radio in receive-when-idle. Expect tens of milliamps continuously. There is no deep-sleep target and no measured current in this spec.

## Limitations

| This example is NOT | Why |
| --- | --- |
| Thread or Zigbee | No network layer and no stack above raw MAC data frames |
| A multi-hop mesh | One node, one peer, no relay and no routing |
| Encrypted 802.15.4 | No auxiliary security header. Lab plaintext |
| A Wi-Fi client | The pinned driver does not coexist with Wi-Fi |
| A sleepy sensor | Both roles stay awake so the link is easy to watch on a serial monitor |
| A sniffer | Promiscuous capture of unrelated PAN traffic is out of scope |

Frames are not authenticated beyond the MAC checksum the radio already applies. The PAN id and short addresses are lab constants, not credentials.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

From `examples/ieee802154-raw-frames/`:

1. **T-A1 — DevKitC node (default).** `cargo build --release` and `cargo clippy --release -- -D warnings` exit 0 with no warnings. Default features are `board-devkitc-c6` and `role-node`.
2. **T-A2 — DevKitC peer.** `cargo build --release --no-default-features --features board-devkitc-c6,role-peer` and `cargo clippy --release --no-default-features --features board-devkitc-c6,role-peer -- -D warnings` exit 0 with no warnings. `--no-default-features` is required so `role-node` is not also enabled.
3. **T-A3 — XIAO, both roles.** The same build and clippy pair for `board-xiao-c6,role-node` and for `board-xiao-c6,role-peer`, each with `--no-default-features`. All four commands exit 0 with no warnings.
4. **T-A4 — Role guard.** `cargo build --release --no-default-features --features board-devkitc-c6,role-node,role-peer` fails to compile, and `cargo build --release --no-default-features --features board-devkitc-c6` fails to compile. The default build (T-A1) still succeeds.

### Manual — hardware required

1. **T-M1 — Two boards.** Why automation is impractical: ACK and echo need a second radio. No host tool speaks 802.15.4 to the board.
   - Flash a node (`cargo run --release` on DevKitC, or the XIAO node feature set) and a peer (DevKitC or XIAO peer feature set) on the same `IEEE802154_CHANNEL` and `IEEE802154_PAN_ID`.
   - Over about 20 seconds the node sequence increases by one each period, the peer logs it, the node shows the success pattern when an ACK arrives, and the peer blinks. The node log may also show an echoed payload. Neither side panics.
2. **T-M2 — Node alone.** Why automation is impractical: the missing-ACK path is a live radio timeout.
   - Flash only the node. Leave the peer unpowered. Watch at least three periods.
   - Each period logs a missing ACK, shows the failure pattern, and the node keeps looping.
3. **T-M3 — Channel mismatch.** Why automation is impractical: the radios drop the frame with no error code on the sender. That only shows up on the air.
   - Flash the peer on channel 15 and the node on a different channel in 11 through 26. Watch one node period.
   - The node shows the failure indication. The peer does not log the magic. Matching the channel again restores T-M1.

## Spec Files

Edit these, then ask the agent `Activate esp32-sdd-full-project-generator` (typed to the agent, not a shell command). See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md) — observable behavior.
- [specs/CodingSpec.md](specs/CodingSpec.md) — architecture and constraints, with no code.
- [specs/TestSpec.md](specs/TestSpec.md) — how to check.
