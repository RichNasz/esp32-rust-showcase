<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Breathing LED — ESP32-C6

> breathing LED, multi-board (ESP32-C6-DevKitC-1-N8 + XIAO ESP32-C6)

## Implementation status

**Firmware generated.** Crate present since the 2026-09-08 generation. Specs were not rewritten on 2026-10-20. This label is not a hardware pass.

A spec is Markdown a person writes. The agent reads that Markdown and generates the crate from it. See [Using an AI coding agent](../../docs/guides/using-with-ai-agents.md).

## Overview

The onboard status LED breathes continuously. One breath is 4 seconds (2 s fade up, 2 s fade down). The sequence always starts dark and brightens first.

Cargo board features select the path (default `board-devkitc-c6`):

- **WS2812** (ESP32-C6-DevKitC-1-N8): the `esp-hal` RMT peripheral drives 800 kHz NRZ. Brightness walks a colour sequence — red, green, blue, cyan, magenta, amber — with each channel capped at 64/255. The colour advances at the start of each 4-second breath. Byte order is GRB.
- **GPIO** (XIAO ESP32-C6): a simple LED, active LOW, driven by LEDC PWM with Embassy-timed duty. No busy-wait. Duty is inverted so the breath still starts dark.

Pin numbers live in generated `src/board.rs`. Do not hard-code GPIOs in `main.rs`.

This crate enables `esp-hal` feature `unstable` because the RMT (WS2812) and LEDC APIs are unstable on the 1.2.1 pin and may change with `esp-hal` minor releases. It does not enter deep sleep and it does not use the radio. Non-radio pins are crates.io `esp-hal` 1.2.1 and `esp-rtos` 0.4.0. Do not mix those with a git-pinned `esp-hal`.

## Prerequisites

- **ESP32-C6-DevKitC-1-N8** (default). Use the **left** USB-C port (UART bridge, UART-C). The right USB-C port is USB Serial/JTAG.
- **Seeed XIAO ESP32-C6** (optional). USB-C, USB Serial/JTAG console.
- rustup **stable** (MSRV 1.95.0), component `rust-src`, target `riscv32imac-unknown-none-elf`, and `espflash` or `cargo-espflash`. Install steps: [Toolchain setup](../../docs/guides/01-toolchain-setup.md).

ESP-IDF is not a prerequisite. Do not install `espup` or the `esp` rustup channel.

## Supported Boards

Chip ≠ board: the v1 chip feature is `esp32c6` (the SoC), while boards are pins and USB via Cargo features, not `idf.py set-target`.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

## Per-Board Behavior

| Board | LED feedback |
| --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | WS2812 colour sequence (RMT, GRB, cap 64/255) |
| Seeed XIAO ESP32-C6 | On / off breathing, active LOW |

The DevKitC status LED is a WS2812: it needs an RMT (or equivalent `esp-hal`) driver for the 800 kHz NRZ waveform, and a GPIO level alone stays dark. The XIAO LED is a simple active-LOW GPIO on GPIO 15. It does not use RMT.

## Build & Flash

From `examples/blinky/`:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

