# Cargo Conventions

**Classification:** Human-authored shared spec
**Applies to:** All `Cargo.toml`, `.cargo/config.toml`, and `build.rs` files generated for examples in this repository

Crate versions are pinned in `shared-specs/rust-toolchain.md`. Generators copy those pins at implementation time — do not invent versions.

---

## Package Identity

Examples live at `examples/<kebab-name>/` as **standalone binary crates** (not a Cargo workspace).

- Folder name: kebab-case (`deep-sleep-timer-node`).
- `Cargo.toml` `name`: the same string with hyphens converted to underscores (`deep_sleep_timer_node`).
- `edition = "2024"`.
- `publish = false`.
- License: MIT. Copyright holder: Richard Naszcyniec.

There is **no** `Cargo.toml` at the repository root and **no** `[workspace]` table anywhere in v1.

---

## Cargo.toml — Dependencies and Features

Every example depends on `esp-hal` with feature `esp32c6` (the only chip feature in v1) and on `esp-rtos` + Embassy (`embassy-executor`, `embassy-time`). Add `esp-hal` `unstable` when the example uses radio (Wi-Fi / BLE / ESP-NOW) or sleep / RTC APIs that require it.

Non-radio sample (crates.io `esp-hal` 1.2.1):

```toml
[package]
name = "<underscore_name>"
version = "0.1.0"
edition = "2024"
license = "MIT"
publish = false

[dependencies]
esp-hal = { version = "1.2.1", features = ["esp32c6"] }
esp-rtos = { version = "0.4.0", features = ["esp32c6", "embassy"] }
embassy-executor = "0.10"
embassy-time = "0.5"
esp-backtrace = { version = "0.20.0", features = ["esp32c6", "panic-handler", "println"] }
esp-println = { version = "0.18.0", features = ["esp32c6", "log-04"] }
esp-bootloader-esp-idf = { version = "0.6.0", features = ["esp32c6"] }
log = "0.4"

[features]
default = ["board-devkitc-c6"]
board-devkitc-c6 = []
board-xiao-c6 = []
```

Radio examples use the crates.io radio pin in `rust-toolchain.md`. Do **not** paste crates.io `esp-radio = "1.0.0-beta.0"` next to `esp-hal` 1.2.1 (beta.0 depends on `esp-hal ~1.1.0`). Do **not** mix a git `esp-hal` pin into an HP radio crate. The nested LP crate still git-pins `esp-lp-hal` and does not pull the HP crates with it.

```toml
esp-hal = { version = "=1.2.1", features = ["esp32c6", "unstable"] }
esp-radio = { version = "=1.0.0-beta.1", features = ["esp32c6", "unstable"] }
esp-rtos = { version = "=0.4.0", features = ["esp32c6", "embassy", "esp-radio"] }
esp-bootloader-esp-idf = { version = "=0.6.0", features = ["esp32c6"] }
# when the CodingSpec justifies a heap:
esp-alloc = { version = "=0.11.0" }
embassy-executor = "0.10"
embassy-time = "0.5"
# plus wifi and/or ble and/or esp-now and/or ieee802154 and/or coex on esp-radio, matching the example
```

Rules:

