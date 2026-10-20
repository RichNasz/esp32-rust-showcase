<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# BLE Beacon + Deep Sleep — ESP32-C6

> Duty-cycled BLE beacon (demo window, not a multi-year budget).

## Implementation status

**Firmware generated.** Crate present since the 2026-09-08 generation. 10 s / 10 s demo duty cycle, not a multi-year budget. This label is not a hardware pass.

A spec is Markdown a person writes; the agent generates the crate from it. See [Using With AI Agents](../../docs/guides/using-with-ai-agents.md).

`esp-hal` feature **`unstable`** is required for the radio, sleep / RTC, and RMT. Those APIs may change with an `esp-hal` minor release. **`esp-radio` is an unstable beta** (crates.io `=1.0.0-beta.1` with feature `ble`, plus `esp-hal` `=1.2.1`, `esp-rtos` `=0.4.0`, `esp-alloc` `=0.11.0`, and `esp-bootloader-esp-idf` `=0.6.0`). Do not git-pin those HP crates. The host is crates.io `trouble-host` **0.7**.

## Overview

The chip wakes from deep sleep, advertises one **non-connectable, scannable, undirected** legacy BLE burst for about **10 seconds**, tears the radio down, and deep-sleeps for about **10 seconds**. The 50 % duty cycle is so a scanner and the LED are easy to see. It is not a production multi-year window. Average current is about **15 mA**, not microamps.

The complete local name is **`ESP32-SDD`**. Manufacturer data is company ID `0xFFFF` (Bluetooth SIG test/internal), then little-endian boot count, little-endian sequence, then magic `DE AD BE EF`. Advertising interval is about **100 ms**. The legacy PDU is **26 bytes** and must stay **≤ 31**. If encoding would overflow, the firmware logs and skips advertising (fail closed). There is no GATT server, no scanning, and no bonding. Host resources are sized for **zero connections**.

The onboard LED is **on only while advertising is running**. It stays dark during radio init, teardown, and sleep. DevKitC: WS2812 on GPIO 8, solid blue at **64/255** (a cap, not a floor). XIAO: GPIO 15 active LOW, on/off. A GPIO level on the DevKitC pin does not light the LED.

Wake sources, both always armed: an RTC timer (~10 s from sleep entry) and **RTC GPIO 0**, active LOW, internal pull-up. On DevKitC that pin is the BOOT button. On XIAO the onboard BOOT button is **GPIO 9**, which is **not** an RTC GPIO and cannot wake the chip. This firmware arms GPIO 0 (D0) for an external active-LOW button instead. Do not invent a GPIO 9 wakeup. Deep sleep does not return; the chip resets on wake. A GPIO 0 press should start an advertisement within **500 ms** of the press being observed.

Boot count, sequence, and a static-random BLE address live in persistent RAM (`#[esp_hal::ram(unstable(rtc_fast, persistent))]` on this pin), guarded by magic and a checksum. They survive deep sleep and are lost on power-cycle. A bad image after a raced reset counts as first boot, not a huge counter. The log line `BLE MAC: XX:XX:XX:XX:XX:XX` is that address. Scanners see the same identity across sleep, and a new address only after power-cycle.

If advertising fails to start, the firmware logs a recoverable error, leaves the LED dark, and still sleeps. It does not block forever. Teardown is about **5–10 ms** and is part of the active window. The whole wake must not exceed **12 s** before sleep.

Pin numbers live in generated `src/board.rs`. Do not hard-code GPIOs in `main.rs`.

## Prerequisites

- **Board:** Espressif **ESP32-C6-DevKitC-1-N8** (default) or Seeed **XIAO ESP32-C6**.
- **USB:** DevKitC **left** USB-C (UART-C bridge). That port stays connected through deep sleep. The DevKitC **right** port and the XIAO are USB Serial/JTAG and drop on sleep. XIAO needs an external active-LOW button on D0 (GPIO 0) only if you want button wakeup. The onboard BOOT key cannot do that.
- **Scanner:** a phone app such as nRF Connect, for the manual air check. No other parts.
- **Toolchain:** rustup **stable** (MSRV 1.95.0), `rust-src`, target `riscv32imac-unknown-none-elf`, and `espflash` / `cargo-espflash`. Install steps: [01 — Toolchain Setup](../../docs/guides/01-toolchain-setup.md).

ESP-IDF is not a prerequisite. Do not install `espup` or the `esp` rustup channel.

## Supported Boards

Chip ≠ board. v1 chip feature `esp32c6` is the SoC. Boards select pins and USB. Not `idf.py set-target`.

Auto-discovered from `board-specs/` for ESP32-C6 with BLE, a deep-sleep RTC timer, and a user LED. Both catalog boards pass. Default board: Espressif ESP32-C6-DevKitC-1-N8.

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

GPIO 9 (XIAO boot button) is **not** an RTC GPIO and cannot wake the chip from deep sleep. RTC / LP GPIOs on C6 are **0–7** only.

## Per-Board Behavior

