# Coding Specification — Thread UDP Pair (ESP32-C6)

## Architecture

One crate, two compile-time roles, `role-child` (default) and `role-leader`.
`compile_error!` if both are set or neither is. Board features stay mutually
exclusive the same way.

Embassy entry from `esp-rtos` after `esp_hal::init`, executor on TIMG0 plus the
CPU software interrupt. Initialize a small Rust heap. OpenThread keeps its own
internal heap. Branch on the role feature. Neither role sleeps.

Top-level wiring lives in `src/main.rs`. Pins live only in `src/board.rs`. A radio
module owns the dataset, the role bring-up, and the UDP loop. A frame module owns
the payload magic so both roles share one definition. The status LED is its own
module.

## Why this radio stack

Thread on this pin is crates.io `openthread` 0.4.0 over `esp-radio`'s IEEE 802.15.4
driver. Copy the crates.io radio pin from `shared-specs/rust-toolchain.md`
(`esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1`, `esp-rtos` `=0.4.0`,
`esp-alloc` `=0.11.0`, `esp-bootloader-esp-idf` `=0.6.0`). Enable `ieee802154`.
Do not enable `wifi`. Do not git-pin the HP radio crates.

`openthread` features for both roles: `esp-radio`, `isupper`, `log`, and `udp`.
The leader also enables `ftd`. The child does not, so it links the prebuilt MTD
archive. `tinyrlibc` 0.5, default features off, with `utoa` and `strtoul`, supplies
the C string symbols that archive calls.

`ftd` is not the prebuilt feature profile. `openthread-sys` then compiles
OpenThread. The host needs Clang, CMake, and Ninja. This crate's `build.rs` does
not shell out to CMake. It still emits the final linker script last.

`esp-hal` feature `unstable` is required. `esp-radio` stays an unstable beta. Say
that in the README.

## Dataset

`THREAD_DATASET` is a compile-time environment value with a lab placeholder in
`.cargo/config.toml`. Both roles load that active dataset and then enable IPv6 and
Thread. Do not treat the placeholder as a secret. Do not invent a second dataset
per role.

## UDP behaviour

One fixed UDP port. The leader binds the unspecified IPv6 address and waits until
the device role is leader before treating a datagram as success. The child waits
until it is attached as a child, then sends one short payload about every 2 seconds
to the Thread realm-local all-nodes address. A send or receive error is logged and
the loop continues. Do not grow an unbounded queue.

## Randomness and identity

OpenThread needs a cryptographic RNG. Keep the TRNG entropy source alive for the
life of the image. The extended address comes from that RNG. Settings storage is
RAM-only, so a reset rejoins from the compiled dataset.

## Heap

The Rust heap is small. OpenThread's internal heap is the large buffer. Do not
allocate image-sized buffers in application code. `build-std` includes `alloc`.

## Board abstraction

`src/board.rs` exposes LED GPIO, kind, and polarity. No wake pin. Application code
must not hardcode pin numbers. Chip feature is `esp32c6` only.

- **WS2812** (`board-devkitc-c6`): RMT 800 kHz NRZ, GRB, cap 64/255, reset in the
  RMT buffer. Three green flashes on leader success, three blue on child send.
  Start dark.
- **GPIO LED** (`board-xiao-c6`): active LOW. Three blinks for both success cases.

## Logging and errors

INFO for role, channel, PAN, and each accepted payload. WARN when the role is not
yet leader or child. ERROR for a transmit or OpenThread bring-up failure, then
keep running or halt if the stack cannot start. No `unwrap` or `expect` on fallible
HAL, radio, or OpenThread calls.

## Out of scope

No border router, no Matter, no Zigbee, no Wi-Fi, no deep sleep, no CSL.
