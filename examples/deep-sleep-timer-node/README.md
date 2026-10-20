<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Deep Sleep Timer Node — ESP32-C6

> RTC wakeup + persistence

## Implementation status

**Firmware generated.** Crate present since the 2026-09-08 generation. This label is not a hardware pass.

A spec is Markdown a person writes. The agent reads that Markdown and generates the crate from it. See [Using an AI coding agent](../../docs/guides/using-with-ai-agents.md).

## Overview

An RTC timer wakes the chip about every 5 seconds. Each wake increments a boot counter in persistent RAM (`#[esp_hal::ram(unstable(rtc_fast, persistent))]` on this HAL pin), logs the count and wakeup cause, blinks the onboard LED once, and returns to deep sleep. Persistent RAM survives deep sleep and is cleared on power-on. There is no sdkconfig RTC-memory option. A magic value plus a checksum treats random bytes after a raced reset as first boot, not a huge count.

On the DevKitC WS2812, each heartbeat is one colour drawn at random from the 6-colour palette (red, green, blue, cyan, magenta, amber) at a cap of 64/255. The XIAO path is a single active-LOW blink. The LED is off during sleep. The active window is about 200 ms. There is no radio.

Pin numbers live in generated `src/board.rs`. Do not hard-code GPIOs in `main.rs`.

Sleep, RTC, and RMT APIs need `esp-hal` feature `unstable`. Those APIs may change with `esp-hal` minor releases. This is a non-radio crate. Pins use an exact `=` so Cargo cannot select a later 1.2.x: crates.io `esp-hal` `=1.2.1`, `esp-rtos` `=0.4.0`, and `esp-bootloader-esp-idf` `=0.6.0`. Do not mix those with a git-pinned `esp-hal`.

GPIO 9 (XIAO boot button) is not an RTC GPIO and cannot wake the chip from deep sleep. This example is timer-only. DevKitC boot (GPIO 0) is RTC-capable and is not used as a wakeup source here.

## Prerequisites

- **ESP32-C6-DevKitC-1-N8** (default). Use the **left** USB-C port (UART bridge, UART-C) so the monitor stays connected through deep sleep. The right USB-C port is USB Serial/JTAG and drops on sleep.
- **Seeed XIAO ESP32-C6** (optional). USB-C, USB Serial/JTAG. The port drops when the chip sleeps.
- rustup **stable** (MSRV 1.95.0), component `rust-src`, target `riscv32imac-unknown-none-elf`, and `espflash` or `cargo-espflash`. Install steps: [Toolchain setup](../../docs/guides/01-toolchain-setup.md).

ESP-IDF is not a prerequisite. Do not install `espup` or the `esp` rustup channel. No external parts are required.

## Supported Boards

Chip ≠ board: the v1 chip feature is `esp32c6` (the SoC), while boards are pins and USB via Cargo features, not `idf.py set-target`.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

RTC GPIOs on the ESP32-C6 are 0–7 only. GPIO 9 (XIAO boot button) cannot wake the chip. This example arms an RTC timer only.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | One random WS2812 colour per wake (6-colour palette, cap 64/255); off during sleep | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW | ~15 µA |

The UART bridge on the DevKitC **left** USB-C port stays connected through deep sleep (silence while asleep, log lines again on wake). Native USB Serial/JTAG on the XIAO drops `/dev/cu.usbmodem*` and `espflash monitor` exits. That drop is expected, not a crash. Use the DevKitC UART-C port for counter-persistence tests. Treat the XIAO as LED-only observation unless a single-cycle capture is enough.

## Build & Flash

From `examples/deep-sleep-timer-node/`:

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