| Board | Serial monitor during deep sleep | LED feedback | Deep sleep floor |
| --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | Stays connected throughout sleep | WS2812: solid blue while advertising, off otherwise | ~7–15 µA |
| Seeed XIAO ESP32-C6 | Drops and reconnects on wake (USB Serial/JTAG) | On / off, active LOW | ~15 µA |

The difference that surprises people the first time is the serial port, not the beacon. UART-bridge boards (DevKitC **left** USB-C) keep `espflash monitor` connected through deep sleep. Native USB boards drop `/dev/cu.usbmodem*` and the monitor exits. That is expected, not a crash. The DevKitC **right** USB-C port is also USB Serial/JTAG and drops the same way. Use the DevKitC UART-C port when you need the counter across sleeps. Treat XIAO as LED-only observation unless you accept a single-cycle capture. Both boards use the same 10 s / 10 s demo window.

## Build & Flash

From this directory:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

The zero-warning check in Testing is the release-profile clippy invocation (`cargo clippy --release -- -D warnings`).

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

Radio crates are the crates.io pin: `esp-hal` `=1.2.1`, `esp-radio` (features `esp32c6`, `unstable`, `ble`) `=1.0.0-beta.1`, `esp-rtos` (features `esp32c6`, `embassy`, `esp-radio`) `=0.4.0`, `esp-alloc` `=0.11.0`, and `esp-bootloader-esp-idf` `=0.6.0`. Do not git-pin those HP crates. `trouble-host` 0.7 comes from crates.io. BLE buffers use about **72 KiB** of internal DRAM. Part of that budget is the reclaimed DRAM region, which the linker rejects without `esp_app_desc!()`.

Use the DevKitC **left** USB-C UART-C port so the monitor stays up through sleep.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/ble-beacon-deep-sleep/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected behavior

These are the lines the firmware logs. They are not a transcript from a board. The MAC bytes below show the log shape. The real address is a static-random value, new only on power-cycle, and stable across deep sleep. A scanner should list **`ESP32-SDD`**.

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

The WS2812 is solid blue only between adv start and adv stop, then dark. On the UART-C port the monitor stays connected across `Sleeping for 10 s`.

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

A GPIO 0 wake names a GPIO cause instead of `timer`. XIAO logs the GPIO 15 active-LOW LED path. Its USB monitor drops on the first sleep. That drop is not a firmware failure.

## Key Concepts

- **Standalone Cargo crate** — `#![no_std]` `#![no_main]`; Embassy entry via `esp-rtos` on TIMG0 + `FROM_CPU_INTR0`; no workspace at the repo root
- **`esp-radio` + TrouBLE** — non-connectable scannable undirected advertising at about 100 ms; host sized for zero connections; `esp-radio` is an unstable beta
- **Radio pin** — crates.io `esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1`, `esp-rtos` `=0.4.0`, and `esp-alloc` `=0.11.0`; `trouble-host` 0.7; do not git-pin the HP radio crates
- **`esp-hal` `unstable`** — radio, sleep, RTC, and RMT
- **Bounded advertising window** — `embassy_time` `select` against the TrouBLE runner; whole wake ≤ 12 s
- **Radio teardown before sleep** — drop the advertiser, stop the runner, drop `BleConnector`, then `sleep_deep`. Skipping teardown leaves the radio in milliamps
- **Persistent counters + BLE address** — `#[esp_hal::ram(unstable(rtc_fast, persistent))]`; magic + checksum; lost on power-cycle
- **LED = advertising-active only** — dark during init and teardown; DevKitC blue at 64/255; XIAO active LOW
- **GPIO 9 is not RTC** — XIAO BOOT cannot wake from deep sleep; GPIO 0 is the RTC wake pin
- **Heap** — about 72 KiB internal DRAM for BLE buffers, including the reclaimed DRAM region; `esp_app_desc!()` is required for current `espflash` and for that reclaimed region
- **No sdkconfig** — board differences live in Cargo features and `src/board.rs`

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

Total **26 bytes** (Flags 3 + manufacturer 12 + local name 11). Overflow is fail-closed: log and skip advertising. Company ID `0xFFFF` is not a registered product identifier.

## Power Budget

C6 with BLE advertising. Sleep floors come from the board-specs (~7–15 µA DevKitC with the RTC domain on; ~15 µA XIAO). Typical advertising current is ~30 mA average (peaks higher during TX). Radio teardown (~5–10 ms) is inside the active window. The 10 s advertise + 10 s sleep cycle is a **demo** so a scanner and the LED are easy to observe. A production beacon would use a much shorter burst. This is not a multi-year coin-cell budget.

| Mode | Typical current | Duration | Energy per cycle |
| --- | --- | --- | --- |
| Active (CPU + BLE advertising) | ~30 mA | ~10 s | ~83 µAh |
| Radio teardown | included in active | ~5–10 ms | — |
| Deep sleep (RTC timer + GPIO 0 + persistent RAM) | ~7–15 µA | 10 s | ~0.04 µAh |
| **Total per duty cycle** | — | ~20 s | ~83 µAh |

