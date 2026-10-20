# BLE Beacon + Deep Sleep (ESP32-C6)

## Overview

Duty-cycled rust-on-metal BLE beacon for ESP32-C6 (`no_std` `esp-hal`). Wakes from
deep sleep, advertises a non-connectable BLE packet for about **10 seconds** with
complete local name `"ESP32-SDD"` and custom manufacturer data (boot counter,
sequence, magic `DE AD BE EF`), tears `esp-radio` down, then returns to deep sleep
for about **10 seconds**. The 50 % duty cycle is for easy demonstration, not a
production multi-year window. Chip is ESP32-C6 only.

The onboard LED is **on only while advertising is actually running** (solid blue
on DevKitC WS2812; GPIO active level on XIAO). It stays dark during radio init,
teardown, and sleep.

On DevKitC-C6, GPIO 0 (boot button, RTC-capable) can trigger an on-demand
advertisement. On XIAO ESP32-C6 the onboard boot button is GPIO 9, which is
**not** an RTC GPIO and cannot wake from deep sleep; use the RTC timer or an
external button on LP GPIO 0–7 (this example arms GPIO 0 / D0 as that external
wake pin).

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/main.rs` are present (generated 2026-09-08). The 10 s / 10 s cycle is a demo, not a multi-year budget. This line is not a hardware-test result. The catalog table in `specs/showcase-master-spec.md` is the source of truth across examples.

## Target Hardware

v1 chip: **ESP32-C6** only (`esp32c6`). Other SoCs are not in v1.

### Board Compatibility Checklist

- [x] **Espressif ESP32-C6-DevKitC-1-N8** *(default)* — `board-specs/espressif/esp32-c6-devkitc-1-n8.md` — Cargo feature `board-devkitc-c6`
- [x] **Seeed XIAO ESP32-C6** — `board-specs/seeed/xiao-esp32c6.md` — Cargo feature `board-xiao-c6`

Do not add boards for any SoC other than ESP32-C6.

RTC / LP GPIOs on C6 are **0–7 only**. GPIO 9 (XIAO boot button) is **not** RTC-capable
and **cannot** wake the chip from deep sleep. Do not invent a GPIO-9 wakeup workaround.

## Features

- Wake from deep sleep on an RTC timer of about **10 seconds**, measured from sleep entry.
- Optional RTC GPIO wakeup on **GPIO 0** (active LOW, internal pull-up): DevKitC boot
  button; XIAO external button on D0. Same advertise pipeline as a timer wake. GPIO 9
  is not used as a wakeup source.
- On every wake: increment a boot counter and a sequence counter stored with
  `#[esp_hal::ram(persistent)]` (or the equivalent `#[ram(unstable(...))]` form). Guard
  with magic + checksum so random bytes after a raced reset are first boot, not a huge
  count. Lost on power-cycle. Persist a static-random BLE address the same way so the
  scanner sees a stable identity across sleep (new address only on first boot / power-cycle).
- Advertise one **non-connectable, scannable, undirected** legacy BLE burst for about
  **10 seconds** via `esp-radio` + TrouBLE (`trouble-host`):
  - Complete Local Name AD element: `"ESP32-SDD"` (visible by name in nRF Connect without
    a MAC filter).
  - Manufacturer data: company ID `0xFFFF` (Bluetooth SIG test/internal), payload
    `[boot_count (2 bytes LE)][sequence (2 bytes LE)][0xDE 0xAD 0xBE 0xEF]`.
  - Total ADV PDU **must remain ≤ 31 bytes**. Fail closed (log and skip advertising) if
    encoding would overflow. Budget: Flags (3) + manufacturer (12) + local name (11) = 26.
  - Advertising interval about **100 ms**.
- Host resources sized for **zero connections**. No GATT server, no scanning, no
  security manager / bonding path.
- After the host is up and before advertising starts, log the advertised BLE address as
  `BLE MAC: XX:XX:XX:XX:XX:XX`. Also log wakeup cause, counters, and payload on every cycle.
- Stop advertising after the 10 s window, then **fully tear down** TrouBLE + `esp-radio`
  before deep sleep so sleep current stays in microamps. If advertising fails to start,
  log a recoverable error (LED stays dark) and still sleep — never block indefinitely.
- LED on **only while advertising is active**; off immediately when advertising stops;
  dark during init, teardown, and sleep.
  - **DevKitC-C6**: WS2812 on GPIO 8 via RMT, GRB, solid blue at **64/255** (cap, not a
    floor). A GPIO level produces no light.
  - **XIAO ESP32-C6**: GPIO 15 active-LOW binary on/off. No RMT on this board.
- Pin numbers live only in generated `src/board.rs`. Board selection is a Cargo feature
  (`board-devkitc-c6` default; XIAO: `--no-default-features --features board-xiao-c6`).
- USB console: DevKitC **left** USB-C UART-bridge stays connected through sleep. Native
  USB Serial/JTAG (DevKitC right port; XIAO) drops `/dev/cu.usbmodem*` on sleep entry.