- Pin versions from `rust-toolchain.md` (radio pin rechecked 2026-10-20). Do not use `"*"`.
- Enable `esp-hal` feature `"unstable"` **in addition to** `"esp32c6"` when radio or sleep needs it.
- `esp-rtos` (features `esp32c6` + `embassy`) is the Embassy integration path. Time is `embassy_time`. Radio examples also enable `esp-rtos` feature `esp-radio`. Radio and non-radio both use crates.io `esp-rtos` `=0.4.0`.
- `esp-radio` is **only** for radio examples. crates.io `=1.0.0-beta.1` with crates.io `esp-hal` `=1.2.1`. Treat it as an unstable beta. Enable `wifi` / `ble` / `esp-now` / `ieee802154` / `coex` as the example requires. Raw IEEE 802.15.4 and Thread do not enable `wifi`: that driver does not coexist with Wi-Fi. Matter over Wi-Fi enables `wifi`, `ble`, and `coex`, and does not enable `ieee802154`.
- `esp-alloc` is **only** when the CodingSpec justifies a heap. crates.io `=0.11.0`.
- `esp-bootloader-esp-idf` is **required on every example** (feature `esp32c6`, crates.io `=0.6.0`). Current `espflash` will not flash an `esp-hal` image that lacks the ESP-IDF-compatible application descriptor. Invoke `esp_bootloader_esp_idf::esp_app_desc!()` in `src/main.rs`. This crate is **not** ESP-IDF.
- `esp-storage` is **only** when the example needs flash storage. crates.io `=0.10.0`.
- Board features `board-devkitc-c6` (default) and `board-xiao-c6` are **mutually exclusive**. Generated `src/board.rs` (or `main.rs`) must `compile_error!` if both are enabled or if neither is enabled.
- Do not add a chip feature other than `esp32c6` in v1.

---

## .cargo/config.toml

Every example must ship this file (values are required, not suggestions):

```toml
[build]
target = "riscv32imac-unknown-none-elf"

[target.riscv32imac-unknown-none-elf]
runner = "espflash flash --monitor"

[unstable]
build-std = ["core"]
```

- If the CodingSpec justifies `alloc` / `esp-alloc`, use `build-std = ["core", "alloc"]`.
- Custom partition tables (OTA, extra NVS): pass `--partition-table partitions.csv` on the runner. See `partition-table-profiles.md`. Do not use `sdkconfig` / `CONFIG_PARTITION_TABLE_*`.
- Do not set a different `--target` or runner. Do not use `probe-rs` or `idf.py` as the runner.
- Do not commit a `config.toml` that selects Xtensa targets.

---

## Toolchain file

Do **not** generate a per-example `rust-toolchain.toml`. It is not in the locked layout (`specs/example-project-structure-spec.md`). Channel, MSRV, target, and components are specified in `shared-specs/rust-toolchain.md` (rustup **stable**, not `esp` / `espup`).

---

## build.rs

Every example has a `build.rs`. Use it for linker arguments, `esp-hal` build-time config, and (when applicable) embedding an LP-core image. Do not shell out to `idf.py` or CMake.

---

## Build, Lint, and Flash Commands

Primary commands (from the example directory):

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
cargo run --release --no-default-features --features board-xiao-c6
```

`cargo run --release` uses the configured runner (`espflash flash --monitor`). Wokwi is an allowed tier-1 alternative when TestSpec says so.

---

## Forbidden Patterns

| Pattern | Why forbidden |
|---|---|
| Workspace at the repo root | Examples are standalone crates |
| Committed `sdkconfig` / `sdkconfig.defaults` | There is no ESP-IDF; there is no sdkconfig |
| `main.c`, `CMakeLists.txt`, `idf_component.yml` | Rust / Cargo only |
| `idf.py` instructions in generated READMEs | Primary build tool is Cargo + espflash |
| `channel = "esp"` or `espup` | v1 uses rustup stable |
| Chip features other than `esp32c6` | Other SoCs are not in v1 |
| Both board features enabled without `compile_error!` | Pins would be silently wrong |
| crates.io `esp-radio = "1.0.0-beta.0"` with crates.io `esp-hal = "1.2.1"` | beta.0 depends on `esp-hal ~1.1.0` and will not resolve. Use `=1.0.0-beta.1` |
| git `esp-hal` mixed with crates.io `esp-radio` / `esp-rtos` on an HP crate | Cargo builds two HAL packages. Radio HP crates are crates.io. LP `esp-lp-hal` stays on the git tag |
| `esp-radio` `1.0.0-beta.2` | Yanked |
| Firmware without `esp-bootloader-esp-idf` / `esp_app_desc!()` | Current `espflash` refuses to flash (`ESP-IDF App Descriptor missing`) |