This folder is a complete standalone Cargo package. Open **only** `examples/deep-sleep-timer-node/`, not the repository root, so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/deep-sleep-timer-node/`**

That loads this crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target and `espflash` runner). Do not open the repository root as the active workspace while developing this example.

## Expected behavior

### First boot (ESP32-C6-DevKitC-1-N8, UART-C port)

```
deep_sleep_timer_node: First boot — counter starts at 1 (resets on power-cycle)
deep_sleep_timer_node: Timer node — WS2812 RGB LED, GPIO 8, sleep 5 s
deep_sleep_timer_node: WS2812 RMT init on GPIO 8 (GRB order, max brightness 64/255)
deep_sleep_timer_node: Heartbeat color 3/6  r=0 g=0 b=64
deep_sleep_timer_node: Sleeping for 5 s
```

### Subsequent wakeups (UART-C port)

```
deep_sleep_timer_node: Wake #2 — timer wakeup
deep_sleep_timer_node: Timer node — WS2812 RGB LED, GPIO 8, sleep 5 s
deep_sleep_timer_node: WS2812 RMT init on GPIO 8 (GRB order, max brightness 64/255)
deep_sleep_timer_node: Heartbeat color 1/6  r=64 g=0 b=0
deep_sleep_timer_node: Sleeping for 5 s
```

Persistent RAM is retained across deep sleep and cleared on power-on. Heartbeat colour comes from the hardware RNG each wake, so the palette index varies. The heartbeat is about 100 ms on and 100 ms off, then dark for sleep. An unrecognised wakeup cause logs a warning and still sleeps.

### Seeed XIAO ESP32-C6 (active LOW, GPIO 15)

```
deep_sleep_timer_node: First boot — counter starts at 1 (resets on power-cycle)
deep_sleep_timer_node: Timer node — GPIO LED (active-LOW), GPIO 15, sleep 5 s
deep_sleep_timer_node: GPIO LED heartbeat on GPIO 15 (GPIO LED (active-LOW))
deep_sleep_timer_node: Sleeping for 5 s
```

The USB Serial/JTAG monitor typically exits when the chip enters deep sleep. One brief blink per wake; the LED is off during the ~5 s sleep.

## Key Concepts

- **Standalone Cargo crate** — `#![no_std]` `#![no_main]`; Embassy entry via `esp-rtos`; no workspace at the repo root
- **`#[esp_hal::ram(unstable(rtc_fast, persistent))]`** — boot counter survives deep sleep; magic + checksum reject random bytes; types must be persistable. `persistent` is an unstable option
- **RTC timer wakeup** — `LowPower::set_wakeup_deadline` + `sleep_deep` (feature `unstable`); ~5 s from sleep entry. Those sleep / RTC APIs may change with `esp-hal` minor releases. Deep sleep does not return; the chip resets on wake
- **WS2812 heartbeat** — one random colour from the 6-colour palette at ≤64/255 via RMT NRZ (GRB, 800 kHz); GPIO boards blink once, active LOW, with no RMT. RMT is an unstable API
- **USB vs UART console** — UART bridge stays up; USB Serial/JTAG drops on sleep
- **GPIO 9 is not RTC** — XIAO boot button cannot wake from deep sleep; this example is timer-only
- **No sdkconfig** — board differences live in Cargo features + `src/board.rs`. The image still carries `esp_app_desc!()` from `esp-bootloader-esp-idf` so current `espflash` will flash it. That crate is image metadata, not ESP-IDF

## Power Budget

No radio. The ~25 mA active figure is CPU-only on the C6, not a legacy Xtensa 68 mA number. Sleep floor from the board specs: ~7–15 µA on DevKitC with the RTC domain, ~15 µA on XIAO.

| Mode | Typical current | Duration | Energy per cycle |
| --- | --- | --- | --- |
| Active (CPU, no RF) | ~25 mA | ~200 ms | ~1.4 µAh |
| Deep sleep (RTC timer + persistent RAM) | ~7–15 µA | 5 s | ~0.02 µAh |
| **Total per duty cycle** | — | ~5.2 s | ~1.4 µAh |

