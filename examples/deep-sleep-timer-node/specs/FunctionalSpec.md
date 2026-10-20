# Deep Sleep Timer Node (ESP32-C6)

## Overview

Baseline deep-sleep example for rust-on-metal ESP32-C6 (`no_std` `esp-hal`). Demonstrates
the wake–persist–heartbeat–sleep loop that battery-powered C6 projects depend on. An RTC
timer fires about every 5 seconds; the firmware increments a boot counter in persistent
RAM, logs the count and wakeup cause over serial, blinks the onboard LED once as a visible
heartbeat, then returns to deep sleep. Chip is ESP32-C6 only.

On the DevKitC WS2812, each heartbeat uses a randomly selected colour from the project's
standard 6-colour palette (cap 64/255).

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/main.rs` are present (generated 2026-09-08). This line is not a hardware-test result. The catalog table in `specs/showcase-master-spec.md` is the source of truth across examples.

## Target Hardware

v1 chip: **ESP32-C6** only (`esp32c6`). Other SoCs are not in v1.

### Board Compatibility Checklist

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [x] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add boards for any SoC other than ESP32-C6.

RTC / LP GPIOs on C6 are **0–7 only**. GPIO 9 (XIAO boot button) is **not** RTC-capable and
**cannot** wake the chip from deep sleep. This example is **timer-only**; do not invent a
GPIO-9 wakeup workaround.

## Features

- Wake from deep sleep on an RTC timer of about **5 seconds**.
- On every wake: increment a boot counter stored with `#[esp_hal::ram(persistent)]` (or the
  equivalent `#[ram(unstable(...))]` form on the pinned crate). Guard the static with a
  magic/checksum so random bytes after a raced reset are treated as first boot, not a huge
  count. Persistent RAM survives deep sleep and is lost on power-cycle.
- Distinguish first boot (empty wakeup cause) from timer wakeup and from any other cause.
  Log the boot count and cause on every wake. Unrecognised causes log a warning and still
  sleep — never abort the loop.
- Blink the onboard status LED **once** per wake (about 100 ms on, 100 ms off) as a
  heartbeat, then sleep. LED off during sleep.
  - **DevKitC-C6 (default, `board-devkitc-c6`)**: WS2812 RGB LED on GPIO 8, driven by the
    `esp-hal` RMT peripheral (not a GPIO level). Colour is drawn from the 6-colour palette
    (red, green, blue, cyan, magenta, amber) at ≤64/255 using the hardware RNG. GRB byte
    order, 800 kHz NRZ. Start dark, return to dark before sleep.
  - **XIAO ESP32-C6 (`board-xiao-c6`)**: simple user LED on GPIO 15, **active LOW**. Binary
    on/off blink at the documented polarity. Do not run the RMT WS2812 encoder on this board.
- Pin numbers live in generated `src/board.rs`. Application code must not hardcode GPIOs.
- Board selection is a Cargo feature, not menuconfig. Default features select DevKitC-C6.
  XIAO: `cargo run --release --no-default-features --features board-xiao-c6`.
- Active window target: about **200 ms** (heartbeat plus log). No radio, no heap, no NVS.
- Serial log at INFO confirms first boot vs timer wake, the boot count, LED path, and sleep
  entry (`log` + `esp-println`).
- USB console: DevKitC **left** USB-C UART-bridge stays connected through sleep (silence
  during sleep, output resumes on wake). Native USB Serial/JTAG (DevKitC right port; XIAO)
  drops `/dev/cu.usbmodem*` on sleep entry and `espflash monitor` / `cargo run` exits. That
  is expected. Use the DevKitC UART-C port for counter-persistence tests; treat XIAO as
  LED-only observation unless a single-cycle capture is enough.

## Wakeup Strategy

**RTC timer only**, period ~5 s. No GPIO wakeup, no LP-core wakeup.

GPIO 9 is not an RTC GPIO on C6 and cannot wake from deep sleep. DevKitC boot (GPIO 0) is
RTC-capable but is not used as a wakeup source in this example.

Deep sleep does not return; the chip resets on wake. Sleep / RTC APIs require `esp-hal`
feature `unstable`.

## Connectivity

None. Non-radio crate pin: crates.io `esp-hal` 1.2.1 + `esp-rtos` 0.4.0.

## Security Requirements

None.

## Hardware Dependencies

- Board-spec: `board-specs/espressif/esp32-c6-devkitc-1-n8.md` (WS2812 RGB on GPIO 8;
  flash/monitor on the **left** USB-C UART-C port).
- Board-spec: `board-specs/seeed/xiao-esp32c6.md` (user LED GPIO 15, active LOW;
  USB Serial/JTAG console).
- No external components required.

The generator auto-invokes:

- `esp32-deep-sleep-engineer` — RTC timer wakeup, persistent RAM, sleep entry
- `esp32-ws2812-led-engineer` — DevKitC WS2812 RMT path

## Power Budget

C6, no radio. Sleep floor from board-specs (~7–15 µA DevKitC with RTC domain; ~15 µA XIAO).
Active current is CPU-only (no Wi-Fi / BLE); ~25 mA typical is used for the average, not the
legacy Xtensa 68 mA figure.

| Mode | Typical current | Duration | Energy per cycle |
|---|---|---|---|
| Active (CPU, no RF) | ~25 mA | ~200 ms | ~1.4 µAh |
| Deep sleep (RTC timer + persistent RAM) | ~7–15 µA | 5 s | ~0.02 µAh |
| **Total per duty cycle** | — | ~5.2 s | ~1.4 µAh |

**Average current:** `(25 mA × 0.2 s + 0.015 mA × 5 s) / 5.2 s ≈ 0.98 mA` — target **< 1 mA**.

**Sleep wakeup source:** RTC timer  
**Wakeup interval:** 5 seconds

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf` (default DevKitC-C6 features).
- `cargo clippy --release -- -D warnings` is clean for the default features.
- `cargo build --release --no-default-features --features board-xiao-c6` and the matching clippy invocation are clean.
- **`cargo run --release`** flashes via `espflash` on DevKitC-C6 (**left** USB-C / UART-C). Serial shows first boot, then `Wake #N` with a monotonic counter; the WS2812 blinks once per wake in a visibly varying colour and is dark during sleep. Monitor stays connected across sleep.
- Optional XIAO: `cargo run --release --no-default-features --features board-xiao-c6`. GPIO 15 blinks once per wake (active-LOW); LED off during sleep. USB monitor dropping on first sleep is expected.
- Boot counter increments by 1 on every timer wakeup with no skips; resets only on power-cycle.
- No stack overflow or watchdog across 20+ consecutive cycles.
