# Thread UDP Pair (ESP32-C6)

## Overview

Two-role rust-on-metal Thread lab link for ESP32-C6 (`no_std` `esp-hal`). One firmware
image, two mutually exclusive Cargo features. The **leader** is a Full Thread Device.
It brings up a fixed lab dataset and waits until its role is leader. The **child** is
a Minimal Thread Device on the same dataset. After it attaches, it sends one short UDP
payload about every 2 seconds to the Thread realm-local all-nodes address. The leader
logs the payload and blinks. Chip is ESP32-C6 only.

This example is a two-node Thread network. It is not a Thread border router, not
Zigbee, not Matter, and not raw IEEE 802.15.4. Raw frames stay in
`ieee802154-raw-frames`. Wi-Fi stays off.

## Implementation status

**Firmware generated** as of 2026-10-20. The catalog row in
`specs/showcase-master-spec.md` is the source of truth. A clean host build is not a
two-board on-air pass. The leader role compiles OpenThread from C and needs Clang,
CMake, and Ninja on the host. The child role uses the prebuilt MTD archive.

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

## Features

- One image. Default features are `board-devkitc-c6` and `role-child`. `role-leader`
  is the other role. Selecting both roles, or neither, fails at compile time.
  `--no-default-features` drops the board and the role, so every non-default command
  names both.
- Both roles load the same lab Thread dataset from `THREAD_DATASET`. A placeholder in
  `.cargo/config.toml` lets `cargo build --release` run with no second board. A shell
  export overrides it. The dataset is a lab value, not a secret.
- The leader enables OpenThread's FTD build. The child stays on the default MTD
  prebuilt library.
- The child transmits one short payload to the realm-local all-nodes address on a
  fixed UDP port. The leader binds that port, logs a matching payload, and blinks.
- Neither role brings up Wi-Fi, BLE, ESP-NOW, or Matter.
- Status LED: three green flashes when the leader is leader and has logged a payload.
  Three blue flashes when the child has attached and sent. Start dark. DevKitC is the
  WS2812 on the board spec. XIAO is the active-LOW GPIO LED.

## Wakeup Strategy

Not applicable. Neither role sleeps. The radio stays up.

## Connectivity

Thread (OpenThread over IEEE 802.15.4). Copy the radio pin from
`shared-specs/rust-toolchain.md`, including `openthread` 0.4.0 and `tinyrlibc` 0.5.
Enable `esp-radio` feature `ieee802154`. Do not enable `wifi`.

## Security Requirements

None. The lab dataset is compiled in and is not a credential. There is no Matter
attestation and no application-layer encryption beyond what Thread applies to the
lab network. Production Target: no.

## Power

Not a battery example. Both roles keep the 802.15.4 radio in receive-when-idle.
Expect tens of milliamps continuously. There is no deep-sleep target.

## Limitations

| Topic | What this example does |
| --- | --- |
| Border router | Not implemented. OpenThread's binding lists it as a non-goal |
| Matter | Not this example. See `matter-wifi-onoff` |
| Zigbee | No Zigbee stack |
| Sleepy end device CSL | Not in `openthread` 0.4.0 |
| Wi-Fi | Left off. This `esp-radio` pin does not coexist 802.15.4 with Wi-Fi |
| Certified Thread product | Production Target: no |

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on
  `riscv32imac-unknown-none-elf` for the default child role, and for
  `role-leader` on a host that has Clang, CMake, and Ninja.
- `cargo clippy -- -D warnings` is clean for each role that was built.
- **`cargo run --release`** flashes the child via `espflash`.
- Leader: `cargo run --release --no-default-features --features board-devkitc-c6,role-leader`.
- Optional XIAO: add `board-xiao-c6` instead of `board-devkitc-c6`.
- On two boards with the same dataset, the leader log shows role leader, and a
  child payload is logged. The leader blinks for that payload. The child blinks
  after a send once it has attached.
