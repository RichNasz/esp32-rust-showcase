<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Thread UDP Pair — ESP32-C6

> Two-node Thread lab network. A leader and a child exchange one UDP payload. Not a border router and not Zigbee.

## Implementation status

**Firmware generated.** Crate generated 2026-10-20 (FTD leader / MTD child, one UDP payload). Not a border router, not Zigbee, not Matter. A host build is not a two-board on-air pass. The leader role builds OpenThread from source.

A spec is Markdown a person writes; the agent generates the crate. See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

`esp-radio` is an unstable beta (crates.io `=1.0.0-beta.1` with `esp-hal` `=1.2.1`, `esp-rtos` `=0.4.0`, `esp-alloc` `=0.11.0`, and `esp-bootloader-esp-idf` `=0.6.0`). Do not git-pin those HP crates. `esp-hal` feature `unstable` is required for the 802.15.4 radio and for RMT. The `ieee802154` feature is unstable. These APIs can change with the next `esp-hal` minor. Wi-Fi stays off.

The default child role links the prebuilt OpenThread MTD archive from crates.io `openthread` 0.4.0. The leader role enables `openthread` feature `ftd`, which compiles OpenThread from C and needs Clang, CMake, and Ninja on the host. This crate's `build.rs` does not invoke CMake. On 2026-10-20 the child release ELF on disk was about 938 KiB, inside the default 1 MB factory slot. That measurement is a host link, not an on-air pass. The leader image was not linked on this host because Clang is present and CMake and Ninja are not.

## Overview

One firmware image, two compile-time roles. The child (default) loads a fixed lab Thread dataset and, after it attaches, sends one short UDP payload about every 2 seconds to the Thread realm-local all-nodes address `ff03::1` on port 1212. The payload bytes are `TUDP`. The leader loads the same dataset, waits until its role is leader, and logs a matching payload. Three green flashes on the leader when that payload arrives. Three blue flashes on the child after a send. The LED starts dark.

There is no Thread border router, no Zigbee stack, no Matter stack, and no raw-frame example folded in. Raw IEEE 802.15.4 stays in `ieee802154-raw-frames`. Settings are RAM only, so a reset rejoins from the compiled dataset.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- ESP32-C6-DevKitC-1-N8 (default) or XIAO ESP32-C6
- A second C6 board for the on-air check. One board can still build the child role.
- Clang, CMake, and Ninja only when you build `role-leader`
- See [Toolchain setup](../../docs/guides/01-toolchain-setup.md).

ESP-IDF is not required. Do not install `espup` or the `esp` rustup channel. There is no `idf.py` and no `sdkconfig`. No Wi-Fi access point is required.

## Supported Boards

The chip feature `esp32c6` is the SoC. Board features select pins. This is not ESP-IDF `idf.py set-target`.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

A GPIO level alone produces no visible output on the DevKitC — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

## Per-Board Behavior

Neither role sleeps, so this table is the LED only. The DevKitC UART bridge and USB Serial/JTAG (DevKitC right port, and XIAO) both stay usable for the monitor.

| Board | LED feedback |
| --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | WS2812: three green flashes when the leader logs a payload, three blue flashes when the child sends. Off at start. Brightness cap 64/255 |
| Seeed XIAO ESP32-C6 | On / off, active LOW: three blinks for a leader payload or a child send |

The first-time difference that matters is the status LED. DevKitC GPIO 8 is a WS2812: a GPIO level alone stays dark, and colour is the leader or child signal. XIAO GPIO 15 is a simple active-LOW LED with no colour.

## Build & Flash

From this folder:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

`cargo run --release` is the child on DevKitC-C6. Default features are `board-devkitc-c6` and `role-child`.

Leader on DevKitC-C6 (needs Clang, CMake, and Ninja):

```sh
cargo run --release --no-default-features --features board-devkitc-c6,role-leader
```

