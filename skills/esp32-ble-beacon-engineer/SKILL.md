---
name: esp32-ble-beacon-engineer
description: Designs and generates BLE advertising/beaconing for ESP32-C6 rust-on-metal firmware using esp-radio + TrouBLE. Handles non-connectable advertising, manufacturer data PDUs, bounded advertising windows via embassy_time, the crates.io radio pin, and full teardown before deep sleep.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, ble, trouble, esp-radio, beacon, advertising, manufacturer-data, deep-sleep, power-budget, rust
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 BLE Beacon Engineer

## When to use
Any time a FunctionalSpec requires BLE advertising, beaconing, manufacturer data payloads, or iBeacon/Eddystone frames. Invoked automatically as a sub-workflow by `esp32-sdd-full-project-generator` when BLE advertising or beaconing is specified. Can also be run standalone.

v1 BLE is rust-on-metal: `esp-radio` (controller) + **TrouBLE** (`trouble-host`). There is no NimBLE C stack, no Bluedroid, and no `CONFIG_BT_*`.

## Workflow (follow exactly)
MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else. Then follow `shared-specs/ble-patterns.md` as the BLE source of truth.

1. **Read FunctionalSpec BLE requirements**: Extract advertising type (connectable vs. non-connectable), payload format (manufacturer data, iBeacon, Eddystone), advertising interval, advertising window duration, and whether deep sleep follows the advertising window.

2. **Select the stack**: TrouBLE + `esp-radio` for every BLE example in v1, including beacons and beacon+sleep. Do not generate NimBLE C, Bluedroid, or sdkconfig.

3. **Pin the radio stack** by copying the **Radio pin** in `shared-specs/rust-toolchain.md` (crates.io exact versions; do not git-pin the HP crates). Include `esp-bootloader-esp-idf` with the other radio crates, and invoke `esp_app_desc!()` in `src/main.rs`. Do not write a shorter pin list. Enable:
   - `esp-hal` features `esp32c6` + `unstable`
   - `esp-radio` features `esp32c6` + `ble` + `unstable`
   - `esp-rtos` features `esp32c6` + `embassy` + `esp-radio`
   Document in the README that `esp-radio` is an unstable beta.

4. **Heap**: TrouBLE + radio blobs typically need `esp-alloc`. The example CodingSpec **must** justify the heap. Use crates.io `esp-alloc` `=0.11.0` from the same radio pin. Do not allocate after init unless that justification covers it. Set `.cargo/config.toml` `build-std = ["core", "alloc"]`.

5. **Init order** (`ble-patterns.md`):
   1. `esp_hal::init`
   2. Start `esp-rtos` (`esp_rtos::start` on TIMG0 + `FROM_CPU_INTR0` on C6) **before** constructing `BleConnector`
   3. `BleConnector` + TrouBLE `ExternalController` + `Host` / `Peripheral`
   4. `join` the TrouBLE `runner.run()` task with the advertise task

   Radio init is internal to `BleConnector::new` / the radio types. Do not generate `nvs_flash_init()` C calls or ESP-IDF controller APIs.

6. **Non-connectable advertising** for beacons:
   - `Advertisement::NonconnectableScannableUndirected`
   - Intervals via `embassy_time::Duration` (100 ms → `Duration::from_millis(100)`)
   - Size TrouBLE `HostResources` for **zero connections** when the example never accepts a link
   - Do not enable a security manager / bonding path for a non-connectable beacon

7. **Manufacturer data PDU** — total advertising payload must not exceed **31 bytes**. Encode with TrouBLE `AdStructure::encode_slice` (Flags + ManufacturerSpecificData). Max application payload is 24 bytes after Flags (3) + mfr header (4). Fail closed if `len` would exceed 31. Reference `shared-specs/ble-patterns.md` for the byte map.

8. **Bounded advertising window** with `embassy_time` (`Timer` / `select`), not a FreeRTOS semaphore or `esp_timer`. When the timer fires, drop the advertiser (stops advertising). TrouBLE callbacks run on the Embassy executor.

9. **Teardown before deep sleep** (mandatory). Deviation leaves the radio powered:

   1. Stop advertising (drop the TrouBLE `Advertiser`).
   2. Stop the TrouBLE host runner (`runner.run()`).
   3. Drop the controller / `BleConnector` (`esp-radio` tears down on `Drop`).
   4. **Then** enter deep sleep. The `esp32-deep-sleep-engineer` sub-workflow owns the sleep call — do not duplicate it here.

   After wakeup the chip resets; bring the scheduler and radio up again from `main`.

10. **GPIO hold** (if used for peripheral power gating) is applied after BLE teardown and before deep sleep (deep-sleep engineer).

11. **Document in the example README**:
    - TrouBLE + `esp-radio` choice; unstable-beta honesty; crates.io `esp-radio` `=1.0.0-beta.1` with `esp-hal` `=1.2.1`
    - Advertising payload byte map (table: offset, length, value, description)
    - Power budget breakdown: sleep current (µA) + active window current (mA) × duration (ms) = average µA, including BLE teardown time (~5–10 ms)
    - Reference to `shared-specs/ble-patterns.md`