**Average current:** `(25 mA × 0.2 s + 0.015 mA × 5 s) / 5.2 s ≈ 0.98 mA` — target **< 1 mA**.

**Sleep wakeup source:** RTC timer. **Wakeup interval:** 5 seconds from sleep entry.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (DevKitC-C6, default features)**

   From `examples/deep-sleep-timer-node/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This checks the WS2812 / RMT and RTC sleep path.

2. **T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This checks the GPIO LED and RTC sleep path.

3. **T-A3 — Flash via espflash runner**

   ```sh
   cargo run --release
   ```

   Pass criterion: the image flashes, and the serial log shows first boot or a timer wake, LED-driver init, and sleep entry. Use the DevKitC **left** USB-C UART-C port so the monitor stays up through sleep. Native USB Serial/JTAG drops the port on the first sleep; that is not a firmware failure. A board is required for the flash half. T-A1 covers the build without a board. These commands are the check, not a recorded hardware pass.

### Manual — hardware required

**T-M1 — Boot counter persistence through real deep sleep**

Why automation is not practical: persistent RAM depends on the hardware RTC domain staying live through a true deep-sleep state. A host build cannot certify retention.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8, USB-C on the **left** UART-C port (UART bridge; typically `/dev/cu.usbserial-*` on macOS).

1. From `examples/deep-sleep-timer-node/`: `cargo run --release`
2. Watch at least five consecutive wakeups. Confirm the boot counter increments by 1 each cycle with no gaps, and that timer wakeup is logged after the first boot.
3. Confirm the monitor stays connected through the silent sleep interval (~5 s).
4. Power-cycle the board (disconnect USB, reconnect) and flash or monitor again if needed.
5. Confirm the log shows first boot and the counter has reset.

Pass criterion: the counter increments monotonically across sleep, resets only on power-cycle, and the monitor stays connected across sleep on the UART-C port.

**T-M2 — LED heartbeat (Seeed XIAO ESP32-C6)**

Why automation is not practical: on/off timing and active-LOW polarity need a person looking at the LED. The USB Serial/JTAG port also drops on sleep, so a monitor session cannot watch the whole cycle.

Hardware required: Seeed XIAO ESP32-C6, USB-C cable. `espflash monitor` exits when the chip sleeps. Watch the LED.

1. From `examples/deep-sleep-timer-node/`: `cargo run --release --no-default-features --features board-xiao-c6`
2. Watch the blue user LED on GPIO 15.
3. Confirm one brief blink per ~5 s wake, and that the LED is off during sleep.

Pass criterion: a single brief blink per cycle, off during sleep, correct active-LOW polarity.

**T-M3 — WS2812 random-colour heartbeat (ESP32-C6-DevKitC-1-N8)**

Why automation is not practical: colour variation, GRB order, and whether 64/255 is visible in room light need a person looking at the LED. A GPIO-level bug looks the same as a missing jumper.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8. Serial is optional; use the **left** USB-C UART-C port if you also want the T-M1 log.

1. From `examples/deep-sleep-timer-node/`: `cargo run --release`
2. Watch the onboard RGB LED for 5 or more consecutive wakeups.
3. Confirm the colour is not identical every cycle. One colour forever means the RNG is not applied to the 6-colour palette.
4. Confirm each blink is clearly visible in room light (not 8/255-dim) and that the LED is dark between blinks and during the ~5 s sleep.

Pass criterion: colour varies, blinks are visible, and the LED is off during sleep.

## Spec Files

- [FunctionalSpec.md](specs/FunctionalSpec.md) states observable behavior.
- [CodingSpec.md](specs/CodingSpec.md) states architecture and constraints, with no code.
- [TestSpec.md](specs/TestSpec.md) states how to check the example.

Edits go in those files. Then ask the agent **Activate esp32-sdd-full-project-generator**. Type that phrase to the agent, not in the shell. See [Using an AI coding agent](../../docs/guides/using-with-ai-agents.md).
