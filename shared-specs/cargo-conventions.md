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
log = "0.4"

[features]
default = ["board-devkitc-c6"]
board-devkitc-c6 = []
board-xiao-c6 = []
```

Radio examples **must not** mix crates.io crates from the `esp-hal` monorepo with git pins. Do **not** paste crates.io `esp-radio = "1.0.0-beta.0"` next to crates.io `esp-hal = "1.2.1"` (that crates.io radio crate depends on `esp-hal ~1.1.0` and will not resolve). Do **not** paste crates.io `esp-rtos = "0.4.0"` next to git `esp-hal`. Git-pin `esp-hal`, `esp-radio`, `esp-rtos`, and `esp-alloc` (when heap is justified) from tag `esp-hal-v1.2.1` so path deps unify (`rust-toolchain.md`):

```toml
esp-hal = { git = "https://github.com/esp-rs/esp-hal", tag = "esp-hal-v1.2.1", features = ["esp32c6", "unstable"] }
esp-radio = { git = "https://github.com/esp-rs/esp-hal", tag = "esp-hal-v1.2.1", features = ["esp32c6", "unstable"] }
esp-rtos = { git = "https://github.com/esp-rs/esp-hal", tag = "esp-hal-v1.2.1", features = ["esp32c6", "embassy", "esp-radio"] }
# when the CodingSpec justifies a heap:
esp-alloc = { git = "https://github.com/esp-rs/esp-hal", tag = "esp-hal-v1.2.1" }
embassy-executor = "0.10"
embassy-time = "0.5"
# plus wifi and/or ble and/or esp-now on esp-radio, matching the example
```

Rules:

- Pin versions from `rust-toolchain.md` (looked up 2026-09-08). Do not use `"*"`.
- Enable `esp-hal` feature `"unstable"` **in addition to** `"esp32c6"` when radio or sleep needs it.
- `esp-rtos` (features `esp32c6` + `embassy`) is the Embassy integration path. Time is `embassy_time`. Radio examples also enable `esp-rtos` feature `esp-radio` and git-pin `esp-rtos` at tag `esp-hal-v1.2.1`. Non-radio examples use crates.io `esp-rtos` 0.4.0.
- `esp-radio` is **only** for radio examples. Git pin at tag `esp-hal-v1.2.1` with the rest of the monorepo stack — see `rust-toolchain.md`. Treat it as an unstable beta. Enable `wifi` / `ble` / `esp-now` features as the example requires.
- `esp-alloc` is **only** when the CodingSpec justifies a heap. Radio examples that need it git-pin it from the same tag; non-radio examples use crates.io 0.11.0.
- `esp-bootloader-esp-idf` and `esp-storage` are **only** when the example needs IDF-compatible image metadata or flash storage.
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
| crates.io `esp-radio = "1.0.0-beta.0"` with crates.io `esp-hal = "1.2.1"` | crates.io radio depends on `esp-hal ~1.1.0` and will not resolve |
| crates.io `esp-rtos = "0.4.0"` with git `esp-hal` | crates.io `esp-rtos` pulls crates.io `esp-hal`; Cargo then builds two HAL packages |
