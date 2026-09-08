<!-- See shared-specs/AIGenLessonsLearned.md — read before generating. -->

# Coding Specification — BLE Beacon + Deep Sleep (ESP32-C6)

## Architecture

Single-shot Embassy firmware. Entry is async main from `esp-rtos`; after `esp_hal::init`,
start the Embassy executor on TIMG0 + `FROM_CPU_INTR0` (C6), initialize a heap (justified
below), then run one linear pipeline: persist boot/sequence/BLE address, log wakeup cause,
bring the status LED up **dark**, construct `BleConnector` + TrouBLE host, advertise for
the bounded window with the LED on, drop the advertiser (LED off), stop the host runner,
drop the connector, arm wakeup sources, enter deep sleep. Deep sleep does not return; the
chip resets on wake.

The advertising window is bounded with `embassy_time` (`Timer` / `select`) against the
TrouBLE runner — not a FreeRTOS semaphore or `esp_timer`. When the timer fires, drop the
advertiser (stops advertising), then tear the stack down. `join` of `runner.run()` with
an advertise task that never returns would leave the radio up; prefer `select` so the
runner is cancelled when the window ends.

Top-level wiring lives in `src/main.rs`. Board GPIO, LED kind, polarity, and the RTC wake
pin live only in generated `src/board.rs`. Additional modules for the beacon host path and
status LED are allowed so no file exceeds ~500 lines.

## Board Abstraction

`src/board.rs` is generated from `board-specs/`. It exposes LED GPIO / kind / polarity and
the RTC wake GPIO (GPIO 0 on both boards). `compile_error!` if both `board-devkitc-c6` and
`board-xiao-c6` are enabled, or if neither is. Application code must not hardcode pin
numbers. Chip feature is `esp32c6` only.

Two compile-time LED paths:

- **WS2812** (default `board-devkitc-c6`): RMT 800 kHz NRZ, GRB, cap 64/255. Solid blue
  while advertising; black at init, after stop, and before sleep. Not a pass/fail blink —
  a 10 s solid-on is the advertising-active indicator.
- **GPIO LED** (`board-xiao-c6`): active-LOW binary on while advertising, off otherwise.
  Do not run the RMT encoder on this board.

## Persistence

Store boot count, sequence, and a static-random BLE address in RTC-backed RAM that
survives deep sleep and is lost on power-cycle. Use `#[esp_hal::ram(persistent)]` or the
equivalent `#[ram(unstable(...))]` form required by the pinned `esp-hal` 1.2.1 proc-macro
(`persistent` is an unstable option and, on C6, belongs in RTC fast memory). Types must be
persistable bit-patterns.

Treat uninitialized or corrupted persistent RAM as first boot: distinctive magic plus a
checksum. Increment counters as the first application action after HAL/Embassy/heap init.
Generate the static-random address from the hardware RNG only on first boot (two MSBs of
the most significant octet set, per BLE static-random rules) and reuse it on later wakes
so a scanner can keep the same identity. Do not use NVS or flash for these integers.

Payload fields are 16-bit little-endian; wrap on overflow rather than saturating.

## Sleep and wakeup

Enable `esp-hal` feature `unstable` — sleep, RTC, RMT, and radio need it on this pin.

Arm an RTC timer wakeup for about 10 seconds from sleep entry (absolute deadline so work
between arming and sleep does not shorten sleep). Also arm RTC GPIO 0 as a low-level wake
with a pull-up and the low-power pad path (deep sleep powers down the HP GPIO peripheral).
GPIO 9 is not an RTC wakeup pin.

Enter deep sleep via the 1.2-line `LowPower` / `sleep_deep` API. Panic only if no wakeup
source is armed. Unrecognised wakeup causes log a warning and still run the advertise
pipeline.

**Tear down `esp-radio` before sleep.** Order is mandatory (`shared-specs/ble-patterns.md`):
stop advertising (drop the TrouBLE advertiser), stop the host runner, drop the controller /
`BleConnector` (`esp-radio` tears down on `Drop`), **then** sleep. Skipping teardown leaves
sleep current in milliamps. The deep-sleep skill owns the sleep call; the BLE skill owns
the teardown sequence.

Drive the LED dark before sleep. Allow a brief UART drain after the last log line.

## BLE stack

This is a **radio** example. Git-pin `esp-hal`, `esp-radio` (features `esp32c6`,
`unstable`, `ble`), `esp-rtos` (features `esp32c6`, `embassy`, `esp-radio`), and
`esp-alloc` from tag `esp-hal-v1.2.1`. Never mix crates.io `esp-rtos = "0.4.0"` with git
`esp-hal`. Host is crates.io `trouble-host` **0.7** (upstream `esp-hal` 1.2.1 BLE examples
use that line). Enable TrouBLE **peripheral** + default packet pool; do **not** enable
GATT, scan/central, or security for this broadcaster.

Init order: `esp_hal::init` → heap → `esp_rtos::start` → `BleConnector` + TrouBLE
`ExternalController` + `Host` / `Peripheral`. Radio init is internal to `BleConnector::new`.
Do not emit `nvs_flash_init`, NimBLE, Bluedroid, or `sdkconfig`.