**Average current:** `(30 mA × 10 s + 0.015 mA × 10 s) / 20 s ≈ 15.0 mA` — target **< 15 mA**.

**Sleep wakeup source:** RTC timer + RTC GPIO 0. **Wakeup interval:** 10 seconds.

## Limitations

- 10 s / 10 s is a demonstration duty cycle. Average current is milliamps, not a multi-year coin-cell budget.
- XIAO onboard BOOT (GPIO 9) cannot wake from deep sleep.
- Company ID `0xFFFF` is not a product identifier. Do not ship this PDU as a commercial beacon.
- Legacy 31-byte PDUs only; BLE 5 extended advertising is out of scope.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)**

   From `examples/ble-beacon-deep-sleep/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This also checks that the crates.io radio pin resolves (RMT + RTC sleep + `esp-radio` + TrouBLE). It does not need a board.

2. **T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This checks the GPIO LED + BLE + RTC sleep path.

3. **T-A3 — Flash via the espflash runner (DevKitC-C6)**

   ```sh
   cargo run --release
   ```

   The runner is `espflash flash --monitor`. Pass criterion: the image flashes, and the monitor shows a first boot or a wake, `BLE MAC:`, advertising activity or a logged failure, and sleep entry. Use the DevKitC **left** USB-C UART-C port so the monitor stays connected through deep sleep. Native USB Serial/JTAG drops the port on the first sleep. That drop is not a firmware failure. The build half is T-A1 and does not need a board. The flash half needs hardware, which a host compile cannot open.

### Manual — hardware required

1. **T-M1 — BLE advertisement visible to a scanner.** This cannot be automated: the bytes on the air need a second device acting as a scanner. A host build cannot receive BLE.

   Hardware: DevKitC-1-N8 or XIAO, and a phone with nRF Connect or the same kind of scanner.

   1. From `examples/ble-beacon-deep-sleep/`: `cargo run --release` (add `--no-default-features --features board-xiao-c6` for XIAO).
   2. Open nRF Connect → Scanner → start scanning.
   3. On each wake (~20 s period: 10 s advertise + 10 s sleep), confirm a burst and the name **`ESP32-SDD`**.
   4. Manufacturer data: company ID `0xFFFF`, magic `DE AD BE EF`, little-endian `boot_count` increasing by 1 each wake.
   5. Confirm serial `BLE MAC:` matches the address in the scanner.

   Pass criterion: an advertisement every cycle; name and manufacturer data match the spec. Steps 2–5 need a radio and a second device.

2. **T-M2 — Active window and LED timing.** This cannot be automated: the window and the LED colour or polarity are judged from the physical LED and from serial timestamps. A host build cannot see the LED.

   Hardware: DevKitC-C6 (WS2812) or XIAO (GPIO 15).

   1. LED **off** during init, **on** only while advertising (~10 s), **off** during teardown and sleep. DevKitC is solid blue at a visible 64/255. XIAO is active LOW.
   2. Serial timestamps: adv-start then adv-stop inside the 10 s window. The whole wake is ≤ 12 s before sleep.

   Pass criterion: the LED is dark except during advertising, and the active window is ≤ 12 s.

3. **T-M3 — Button wakeup (DevKitC GPIO 0 / XIAO external GPIO 0).** This cannot be automated: an RTC GPIO wake needs the silicon and a finger on the button. Do **not** use the XIAO BOOT key (GPIO 9).

   Hardware: DevKitC BOOT (GPIO 0), or XIAO with an external active-LOW button on D0 (GPIO 0).

   1. During deep sleep, press the RTC wake button.
   2. Confirm an advertisement and the LED within 500 ms of the press being observed.
   3. Confirm the log shows a GPIO wakeup cause, not a timer cause.

   Pass criterion: GPIO 0 wakes the device. GPIO 9 must not be required. A host build cannot press the pin.

4. **T-M4 — Radio teardown / sleep current (optional).** This cannot be automated: telling milliamps (radio left on) from microamps (torn down) needs a current meter, or at least a look at whether the LED sticks on. A host build cannot measure the rail.

   1. After several cycles, confirm the device keeps cycling (it is not hung in advertising, and the LED is not stuck on).
   2. If a meter is available, confirm the sleep floor is tens of microamps, not milliamps (DevKitC ~7–15 µA, XIAO ~15 µA).

   Pass criterion: when measured, sleep current is microamps-class; otherwise the cycles continue and the LED is not stuck on.

## Spec Files

Behaviour, crates, and checks are edited in the spec files. People do not hand-edit `src/` or this README. Change the spec, then ask the agent (not the shell) **Activate esp32-sdd-full-project-generator**. That overwrites the generated crate and leaves `specs/` in place. See [Using With AI Agents](../../docs/guides/using-with-ai-agents.md).

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md) — observable behaviour (what you see on the LED, the scanner, and the serial log).
- [specs/CodingSpec.md](specs/CodingSpec.md) — architecture and constraints, with no code.
- [specs/TestSpec.md](specs/TestSpec.md) — how to check the example.
