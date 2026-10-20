<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-full-project-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Deep Sleep Timer Node — ESP32-C6

> RTC timer wakeup with boot-counter persistence and a per-cycle LED heartbeat.

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/main.rs` are in this folder (generated 2026-09-08 from the specs). The build commands below apply to this crate. This is not a record of a hardware test on 2026-10-20.

This example uses `esp-hal` sleep / RTC and RMT APIs gated on feature **`unstable`**. Those APIs may change with `esp-hal` minor releases.

## Overview

Baseline deep-sleep example. Demonstrates the wake–persist–heartbeat–sleep loop that every battery-powered ESP32-C6 project depends on. An RTC timer fires about every 5 seconds; the firmware increments a boot counter stored in persistent RAM (`#[esp_hal::ram(unstable(rtc_fast, persistent))]` on this HAL pin), logs the count and wakeup cause over serial, blinks the onboard LED once, then returns to deep sleep.

On the DevKitC WS2812, each heartbeat blink uses a randomly selected colour from the project's standard 6-colour palette (cap 64/255).

Pin numbers live in generated `src/board.rs`. Never hard-code GPIOs in `main.rs`.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6

ESP-IDF is **not** required. Do not install `espup` or the `esp` rustup channel.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

> **Excluded from v1:** Adafruit HUZZAH32 (ESP32), Seeed XIAO ESP32S3 (ESP32-S3), Seeed XIAO ESP32-C5 (ESP32-C5), YEJMKJ ESP32-S3-DevKitC-1-N16R8 (ESP32-S3) — not in v1 — C6 only.

GPIO 9 (XIAO boot button) is **not** an RTC GPIO and cannot wake the chip from deep sleep. This example is timer-only.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | Color: green = success, red = failure, off = sleep (WS2812) | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW | ~15 µA |

UART-bridge boards (DevKitC **left** USB-C) keep `espflash monitor` connected through deep sleep; native USB boards drop `/dev/cu.usbmodem*` and the monitor exits. That is expected, not a crash. Use the DevKitC UART-C port for counter-persistence tests; treat XIAO as LED-only observation unless you accept a single-cycle capture.

## Build & Flash

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

> **Never edit generated source files directly.** To change board configuration, update `specs/FunctionalSpec.md` or `specs/CodingSpec.md` and regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/deep-sleep-timer-node/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

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

Persistent RAM is retained across deep sleep but cleared on power-on. There is no sdkconfig RTC-memory option. Heartbeat colour is chosen by the hardware RNG each wake, so the palette index will vary.

### Seeed XIAO ESP32-C6 (active LOW, GPIO 15)

```
deep_sleep_timer_node: First boot — counter starts at 1 (resets on power-cycle)
deep_sleep_timer_node: Timer node — GPIO LED (active-LOW), GPIO 15, sleep 5 s
deep_sleep_timer_node: GPIO LED heartbeat on GPIO 15 (GPIO LED (active-LOW))
deep_sleep_timer_node: Sleeping for 5 s
```

The USB Serial/JTAG monitor typically exits when the chip enters deep sleep.

## Key Concepts

- **Standalone Cargo crate** — `#![no_std]` `#![no_main]`; Embassy entry via `esp-rtos`; no workspace at the repo root
- **`#[esp_hal::ram(unstable(rtc_fast, persistent))]`** — boot counter survives deep sleep; magic + checksum reject random bytes; types must be persistable
- **RTC timer wakeup** — `LowPower::set_wakeup_deadline` + `sleep_deep` (feature `unstable`); ~5 s from sleep entry
- **WS2812 heartbeat** — random colour from the 6-colour palette at ≤64/255 via RMT NRZ; GPIO boards blink once, active-LOW
- **USB vs UART console** — UART bridge stays up; USB Serial/JTAG drops on sleep
- **GPIO 9 is not RTC** — XIAO boot button cannot wake from deep sleep; this example is timer-only
- **No sdkconfig** — board differences live in Cargo features + `src/board.rs`

## Power Budget

| Mode | Typical current | Duration |
| --- | --- | --- |
| Active (CPU, no RF) | ~25 mA | ~200 ms |
| Deep sleep (RTC timer + persistent RAM) | ~7–15 µA | 5 s |
| **Target average** | **< 1 mA** | ~5.2 s period |

`(25 mA × 0.2 s + 0.015 mA × 5 s) / 5.2 s ≈ 0.98 mA`.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (DevKitC-C6, default features)**

   From `examples/deep-sleep-timer-node/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings.

2. **T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings.

3. **T-A3 — Flash via espflash runner**

   ```sh
   cargo run --release
   ```

   Pass: image flashes; serial log shows first boot or a timer wake, LED-driver init, and sleep entry. Use the DevKitC **left** USB-C UART-C port.

### Manual Tests — hardware required

**T-M1 — Boot counter persistence through real deep sleep**

Why manual: persistent RAM depends on the hardware RTC domain staying live through a true deep-sleep state.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8, USB-C on the **left** UART-C port.

1. `cargo run --release`
2. Observe several consecutive wakeups — confirm the counter increments with no gaps.
3. Power-cycle (disconnect USB, reconnect).
4. Confirm the counter resets to first boot.

Pass: counter increments monotonically; resets only on power-cycle; monitor stays connected across sleep.

---

**T-M2 — LED heartbeat (GPIO board)**

Why manual: visual on/off timing and active-LOW polarity require real hardware.

1. `cargo run --release --no-default-features --features board-xiao-c6`
2. Observe a single brief blink per wake; LED off during sleep. USB monitor drop on sleep is expected.

Pass: single brief blink per cycle, off during sleep.

---

**T-M3 — WS2812 random-colour heartbeat**

Why manual: colour variation and 64/255 brightness require observing the physical LED.

1. `cargo run --release` on DevKitC-C6.
2. Observe 5+ consecutive wakeups.
3. Confirm colour is not identical every cycle; each blink is clearly visible; LED is off between blinks.

Pass: colour varies; blinks visible; LED off during sleep.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
