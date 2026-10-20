<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-full-project-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# BLE Beacon + Deep Sleep — ESP32-C6

> Duty-cycled BLE beacon: advertise a 10 s window, tear the radio down, deep sleep.

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/main.rs` are in this folder (generated 2026-09-08 from the specs). The build commands below apply to this crate. The 10 s / 10 s cycle is a demo, not a multi-year battery budget. This is not a record of a hardware test on 2026-10-20.

This example enables `esp-hal` feature **`unstable`** (radio + sleep / RTC + RMT). **`esp-radio` is an unstable beta** (git-pinned with the `esp-hal` monorepo stack at tag `esp-hal-v1.2.1`; TrouBLE host `trouble-host` 0.7 as specified in `shared-specs/ble-patterns.md`). Those APIs may change with `esp-hal` minor releases.

## Overview

Wakes from deep sleep, advertises a non-connectable BLE beacon with complete local name `"ESP32-SDD"` and custom manufacturer data (boot counter, sequence, magic `DE AD BE EF`), then stops advertising, drops `esp-radio`, and returns to deep sleep for ~10 s. The onboard LED is on **only while advertising is active** (WS2812 solid blue at 64/255 on DevKitC; GPIO active-LOW on XIAO) and stays dark during init, teardown, and sleep.

On DevKitC-C6, GPIO 0 (boot button, RTC-capable) can trigger an on-demand advertisement. On XIAO ESP32-C6 the boot button is GPIO 9, which is **not** an RTC GPIO — this firmware arms GPIO 0 (D0) for an external active-LOW button instead. Timer wakeup (~10 s) is always armed.

Pin numbers live in generated `src/board.rs`. Never hard-code GPIOs in `main.rs`.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- A BLE scanner (e.g. nRF Connect) for manual tests

ESP-IDF is **not** required. Do not install `espup` or the `esp` rustup channel. Radio crates are git-pinned — never mix crates.io `esp-rtos = "0.4.0"` with git `esp-hal`.

## Supported Boards

Auto-discovered from `board-specs/`, filtered to ESP32-C6. BLE + deep-sleep timer + user LED required.

> Default board: Espressif ESP32-C6-DevKitC-1-N8

| Board | board-spec file | Chip feature | Board Cargo feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | `espressif/esp32-c6-devkitc-1-n8.md` | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | `seeed/xiao-esp32c6.md` | `esp32c6` | `board-xiao-c6` | GPIO 15 | Active LOW | Simple GPIO | USB Serial/JTAG |

¹ A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

GPIO 9 (XIAO boot button) is **not** an RTC GPIO and cannot wake the chip from deep sleep.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | Solid blue while advertising, off otherwise (WS2812) | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW | ~15 µA |

UART-bridge boards (DevKitC **left** USB-C) keep `espflash monitor` connected through deep sleep; native USB boards drop `/dev/cu.usbmodem*` and the monitor exits. That is expected, not a crash. Use the DevKitC UART-C port for counter-persistence tests; treat XIAO as LED-only observation unless you accept a single-cycle capture.

## Build & Flash

```sh
cargo build --release
cargo clippy --release -- -D warnings
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