Do not hand-edit `src/`. Behavior changes go in the spec files (see [Spec Files](#spec-files)).

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only** `examples/blinky/`, not the repository root, so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/blinky/`**

That loads this crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target and `espflash` runner). Do not open the repository root as the active workspace while developing this example.

## Expected behavior

### ESP32-C6-DevKitC-1-N8 (WS2812 RGB, GPIO 8)

```
blinky: Blinky — WS2812 RGB LED, GPIO 8, breathing period 4000 ms
blinky: WS2812 RMT channel init on GPIO 8 (GRB order, max brightness 64/255)
blinky: Starting WS2812 breathing loop
blinky: Cycle color 1/6  r=64 g=0 b=0
```

The RGB LED breathes through red, green, blue, cyan, magenta, amber, advancing at the start of each 4-second breath. It starts dark, then brightens.

### Seeed XIAO ESP32-C6 (active LOW, GPIO 15)

```
blinky: Blinky — GPIO LED (active-LOW), GPIO 15, breathing period 4000 ms
blinky: Board GPIO=15  polarity=active-LOW
blinky: Breathing period: 4000 ms
blinky: Starting GPIO LED breathing loop (LEDC PWM, Embassy-timed duty)
blinky: Fade up
blinky: Fade down
```

The blue user LED on GPIO 15 starts dark, then brightens, and breathes over about 4 seconds. The log is the GPIO / LEDC path, not WS2812 / RMT.

## Key Concepts

- **Standalone Cargo crate** — `#![no_std]` `#![no_main]`; Embassy entry via `esp-rtos`; no workspace at the repo root
- **Board features** — default `board-devkitc-c6`; optional `board-xiao-c6`; chip feature `esp32c6` only
- **`src/board.rs`** — GPIO / LED / polarity constants from `board-specs/`; `compile_error!` if both or neither board features are enabled
- **WS2812 via RMT** — GRB byte order, 800 kHz NRZ, brightness cap 64/255; `esp-hal` RMT is an **unstable** API and may change with `esp-hal` minor releases
- **LEDC PWM** — XIAO C6 inverts duty so the breath still starts dark and brightens first; LEDC is an **unstable** API and may change with `esp-hal` minor releases
- **Embassy time** — 4 s breath cadence with `embassy_time`; no busy-wait
- **Application descriptor** — `esp-bootloader-esp-idf` `esp_app_desc!()`; current `espflash` will not flash without it. This is image metadata, not ESP-IDF. A missing descriptor fails at `espflash` with "ESP-IDF App Descriptor missing" before the firmware runs.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (DevKitC-C6, default features)**

   From `examples/blinky/`:

   ```sh
   cargo build --release
   cargo clippy -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This checks the WS2812 / RMT path.

2. **T-A2 — Zero-warning build (XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This checks the GPIO / LEDC path.

3. **T-A3 — Flash via espflash runner**

   ```sh
   cargo run --release
   ```

   Pass criterion: the image flashes (no "ESP-IDF App Descriptor missing") and the serial log shows LED-driver init and breathing-loop start. The image must carry the application descriptor from `esp-bootloader-esp-idf`. A board is required for the flash half. T-A1 covers the build without a board. These commands are the check, not a recorded hardware pass.

### Manual — hardware required

**T-M1 — Visual WS2812 breathing (ESP32-C6-DevKitC-1-N8)**

Why automation is not practical: a WS2812 protocol error produces no light or a solid-on LED. A host build cannot certify perceptual smoothness or colour order.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8, USB-C on the **left** UART-C port (UART bridge; typically `/dev/cu.usbserial-*` on macOS).

1. From `examples/blinky/`: `cargo run --release`
2. Confirm the serial log mentions WS2812 / RMT init on GPIO 8, GRB order, and brightness cap 64/255.
3. Watch the onboard RGB LED.
4. Confirm it starts dark, then breathes through red, green, blue, cyan, magenta, and amber, about 4 seconds per cycle, with no stuck-on or stuck-off.

Pass criterion: the LED breathes smoothly through the colours, and the serial log confirms RMT init.

**T-M2 — Visual GPIO breathing (Seeed XIAO ESP32-C6)**

Why automation is not practical: active-LOW polarity and fade quality need a person looking at the LED.

Hardware required: Seeed XIAO ESP32-C6, USB-C cable.

1. From `examples/blinky/`: `cargo run --release --no-default-features --features board-xiao-c6`
2. Confirm the serial log shows the GPIO LED path (not WS2812 / RMT) on GPIO 15, active LOW.
3. Watch the blue user LED on GPIO 15.
4. Confirm it starts dark, then brightens, and breathes smoothly over about 4 seconds per cycle.

Pass criterion: correct polarity, smooth breathing, and no WS2812 init in the log.

## Spec Files

- [FunctionalSpec.md](specs/FunctionalSpec.md) states observable behavior.
- [CodingSpec.md](specs/CodingSpec.md) states architecture and constraints, with no code.
- [TestSpec.md](specs/TestSpec.md) states how to check the example.

Edits go in those files. Then ask the agent **Activate esp32-sdd-full-project-generator**. Type that phrase to the agent, not in the shell. See [Using an AI coding agent](../../docs/guides/using-with-ai-agents.md).