Child on XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6,role-child
```

Leader on XIAO:

```sh
cargo run --release --no-default-features --features board-xiao-c6,role-leader
```

`--no-default-features` drops both the board and the role, so every non-default command names both. Selecting both roles, or neither, fails at compile time.

Both roles read `THREAD_DATASET`. The placeholder in `.cargo/config.toml` is the lab dataset (network name `NEST-PAN-0FD8`). It is not a secret. A shell export overrides it. Flash both boards with the same value.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder** (not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/thread-udp-pair/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target + `espflash` runner). Do not open the top-level repository folder as your active workspace while developing this example.

## Expected behavior

These lines are the shape the firmware logs. They are not a recorded two-board on-air pass.

Child on DevKitC, after the dataset loads:

```text
INFO - Thread child — WS2812 RGB LED GPIO 8
INFO - Thread child dataset loaded channel … pan …
INFO - child bound UDP 1212, waiting to attach
```

Until the role is child, the log warns `not yet child`. After attach, a send logs `sent payload to [ff03::1]:1212` and the LED flashes three times (blue on DevKitC).

Leader:

```text
INFO - Thread leader — WS2812 RGB LED GPIO 8
INFO - Thread leader dataset loaded channel … pan …
INFO - leader bound UDP 1212, waiting for role leader
```

A matching payload logs `payload from …`. The leader LED flashes three times (green on DevKitC). Other datagrams are logged and ignored. A datagram that arrives before the role is leader is a warning, not a blink.

XIAO logs `GPIO LED (active-LOW)` instead of the WS2812 line.

## Key Concepts

- One image, mutually exclusive `role-child` (default, MTD) and `role-leader` (FTD). Both roles, or neither, fail at compile time.
- crates.io `openthread` 0.4.0 with features `esp-radio`, `isupper`, `log`, and `udp`. The leader feature also enables `openthread/ftd`. `ftd` is not the prebuilt profile, so the host needs Clang, CMake, and Ninja. `build.rs` only emits `-Tlinkall.x` last.
- `tinyrlibc` 0.5, default features off, features `utoa` and `strtoul`, for the C `str*` symbols the prebuilt library calls.
- Radio pin: crates.io `esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1` (feature `ieee802154`, not `wifi`), `esp-rtos` `=0.4.0`, `esp-alloc` `=0.11.0`, `esp-bootloader-esp-idf` `=0.6.0`. Do not git-pin those HP crates. `esp-radio` is an unstable beta.
- Lab dataset from `THREAD_DATASET`. RAM settings (`SimpleRamSettings`). A reset rejoins from the dataset. No border router.
- UDP port 1212. Child destination is realm-local all-nodes `ff03::1`. Payload is the four bytes `TUDP`.
- Rust heap is 8 KiB. OpenThread keeps its own buffers. The TRNG entropy source stays alive for the life of the image.
- `esp_bootloader_esp_idf::esp_app_desc!()` so current `espflash` accepts the image. This is image metadata, not an ESP-IDF install.
- WS2812 via RMT on DevKitC (GRB, 800 kHz NRZ, brightness cap 64/255). XIAO GPIO 15 is active-LOW. Pins only in `src/board.rs`.

## Limitations

| Topic | What this example does |
| --- | --- |
| Border router | Not implemented. The OpenThread binding lists it as a non-goal |
| Matter | Not this example. See `matter-wifi-onoff` |
| Zigbee | No Zigbee stack |
| Sleepy end device CSL | Not in `openthread` 0.4.0 |
| Wi-Fi | Left off. This `esp-radio` pin does not coexist 802.15.4 with Wi-Fi |
| Certified Thread product | Production Target: no |

Both roles keep the 802.15.4 radio in receive-when-idle. Expect tens of milliamps continuously. There is no deep-sleep target. The lab dataset is compiled in and is not a credential.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

From `examples/thread-udp-pair/`:

1. **Child, DevKitC (default).** `cargo build --release` and `cargo clippy --release -- -D warnings` exit 0 with no warnings. This role uses the prebuilt OpenThread archive and does not need CMake. On 2026-10-20 both commands exited 0.
2. **Child, XIAO.** `cargo build --release --no-default-features --features board-xiao-c6,role-child` and `cargo clippy --release --no-default-features --features board-xiao-c6,role-child -- -D warnings`. On 2026-10-20 the clippy command exited 0.
3. **Leader, DevKitC.** `cargo build --release --no-default-features --features board-devkitc-c6,role-leader` and the matching clippy invocation, on a host with Clang, CMake, and Ninja. Skip this step when those tools are absent, and say so. A skipped leader build is not a pass of the leader role. On 2026-10-20 this host had Clang and did not have CMake or Ninja, so the leader build was skipped.

`cargo run --release` is the on-device command. A host build does not flash.

### Manual — hardware required

1. **T-T1 — Two boards, one dataset.** Why automation is impractical: Thread attach and UDP delivery need a second 802.15.4 radio. Flash the leader with `role-leader` and the child with `role-child`. Both use the placeholder dataset unless you override `THREAD_DATASET` for both.
2. Confirm the leader log reaches role leader.
3. Confirm the child log reaches role child and then logs a send.
4. Confirm the leader logs the child payload and the leader LED flashes three times (green on DevKitC).
5. Confirm the child LED flashes three times after a send (blue on DevKitC).
6. Power-cycle either board. RAM settings are discarded. The board rejoins from the compiled dataset. There is no border router in this test.

## Spec Files

Edit these, then ask the agent `Activate esp32-sdd-full-project-generator` (typed to the agent, not a shell command). See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md) — observable behavior.
- [specs/CodingSpec.md](specs/CodingSpec.md) — architecture and constraints, with no code.
- [specs/TestSpec.md](specs/TestSpec.md) — how to check.