**File → Open Folder… → select `examples/ble-beacon-deep-sleep/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

### First boot (ESP32-C6-DevKitC-1-N8, UART-C port)

```
ble_beacon_deep_sleep: First boot — counter starts at 1 (resets on power-cycle)
ble_beacon_deep_sleep: BLE beacon — WS2812 RGB LED, GPIO 8, wake GPIO 0, sleep 10 s
ble_beacon_deep_sleep: WS2812 RMT init on GPIO 8 (GRB order, max brightness 64/255)
ble_beacon_deep_sleep: BLE MAC: C0:A1:B2:C3:D4:E5
ble_beacon_deep_sleep: ADV PDU 26 bytes (≤ 31)
ble_beacon_deep_sleep: adv start  name=ESP32-SDD  boot=1 seq=1 magic=DEADBEEF
ble_beacon_deep_sleep: adv stop
ble_beacon_deep_sleep: Radio teardown complete
ble_beacon_deep_sleep: Sleeping for 10 s
```

### Subsequent wakeups (UART-C port)

```
ble_beacon_deep_sleep: Wake #2 — timer
ble_beacon_deep_sleep: BLE beacon — WS2812 RGB LED, GPIO 8, wake GPIO 0, sleep 10 s
ble_beacon_deep_sleep: BLE MAC: C0:A1:B2:C3:D4:E5
ble_beacon_deep_sleep: adv start  name=ESP32-SDD  boot=2 seq=2 magic=DEADBEEF
ble_beacon_deep_sleep: adv stop
ble_beacon_deep_sleep: Radio teardown complete
ble_beacon_deep_sleep: Sleeping for 10 s
```

Scanner apps should list **`ESP32-SDD`**. The BLE MAC is a static-random address persisted across deep sleep (new only on power-cycle).

## Advertising payload

Legacy 31-byte PDU encoded with TrouBLE `AdStructure::encode_slice`. See `shared-specs/ble-patterns.md`.

| Offset | Length | Value | Description |
| --- | --- | --- | --- |
| 0 | 2 | `02 01` | Flags AD length + type |
| 2 | 1 | `06` | LE General Discoverable \| BR/EDR Not Supported |
| 3 | 1 | `0B` | Manufacturer data length (`N+3`, N=8) |
| 4 | 1 | `FF` | Manufacturer data AD type |
| 5 | 2 | `FF FF` | Company ID `0xFFFF` (little-endian, test/internal) |
| 7 | 2 | `boot_count` LE | Wake counter |
| 9 | 2 | `sequence` LE | Sequence counter |
| 11 | 4 | `DE AD BE EF` | Magic |
| 15 | 1 | `0A` | Local name length (type + 9 chars) |
| 16 | 1 | `09` | Complete Local Name AD type |
| 17 | 9 | `ESP32-SDD` | Device name |

Total **26 bytes** (≤ 31). Overflow is fail-closed: log and skip advertising.

## Key Concepts

- **Standalone Cargo crate** — `#![no_std]` `#![no_main]`; Embassy entry via `esp-rtos`; no workspace at the repo root
- **`esp-radio` + TrouBLE** — non-connectable advertising; host resources sized for zero connections; unstable beta
- **Git-pin** — `esp-hal` / `esp-radio` / `esp-rtos` / `esp-alloc` from tag `esp-hal-v1.2.1`; `trouble-host` 0.7
- **Bounded advertising window** via `embassy_time` `select` against the TrouBLE runner
- **Radio teardown before sleep** — drop advertiser, stop runner, drop `BleConnector`, then `sleep_deep`
- **Persistent counters + BLE address** — `#[esp_hal::ram(unstable(rtc_fast, persistent))]`; magic + checksum
- **LED = advertising-active only** — dark during init and teardown
- **GPIO 9 is not RTC** — XIAO boot button cannot wake from deep sleep; GPIO 0 is the RTC wake pin
- **No sdkconfig** — board differences live in Cargo features + `src/board.rs`

## Limitations

- 10 s / 10 s is a demonstration duty cycle. Average current is milliamps, not a multi-year coin-cell budget.
- XIAO onboard BOOT (GPIO 9) cannot wake from deep sleep.
- Company ID `0xFFFF` is not a product identifier. Do not ship this PDU as a commercial beacon.
- Legacy 31-byte PDUs only; BLE 5 extended advertising is out of scope.

## Power Budget

| Mode | Typical current | Duration |
| --- | --- | --- |
| Active (CPU + BLE advertising) | ~30 mA | ~10 s (includes ~5–10 ms teardown) |
| Deep sleep (RTC timer + GPIO 0 + persistent RAM) | ~7–15 µA | 10 s |
| **Target average** | **< 15 mA** | ~20 s period |

`(30 mA × 10 s + 0.015 mA × 10 s) / 20 s ≈ 15.0 mA`. Production beacons should use a much shorter burst.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (DevKitC-C6, default features)**

   From `examples/ble-beacon-deep-sleep/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings (radio git-pin resolves).

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

   Pass: image flashes; serial log shows first boot or a wake, `BLE MAC:`, advertising activity or a logged failure, and sleep entry. Use the DevKitC **left** USB-C UART-C port.

### Manual Tests — hardware required

**T-M1 — BLE advertisement visible to scanner**

Why manual: OTA BLE content needs a second device acting as a scanner.

1. `cargo run --release` on DevKitC-C6.
2. Open nRF Connect → Scanner → start scanning.
3. Confirm a burst each wake; name `ESP32-SDD`; company ID `0xFFFF`; magic `DE AD BE EF`; `boot_count` increments.

Pass: advertisement every cycle; manufacturer data matches spec.

---

**T-M2 — Active window / LED timing**

Why manual: advertising-window duration is easiest to judge from the LED.

1. Observe LED on only while advertising (~10 s); off during init, teardown, and sleep.
2. Serial timestamps: adv-start then adv-stop within the 10 s window; whole wake ≤ 12 s.

Pass: LED dark except during advertising.

---

**T-M3 — Button wakeup (DevKitC GPIO 0 / XIAO external GPIO 0)**

Why manual: physical RTC GPIO wakeup requires silicon.

1. During sleep, press DevKitC BOOT (GPIO 0), or an external active-LOW button on XIAO D0.
2. Confirm advertisement and LED within 500 ms.

Pass: on-demand burst. XIAO GPIO 9 cannot wake from deep sleep.

---

**T-M4 — Radio teardown / sleep current (optional)**

Why manual: milliamps vs microamps needs a current meter.

1. Confirm the device keeps cycling (LED not stuck on).
2. If metered, sleep floor is tens of microamps, not milliamps.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)

## Board Specs

- [board-specs/espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md)
- [board-specs/seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md)