- Active window target: advertising **10 s** plus BLE init/teardown (teardown ~5–10 ms).
  Whole wake must not exceed **12 s** before sleep.

## Wakeup Strategy

**RTC timer** (~10 s from sleep entry) **plus RTC GPIO 0** (active LOW) on both boards.
GPIO 9 is not used as a wakeup source.

Deep sleep does not return; the chip resets on wake. Sleep / RTC / RMT / radio APIs
require `esp-hal` feature `unstable`.

## Connectivity

BLE advertising only (broadcaster). Non-connectable. No Wi-Fi, no GATT, no pairing.

This is a **radio** example: git-pin `esp-hal`, `esp-radio`, `esp-rtos`, and `esp-alloc`
from tag `esp-hal-v1.2.1`. Never mix crates.io `esp-rtos = "0.4.0"` with git `esp-hal`.
`esp-radio` is an unstable beta. Host is TrouBLE `trouble-host` **0.7** (same pin as
upstream `esp-hal` 1.2.1 BLE examples). Follow `shared-specs/ble-patterns.md`.

## Security Requirements

Not production-targeted. Broadcaster-only: no bonding, no GATT, no secrets in the PDU.
Company ID `0xFFFF` is for test/example use, not a registered SIG company identifier.

## Hardware Dependencies

- Board-spec: `board-specs/espressif/esp32-c6-devkitc-1-n8.md` (WS2812 GPIO 8; boot GPIO 0;
  flash/monitor on the **left** USB-C UART-C port).
- Board-spec: `board-specs/seeed/xiao-esp32c6.md` (user LED GPIO 15 active LOW; boot GPIO 9
  is not RTC; USB Serial/JTAG).
- A BLE scanner (e.g. nRF Connect) for manual advertisement checks. No other external
  components.

The generator auto-invokes:

- `esp32-ble-beacon-engineer` — TrouBLE advertising, manufacturer data, teardown
- `esp32-deep-sleep-engineer` — RTC timer + GPIO 0 wakeup, persistent RAM, sleep
- `esp32-ws2812-led-engineer` — DevKitC WS2812 RMT path

## Power Budget

C6 with BLE advertising. Sleep floor from board-specs (~7–15 µA DevKitC with RTC domain;
~15 µA XIAO). Typical advertising current ~30 mA average (peaks higher during TX). Radio
teardown (~5–10 ms) is part of the active window. The 10 s advertise + 10 s sleep cycle
is a **demo** 50 % duty cycle so a scanner and the LED are easy to observe; a production
beacon would use a much shorter burst.

| Mode | Typical current | Duration | Energy per cycle |
|---|---|---|---|
| Active (CPU + BLE advertising) | ~30 mA | ~10 s | ~83 µAh |
| Radio teardown | included in active | ~5–10 ms | — |
| Deep sleep (RTC timer + GPIO 0 + persistent RAM) | ~7–15 µA | 10 s | ~0.04 µAh |
| **Total per duty cycle** | — | ~20 s | ~83 µAh |

**Average current:** `(30 mA × 10 s + 0.015 mA × 10 s) / 20 s ≈ 15.0 mA` — target **< 15 mA**.

**Sleep wakeup source:** RTC timer + RTC GPIO 0  
**Wakeup interval:** 10 seconds

## Success Criteria

- `cargo build --release` completes with zero `rustc` warnings on `riscv32imac-unknown-none-elf` (default DevKitC-C6 features).
- `cargo clippy --release -- -D warnings` is clean for the default features.
- `cargo build --release --no-default-features --features board-xiao-c6` and the matching clippy invocation are clean.
- **`cargo run --release`** flashes via `espflash` on DevKitC-C6 (**left** USB-C / UART-C). Serial shows first boot or a timer/GPIO wake, `BLE MAC: XX:XX:XX:XX:XX:XX`, adv start/stop, radio teardown, and sleep. Monitor stays connected across sleep on UART-C.
- Optional XIAO: `cargo run --release --no-default-features --features board-xiao-c6`. USB monitor dropping on first sleep is expected.
- BLE advertisement is visible to a scanner on every wake. Device appears as **`"ESP32-SDD"`**. Manufacturer data matches company ID `0xFFFF`, little-endian counters, and magic `DE AD BE EF`. `boot_count` increments by 1 each wake.
- Active window does not exceed 12 s. LED is dark during init and teardown; on only while advertising (DevKitC solid blue at 64/255; XIAO GPIO on).
- GPIO 0 (DevKitC BOOT, or XIAO external on D0) wakes the device and starts an advertisement within 500 ms of the press being observed. GPIO 9 is not used for wakeup.
- No stack overflow, BLE host panic, or watchdog across 20+ consecutive cycles.

## Limitations

- 10 s / 10 s is a demonstration duty cycle. Average current is milliamps, not a multi-year coin-cell budget.
- XIAO onboard BOOT (GPIO 9) cannot wake from deep sleep.
- Company ID `0xFFFF` is not a product identifier. Do not ship this PDU as a commercial beacon.
- Legacy 31-byte PDUs only; BLE 5 extended advertising is out of scope.
