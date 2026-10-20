# Coding Specification — Matter Wi-Fi On/Off (ESP32-C6)

## Architecture

One crate, one role. Board features are mutually exclusive. Embassy entry from
`esp-rtos` after `esp_hal::init`, executor on TIMG0 plus the CPU software
interrupt.

The Matter stack is `rs-matter-embassy`'s Wi-Fi stack type, allocated statically
because the wireless stack requires that and because the object is too large for
the task stack. BLE and Wi-Fi run together for commissioning. A user task is not
required if the on/off handler drives the LED directly.

Top-level wiring lives in `src/main.rs`. Pins live only in `src/board.rs`. The
on/off logic and the status LED can live in `src/light.rs` when that keeps
`main.rs` to wiring.

## Why this stack

Matter over Wi-Fi on this pin is git `rs-matter-embassy` at the rev in
`shared-specs/rust-toolchain.md`, with default features off and features `esp`,
`log`, `trouble`, `embassy-net`, and `rustcrypto`. crates.io `rs-matter-embassy`
0.0.0 is empty. Do not use ESP-IDF or the C++ `esp-matter` SDK.

Copy the crates.io radio pin (`esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1`
with `wifi`, `ble`, and `coex`, `esp-rtos` `=0.4.0`, `esp-alloc` `=0.11.0`,
`esp-bootloader-esp-idf` `=0.6.0`). Do not enable `ieee802154`. Do not git-pin
the HP radio crates.

`tinyrlibc` 0.5, default features off, with `memchr`, `utoa`, and `strtoul`,
covers C string symbols pulled in by the stack.

`esp-hal` feature `unstable` is required. `esp-radio` stays an unstable beta.
Say that in the README. Raise the crate recursion limit enough for the Matter
handler types.

## Commissioning data

Use the test device attestation, commissioning, and DAC key that `rs-matter`
publishes for examples. Log the manual pairing code and the QR payload at INFO
during startup. Do not embed a Wi-Fi SSID or passphrase.

## Persistence

Use the stack's dummy RAM key store. Do not add `esp-storage` for this example.
Document that a reset clears fabrics.

## On/off and the LED

Endpoint 1 is an on/off light. The root endpoint uses the stack's root endpoint
and root handler. The on/off handler updates the status LED when the attribute
changes. On is lit. Off is dark. Start dark before the stack runs.

- **WS2812** (`board-devkitc-c6`): RMT 800 kHz NRZ, GRB, cap 64/255. On is white
  at the cap. Off is dark.
- **GPIO LED** (`board-xiao-c6`): active LOW. On drives the active level.

## Heap and flash

A heap is required for the Wi-Fi and BLE blobs and for the X.509 dependency.
About 100 KiB plus reclaimed DRAM is the starting budget. Do not allocate an
unbounded queue. `build-std` includes `alloc`.

If the linked application exceeds the 1 MB default factory slot, emit one custom
`partitions.csv` (single factory app, no OTA slots) that fits in 4 MB, and point
the espflash runner at it. Record the measured size in the README.

## Logging and errors

INFO for startup, the pairing code, and each on/off transition. ERROR if the
stack cannot start, then halt. No `unwrap` or `expect` on fallible HAL, radio,
or Matter startup calls.

## Out of scope

No Thread, no 802.15.4, no Matter OTA, no certified DAC, no flash persistence,
no deep sleep.
