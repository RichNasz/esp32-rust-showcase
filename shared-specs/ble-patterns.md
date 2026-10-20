# BLE Patterns

**Classification:** Human-authored shared spec
**Applies to:** All examples that include BLE advertising, beaconing, or manufacturer data payloads.

v1 BLE is rust-on-metal: `esp-radio` (controller) + **TrouBLE** (`trouble-host`, host). There is no NimBLE C stack, no Bluedroid, no `sdkconfig`, and no `CONFIG_BT_*` Kconfig.

---

## Stack Selection

| Criterion | TrouBLE + `esp-radio` | Anything else |
|---|---|---|
| Beacon / deep-sleep use case | **Required** in v1 | Not in v1 |
| Host | `trouble-host` (Embassy async) | Do not generate NimBLE C or Bluedroid |
| Controller | `esp-radio` BLE (`BleConnector` + TrouBLE `ExternalController`) | Do not call ESP-IDF controller APIs |
| GATT server / client | Supported (TrouBLE GATT) | — |
| Classic BT (A2DP, SPP, HFP) | Not supported | Not in v1 |

**Decision rule:** Use TrouBLE + `esp-radio` for every BLE example in v1, including beacons and beacon+sleep. Do not emit NimBLE C, Bluedroid, `sdkconfig`, or `CONFIG_BT_*`.

Pins: copy the **Radio pin** in `rust-toolchain.md` for every monorepo crate this example uses, including `esp-bootloader-esp-idf` and `esp_app_desc!()`. Do not write a shorter list. Upstream `esp-hal` 1.2.1 BLE examples use `trouble-host` **0.7**. Enable `esp-hal` `unstable`, `esp-radio` features `esp32c6` + `ble` + `unstable`, and `esp-rtos` features `esp32c6` + `embassy` + `esp-radio`.

Start the `esp-rtos` scheduler **before** constructing `BleConnector`. Radio init is internal to `BleConnector::new` / the radio types; there is no public `esp_radio::init()` on this HAL line.

---

## Non-Connectable Advertising — Parameter Template

Beacon / manufacturer-data examples advertise as **non-connectable**. TrouBLE takes wall-clock intervals (`embassy_time::Duration`); the controller still encodes BLE advertising intervals in 0.625 ms units internally.

```rust
let mut params = AdvertisementParameters::default();
params.interval_min = Duration::from_millis(100);
params.interval_max = Duration::from_millis(100);

let advertiser = peripheral
    .advertise(
        &params,
        Advertisement::NonconnectableScannableUndirected {
            adv_data: &adv_data[..len],
            scan_data: &[],
        },
    )
    .await?;
```

Common intervals (BLE spec: `units = desired_ms / 0.625`):

- 100 ms → 160 units → `Duration::from_millis(100)`
- 500 ms → 800 units
- 1000 ms → 1600 units

Connectable GATT examples use `Advertisement::ConnectableScannableUndirected` instead. Beacons must not.

---

## Advertising Payload Layout

The total advertising PDU is capped at **31 bytes**. Typical beacon layout:

| Offset | Length | Field | Value |
|---|---|---|---|
| 0 | 2 | Flags AD type + length | `0x02, 0x01` |
| 2 | 1 | Flags value | `0x06` (LE General Discoverable \| BR/EDR Not Supported) |
| 3 | 1 | Manufacturer data length | `N+3` (1 type + 2 company ID + N payload) — encoded as `N+3` in the length byte; AD type `0xFF` follows |
| 4 | 1 | Manufacturer data AD type | `0xFF` |
| 5 | 2 | Company ID (little-endian) | e.g. `0xFF, 0xFF` (test/internal) |
| 7 | N | Application payload | sensor data, sequence number, etc. |

Maximum application payload: `31 - 7 = 24 bytes` with the header above.

Encode with TrouBLE rather than hand-packing bytes:

```rust
let mut adv_data = [0u8; 31];
let len = AdStructure::encode_slice(
    &[
        AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
        AdStructure::ManufacturerSpecificData {
            company_identifier: 0xFFFF,
            payload: &app_payload,
        },
    ],
    &mut adv_data[..],
)?;
```

`AdStructure::encode_slice` writes Flags as `02 01 06` and manufacturer data as `[N+3, 0xFF, company_id_le, payload…]`. Count the resulting `len` and fail closed if it would exceed 31.

For iBeacon layout (Apple proprietary), prepend the iBeacon subtype `0x02, 0x15` after the company ID and follow with 16-byte UUID + 2-byte major + 2-byte minor + 1-byte TX power (still inside the 31-byte PDU).

---

## BLE Stack Teardown Sequence

This order is required before deep sleep. Deviation leaves the radio powered (sleep current in milliamps instead of microamps).

1. **Stop advertising.** Drop the TrouBLE `Advertiser` (or otherwise stop `peripheral.advertise`). Safe if already stopped.
2. **Stop the TrouBLE host runner.** Return from / cancel the `runner.run()` task so the host is no longer servicing HCI.
3. **Drop the controller / `BleConnector`.** `esp-radio` tears down on `Drop`. Do not leave the connector alive across sleep.
4. **Then** enter deep sleep (`esp-hal` RTC sleep). Do not call ESP-IDF `esp_bt_controller_*` or C host-stack port APIs — those symbols do not exist here.