Size `HostResources` for **zero connections** and no L2CAP channels. Advertise as
`Advertisement::NonconnectableScannableUndirected` with intervals via `embassy_time::Duration`
(100 ms). Encode Flags + ManufacturerSpecificData + CompleteLocalName with
`AdStructure::encode_slice` into a 31-byte buffer; if `len` would exceed 31, fail closed
and sleep. Do not hand-pack AD bytes.

CPU clock: max is acceptable for the short BLE window (matches HAL radio examples).

## Heap

TrouBLE + the BLE controller blob allocate. Heap is justified for radio / HCI / host
buffers only: about **72 KiB** internal DRAM (`esp_alloc::heap_allocator!`). Do not use
`#[ram(reclaimed)]` unless the crate also emits an ESP-IDF-compatible app descriptor —
that attribute is rejected without it. `build-std` must include `alloc`. Do not allocate
after init except as the radio / host stack does internally.

## Key Constraints

- Sleep duration: 10 seconds from sleep entry.
- Advertising window: 10 seconds. Whole wake including init + teardown ≤ 12 s.
- ADV PDU ≤ 31 bytes; fail closed on overflow.
- Heap is justified for BLE blobs / TrouBLE only. No `std`. No FreeRTOS. No `sdkconfig`.
- Embassy entry: `#[esp_rtos::main] async fn main(spawner: Spawner) -> !`. Do not use
  blocking `#[esp_hal::main]`.
- `build.rs` emits `-Tlinkall.x` last.
- Zero `unwrap()` / `expect()` on fallible HAL / radio in the wake path. Init may halt if
  clocks or RMT cannot start. BLE start failure logs and sleeps.

## Preferred Crates and APIs

- Git-pinned `esp-hal` / `esp-radio` / `esp-rtos` / `esp-alloc` at `esp-hal-v1.2.1`
  (reason: radio pin; crates.io `esp-radio` does not resolve with HAL 1.2.1).
- `trouble-host` 0.7 peripheral + packet pool (reason: same host as HAL 1.2 BLE examples;
  no NimBLE C).
- `embassy-futures` for `select` of runner vs advertising window (reason: runner must be
  cancelled when the window ends).
- `esp-hal` RTC / `LowPower` for timer + GPIO wakeup and deep sleep.
- `esp-hal` ram persistent attribute for counters and BLE address.
- `esp-hal` RMT for WS2812; `embassy-time` for the window and UART drain.
- `log` + `esp-println` (`log-04`) for INFO operational events.

## Non-Functional Requirements

- Logging: INFO for first boot / timer / GPIO wake + counts, BLE MAC, adv start (name +
  payload summary), adv stop, radio teardown, and sleep entry. WARN for unexpected wakeup
  causes and recoverable advertise failures. ERROR for init failures. No debug in the
  advertising wait.
- Concurrency: TrouBLE runner plus the advertise/window task. Do not spawn reconnect or
  GATT tasks. Do not enable a security manager.
- Error recovery: any failed BLE stage leaves the LED dark, logs, tears down whatever
  was started, and sleeps.
- LED: never on during init or teardown; on only after advertising has actually started.

## Gotchas

- Persistent RAM is retained across deep sleep but cleared on power-on. Checksum/magic is
  required because a reset can skip zero-init. A new random address every wake makes the
  beacon unfindable — persist it.
- GPIO 9 cannot wake C6 from deep sleep. GPIO 0 is the RTC wake pin on both boards; on
  XIAO that is D0 (external button), not the silkscreen BOOT key.
- A GPIO level on the DevKitC WS2812 produces no light. Cap 64/255; blue is the BLE
  activity colour. This is a 10 s solid-on, not a 3-blink pass/fail.
- XIAO LED is active LOW.
- DevKitC serial for persistence tests is the **left** USB-C UART-C port. Native USB
  Serial/JTAG drops the host CDC port on true deep sleep; `espflash monitor` exiting is
  expected.
- Sleep, RTC, RMT, and `esp-radio` live behind `unstable`. `esp-radio` is an unstable beta.
- Do not mix crates.io `esp-rtos` 0.4.0 with git `esp-hal`.
- `sleep_deep` panics if no wakeup source is enabled or if the timer deadline is already
  too near. Arm the 10 s deadline immediately before sleep, after radio teardown.
- Tear down BLE before sleep or the sleep floor is milliamps, not microamps. Drop order
  matters: advertiser, then runner, then `BleConnector`.
- 31-byte PDU: Flags (3) + mfr header (4) + 8-byte payload + local-name header (2) +
  9-byte `"ESP32-SDD"` = 26. Adding another AD type overflows. Use `encode_slice` and
  check `len`.
- `BleConnector::new` before `esp_rtos::start` panics (scheduler not initialized).
- TrouBLE default features pull GATT/central/security; this example is broadcaster-only
  and should not enable those.
- Static random addresses need the two MSBs of the most significant octet set; omitting
  that can make scanners drop the device as a non-static random.
