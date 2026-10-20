<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Matter Wi-Fi On/Off — ESP32-C6

> Matter on/off light over Wi-Fi, commissioned over BLE with test credentials. Not Matter over Thread and not a certified product.

## Implementation status

**Firmware generated.** Crate generated 2026-10-20 (Matter on/off over Wi-Fi, BLE commissioning, test credentials, RAM key store). Not Matter over Thread and not CSA-certified. A host build is not a commissioning pass.

A spec is Markdown a person writes; the agent generates the crate. See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

`esp-radio` is an unstable beta (crates.io `=1.0.0-beta.1` with `esp-hal` `=1.2.1`, `esp-rtos` `=0.4.0`, `esp-alloc` `=0.11.0`, and `esp-bootloader-esp-idf` `=0.6.0`). Do not git-pin those HP crates. `esp-hal` feature `unstable` is required for Wi-Fi, BLE, and RMT. Features `wifi`, `ble`, and `coex` are on. `ieee802154` is off. These APIs can change with the next `esp-hal` minor.

The Matter integration crate is git `rs-matter-embassy` at rev `2a806b0b1b521293b34a773618499e87402c843c`. crates.io `rs-matter-embassy` 0.0.0 is an empty placeholder. This is not the ESP-IDF `esp-matter` SDK.

The key store is RAM. Every reset clears fabrics, and the device must be commissioned again. Test attestation does not authenticate a production product.

On 2026-10-20 the DevKitC release link placed about 2.08 MiB in flash (ELF load span `0x215904` bytes, from `0x42000020` through `0x42215924`). The on-disk ELF is larger because it keeps symbols. That is over the 1 MB default factory slot. `partitions.csv` gives the factory app `0x2F0000` bytes (about 2.94 MiB) and the table ends at `0x300000`, which fits the XIAO's 4 MB flash. This is a host link, not a commissioning pass.

## Overview

One firmware image. The device commissions over BLE, then joins the Wi-Fi network the commissioner provides. The image does not embed a Wi-Fi password. Endpoint 1 is an on/off light. An on command lights the status LED. An off command turns it off. The LED starts dark.

DevKitC uses the WS2812: on is white at the brightness cap (64/255). XIAO uses the active-LOW GPIO LED. The boot log prints the Matter manual pairing code and the QR payload computed from the public `rs-matter` test commissioning data. Do not treat those credentials as a product identity.

Thread stays in `thread-udp-pair`. This example does not bring up OpenThread.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- ESP32-C6-DevKitC-1-N8 (default) or XIAO ESP32-C6
- A Matter commissioner (Home Assistant or a phone) and a 2.4 GHz Wi-Fi network for the on-device check. A host build does not need them.
- See [Toolchain setup](../../docs/guides/01-toolchain-setup.md).

ESP-IDF is not required. Do not install `espup` or the `esp` rustup channel. There is no `idf.py` and no `sdkconfig`.

## Supported Boards

The chip feature `esp32c6` is the SoC. Board features select pins. This is not ESP-IDF `idf.py set-target`.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

A GPIO level alone produces no visible output on the DevKitC — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

## Per-Board Behavior

The device stays awake. This table is the LED only.

| Board | LED feedback |
| --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | WS2812: dark at start and while off. On is white at brightness cap 64/255 |
| Seeed XIAO ESP32-C6 | GPIO 15, active LOW: dark at start and while off. On drives the LED |

The first-time difference that matters is the status LED. DevKitC GPIO 8 is a WS2812: a GPIO level alone stays dark. XIAO GPIO 15 is a simple active-LOW LED with no colour.

## Build & Flash

From this folder:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

`cargo run --release` is DevKitC-C6.

XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

Selecting both board features, or neither, fails at compile time.

The runner is `espflash flash --monitor --partition-table partitions.csv`. The factory app slot is `0x2F0000` bytes at `0x10000`. The table ends at `0x300000`. That fits the XIAO's 4 MB flash and the DevKitC's 8 MB flash. This example does not use the `dual-ota` profile and does not implement Matter OTA.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder** (not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/matter-wifi-onoff/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target + `espflash` runner). Do not open the top-level repository folder as your active workspace while developing this example.

## Expected behavior