`esp-radio` must be started only after `esp_rtos::start(...)`. After wakeup the chip resets; bring the scheduler and radio up again from `main`.

**Why this order matters:** advertising stop is host-side; dropping the connector is what powers down the controller. Sleeping while the connector is live leaves the radio on.

---

## Power Save Integration

### Modem Sleep vs. Deep Sleep

| Mode | Use when | BLE teardown needed? |
|---|---|---|
| Connection-interval / controller duty cycle | BLE connection stays alive between events | No — controller manages radio duty cycle |
| Deep sleep | No BLE activity during sleep period | **Yes** — full teardown required |

### Typical Sleep Currents by Chip (deep sleep, RTC timer wakeup)

| Chip | Deep Sleep Current |
|---|---|
| ESP32-C6 | ~7 µA (with LP core off) |

The figure is approximate. Use the C6 board-spec power row for the board you ship. Do not copy another SoC's sleep current.

### Power Budget Formula

```
avg_current_µA = (active_mA × 1000 × active_window_ms + sleep_µA × sleep_period_ms)
                 / (active_window_ms + sleep_period_ms)
```

Include BLE teardown time (~5–10 ms) in `active_window_ms`.

---

## Advertising Window Timeout — Embassy Timer

TrouBLE callbacks run on the Embassy executor, not on a FreeRTOS host task. Time the advertising window with `embassy_time` (`Timer` / `select`), not a FreeRTOS semaphore or `esp_timer`.

Pattern:

1. Start `esp-rtos` (`esp_rtos::start` on TIMG0 + `FROM_CPU_INTR0` on C6).
2. Construct `BleConnector` + `ExternalController` + TrouBLE `Host` / `Peripheral`.
3. `join` the TrouBLE `runner.run()` task with an async block that calls `peripheral.advertise(...)`.
4. Bound the window with `Timer::after(...)` or `select` against the advertiser. When the timer fires, drop the advertiser (stops advertising).
5. Tear down (see above), then deep sleep.

Heap (`esp-alloc`) is typical for TrouBLE + radio blobs; the example CodingSpec must justify it (radio / HCI / host buffers). Do not allocate after init unless that justification covers it.

Do not generate NVS-as-Kconfig or `nvs_flash_init()` C calls. PHY / radio init is `esp-radio`'s job. Persist app data with `esp-storage` only when the CodingSpec says so.

---

## Broadcaster-Only Role (no scanning, no connections)

There is no `sdkconfig` and no Bluetooth Kconfig. Broadcaster-only is expressed in Rust:

- Use `Advertisement::NonconnectableScannableUndirected` (or non-connectable equivalent).
- Size TrouBLE `HostResources` for **zero connections** when the example never accepts a link (`CONNECTIONS_MAX = 0`, no GATT server).
- Do not enable TrouBLE central/scan features unless the FunctionalSpec requires scanning.
- Do not generate a security manager / bonding path for a non-connectable beacon.

BLE 5 extended advertising is optional and only when the FunctionalSpec asks for it (ESP32-C6 supports it). Prefer legacy 31-byte PDUs for beacons unless extended adv is specified.

---

## Common Gotchas

| Issue | Root Cause | Fix |
|---|---|---|
| Duplicate `esp-hal` / radio resolve failure | git `esp-hal` mixed with crates.io `esp-radio` or `esp-rtos`, or `esp-radio` `1.0.0-beta.0` next to HAL 1.2.1 | Use the crates.io radio pin in `rust-toolchain.md`: `esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1`, `esp-rtos` `=0.4.0`, `esp-alloc` `=0.11.0` |
| Panic: scheduler not initialized | `BleConnector::new` before `esp_rtos::start` | Start `esp-rtos` first (C6: timer + `FROM_CPU_INTR0`) |
| Radio left powered in deep sleep | Advertiser / `BleConnector` still live | Stop advertising, stop runner, drop connector, then sleep |
| 31-byte PDU overflow | Payload + headers exceed limit | Count bytes: Flags (3) + Mfr header (4) = 7 bytes consumed before payload (max 24); use `AdStructure::encode_slice` and check `len` |
| GPIO state lost after wakeup | Output not held across sleep | Use `esp-hal` GPIO hold / RTC hold after BLE teardown, before deep sleep |
| High average current | BLE teardown skipped | Always execute the full teardown sequence before deep sleep |
| Boot button not usable for deep-sleep wakeup on ESP32-C6 | GPIO 9 (XIAO boot button) is not an RTC GPIO; only GPIO 0–7 support ext1/LP wakeup | Timer-only wakeup on that board, or an external button on LP GPIO 0–7. Document in the Supported Boards table |
| Mixing blocking and async radio handles | `into_async` on a different executor than the owner | `into_async` only on the Embassy executor / core that owns the driver |
| Generated NimBLE C / `sdkconfig` | Reading stale IDF docs | This file is the BLE source of truth: TrouBLE + `esp-radio` only |
