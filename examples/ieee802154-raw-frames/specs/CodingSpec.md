<!-- See shared-specs/AIGenLessonsLearned.md — read before generating. -->

# Coding Specification — Raw IEEE 802.15.4 Frames (ESP32-C6)

## Architecture

One crate, two compile-time roles selected by mutually exclusive Cargo features
`role-node` and `role-peer`. Default features are `board-devkitc-c6` and `role-node`.
`compile_error!` if both roles are set or neither is. Board features stay mutually
exclusive the same way. `--no-default-features` drops both the board and the role, so
every non-default command must name the board feature and the role feature.

Embassy entry from `esp-rtos` after `esp_hal::init`, executor on TIMG0 + `FROM_CPU_INTR0`.
Initialize a heap, then branch on the role feature. Neither role sleeps.

Top-level wiring lives in `src/main.rs`. Pins live only in `src/board.rs`. A small frame
module owns the magic and the sequence byte so both roles share one payload definition.
A radio module owns channel, PAN, short addresses, and the send or receive loop.

## Why this radio stack

Raw 802.15.4 on this pin is `esp-radio`'s IEEE 802.15.4 driver, feature `ieee802154`,
git-pinned with `esp-hal`, `esp-rtos`, `esp-alloc`, and `esp-bootloader-esp-idf` at tag
`esp-hal-v1.2.1`. Do not enable the Wi-Fi feature: that driver does not coexist with
Wi-Fi. Do not add Thread or Zigbee crates.

MAC header types come from crates.io `ieee802154` 0.6.1, the same crate the pinned
`esp-radio` already uses. Disable its default features. Frames are 2003 data frames
with short addresses. The peer echo must not request acknowledgement.

`esp-hal` feature `unstable` is required. `esp-radio` stays an unstable beta. Say that
in the README.

## Heap

A heap is required. The driver's frame type stores the payload in an allocated buffer,
and the receive path decodes into that type. About 72 KiB is enough. There are no
image-sized buffers. Do not grow an unbounded queue in application code. The driver's
own receive queue stays at its default bound.

`build-std` includes `alloc` as well as `core`.

## Channel and PAN

`IEEE802154_CHANNEL` and `IEEE802154_PAN_ID` are compile-time environment values, with
placeholder defaults in `.cargo/config.toml` so `cargo build --release` works with no
second board. A shell export overrides the file. Reject a channel outside 11 through 26
at compile time. Reject a PAN that is not a 16-bit value at compile time. Do not treat
either value as a secret.

Short addresses are role constants, not environment variables. The node and the peer
are each other's destination.

## Timing and failure

The node period is about 2 seconds, including the blink. The ACK wait is about 200 ms.
Missing ACK and transmit errors use the failure indication and the loop continues.
Clear-channel assessment may be requested on transmit; do not assume the driver's
`Result` reports a busy channel. The observable failure is a missing ACK.

The peer polls rather than spinning. A non-matching payload or a decode error is a
warning. The peer echoes before it blinks so the echo is queued while the node is still
near that period. The node drains any echoed frame after the ACK wait and again after
the blink, and logs it without a second pattern.

## Board abstraction

`src/board.rs` exposes LED GPIO, kind, and polarity. No wake pin: this example does not
sleep. Application code must not hardcode pin numbers. Chip feature is `esp32c6` only.

- **WS2812** (`board-devkitc-c6`): RMT 800 kHz NRZ, GRB, cap 64/255, reset in the RMT
  buffer. Three green flashes on node success, three red on node failure, three blue on
  peer accept. Start dark. Log GPIO, GRB, and that RMT is driving NRZ.
- **GPIO LED** (`board-xiao-c6`): active LOW. Three blinks for success and accept. Node
  failure is three blinks with a clearly longer gap. No RMT.

## Logging and errors

INFO for role, channel, PAN, addresses, and each accepted exchange. WARN for dropped or
undecodable frames. ERROR for a transmit failure. No `unwrap` or `expect` on fallible
HAL or radio calls in the production path. Initialization may halt if the RMT channel
cannot be constructed. Do not discard a transmit `Result`.

## Application descriptor

Invoke `esp_bootloader_esp_idf::esp_app_desc!()` from `src/main.rs`. The bootloader
crate is the git pin, feature `esp32c6`. This is image metadata, not ESP-IDF.