These lines are the shape the firmware logs. They are not a recorded commissioning pass.

DevKitC at boot:

```text
INFO - Matter on/off light — WS2812 RGB LED GPIO 8
INFO - manual pairing code …
INFO - qr payload MT:…
INFO - Matter stack started. RAM key store: recommission after every reset
```

XIAO logs `GPIO LED (active-LOW)` and GPIO 15.

After a commissioner has joined the device to Wi-Fi, an on command logs `light on` and the LED lights. An off command logs `light off` and the LED goes dark. The LED task runs while the operational network is up.

Read the pairing code from the log. It is computed from the test commissioning data. A reset clears the RAM store, so the commissioner must pair the board again.

## Key Concepts

- Matter over Wi-Fi with concurrent BLE commissioning (`run_coex`). Not Matter over Thread. Not a certified product.
- git `rs-matter-embassy` rev `2a806b0b1b521293b34a773618499e87402c843c`, default features off, features `esp`, `log`, `trouble`, `embassy-net`, and `rustcrypto`.
- `tinyrlibc` 0.5, default features off, features `memchr`, `utoa`, and `strtoul`.
- Radio pin: crates.io `esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1` (features `wifi`, `ble`, `coex`; not `ieee802154`), `esp-rtos` `=0.4.0`, `esp-alloc` `=0.11.0`, `esp-bootloader-esp-idf` `=0.6.0`. Do not git-pin those HP crates. No `esp-storage`: the key store is RAM.
- Test attestation only, the public constants shipped with `rs-matter`. Production Target: no.
- Endpoint 1 is an on/off light. The root endpoint carries the stack's system clusters. The attribute starts off and does not toggle on a timer.
- Rust heap is 100 KiB, split so the reclaimed DRAM region has its own allocator. `build.rs` does not invoke CMake. It emits `-Tlinkall.x` last.
- `esp_bootloader_esp_idf::esp_app_desc!()` so current `espflash` accepts the image. This is image metadata, not an ESP-IDF install.
- WS2812 via RMT on DevKitC (GRB, 800 kHz NRZ, brightness cap 64/255). XIAO GPIO 15 is active-LOW. Pins only in `src/board.rs`.

## Limitations

| Topic | What this example does |
| --- | --- |
| Matter over Thread | Not this example. Wi-Fi and 802.15.4 do not coexist on this pin |
| CSA certification | Test attestation only |
| Persistence | RAM store. Recommission after every reset |
| Alexa non-concurrent commissioning | This example uses concurrent BLE and Wi-Fi commissioning |
| Border router | Not used |

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

From `examples/matter-wifi-onoff/`:

1. **DevKitC (default).** `cargo build --release` and `cargo clippy --release -- -D warnings` exit 0 with no warnings. On 2026-10-20 both exited 0.
2. **XIAO.** `cargo build --release --no-default-features --features board-xiao-c6` and `cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings`. On 2026-10-20 the clippy command exited 0.
3. After a successful link, record the application size. The 2026-10-20 DevKitC release image uses about 2.08 MiB of flash, so `partitions.csv` is required. The factory slot is `0x2F0000` bytes and the table ends at `0x300000`.

`cargo run --release` is the on-device command. A host build does not flash and does not commission.

### Manual — hardware required

1. **T-M1 — Commissioner.** Why automation is impractical: commissioning needs a Matter controller (Home Assistant or a phone) and a Wi-Fi access point. Flash with `cargo run --release`. Read the manual pairing code from the log.
2. Commission the board. Remove any previous instance of this test device first, because the RAM store does not survive reset.
3. Turn the light on from the controller. The status LED lights (white on DevKitC, active-LOW on XIAO).
4. Turn the light off. The LED goes dark.
5. Reset the board. The log offers the pairing code again, and the controller must commission it as a new device.

## Spec Files

Edit these, then ask the agent `Activate esp32-sdd-full-project-generator` (typed to the agent, not a shell command). See [Using with AI agents](../../docs/guides/using-with-ai-agents.md).

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md) — observable behavior.
- [specs/CodingSpec.md](specs/CodingSpec.md) — architecture and constraints, with no code.
- [specs/TestSpec.md](specs/TestSpec.md) — how to check.
