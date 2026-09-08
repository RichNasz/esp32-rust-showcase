# Coding Conventions

**Classification:** Human-authored shared spec
**Applies to:** All Rust firmware generated for examples in this repository

---

## Language and Toolchain

- **Language:** Rust, edition **2024** (MSRV 1.95.0 allows it).
- Format with `rustfmt`. Lint with `cargo clippy -- -D warnings`. Generated crates must compile with **zero `rustc` warnings** on the C6 target (`riscv32imac-unknown-none-elf`).
- Every firmware crate is `#![no_std]` `#![no_main]`.
- Do not pull in `std`. Heap (`esp-alloc` / `alloc`) is allowed **only** when the example's `CodingSpec.md` justifies it (typical justifications: MQTT JSON, TLS buffers). After init, do not allocate on the heap unless that justification covers the allocation.

---

## Naming Conventions

| Element | Convention | Example |
|---|---|---|
| Functions, methods, modules, local bindings | `snake_case` | `sensor_read_temperature()` |
| Constants and `static`s | `SCREAMING_SNAKE_CASE` | `SENSOR_READ_INTERVAL_MS` |
| Types, traits, enums | `UpperCamelCase` | `SensorConfig` |
| Type parameters | `UpperCamelCase`, short | `T`, `Driver` |
| Crate / package name | kebab folder → underscores in `Cargo.toml` `name` | folder `deep-sleep-timer-node` → `deep_sleep_timer_node` |
| Embassy task functions | `snake_case` | `sensor_poll_task` |
| Cargo features | kebab-case | `board-devkitc-c6` |

---

## Error Handling

- No `unwrap()` or `expect()` on fallible HAL / radio / network calls in production paths.
- Initialization **may** panic (or `esp_hal::halt`) if the process cannot continue (clocks, radio bring-up, missing board feature).
- Runtime loops check `Result` / `Option` and log without aborting unless the error is unrecoverable.
- Prefer `?` in functions that already return `Result`. Do not discard errors with `let _ = ...` in production paths.

---

## Concurrency and Time

- Use **Embassy tasks** instead of FreeRTOS tasks. Time via `embassy_time` (`Timer`, `Duration`, `Instant`).
- Call `into_async` on a peripheral **only on the Embassy executor / core that owns that driver**. Do not share a blocking handle and an async handle for the same peripheral.
- Do not introduce FreeRTOS, `std::thread`, or ESP-IDF `esp_timer` task APIs.

---

## Logging

- Log via `esp-println` plus the `log` crate (`log-04` feature on `esp-println`).
- Use log levels appropriately: `log::debug!` for verbose debug, `log::info!` for normal operational events, `log::warn!` for recoverable anomalies, `log::error!` for errors.
- Do not leave `debug!` in hot paths (ISRs, tight loops) in production.
- **Never log Wi-Fi / MQTT / OTA secrets** (PSKs, tokens, private keys, session keys) at any log level.

---

## Memory Management

- Prefer `'static` buffers, stack allocation, and `heapless` collections.
- If the CodingSpec justifies a heap: initialize `esp-alloc` once at boot, size the heap from the spec, and never allocate after init unless the spec says so.
- No memory leaks in the steady-state loop. Free in reverse allocation order when the heap is used.

---

## Radio and Sleep

- Wi-Fi, BLE, ESP-NOW, and several sleep / RTC APIs require the `esp-hal` **`unstable`** feature. Enable it when needed and document that honestly in the example README.
- Tear down `esp-radio` (stop + deinit) before deep sleep so the radio is not left powered.
- For LP-core examples: load the LP-core image **before** the HP core enters sleep.

---

## Code Layout

- `src/main.rs` is the entry point and top-level wiring only (`#![no_std]` `#![no_main]`).
- `src/board.rs` is generated from `board-specs/` (GPIO / LED / button). Application code must not hardcode pin numbers.
- Additional modules live at `src/<subsystem>.rs` (or `src/<subsystem>/mod.rs` if the module splits further).
- Maximum file length: ~500 lines. Split if exceeded.
- Maximum function length: ~80 lines. Extract sub-functions if exceeded.

---

## Comments and Documentation

- Comments explain **why**, not **what**. Code is self-documenting through good naming.
- Generated Rust files carry `// AGENT-GENERATED — DO NOT EDIT BY HAND` on line 1.
- Generated Markdown files carry the standard AGENT-GENERATED header.
- Human-authored files carry no agent header (or a human-authored marker as in `AIGenLessonsLearned.md`).
- Non-obvious algorithms or hardware workarounds must include a comment referencing the datasheet section or errata note.

---

## Forbidden Patterns

| Pattern | Why forbidden |
|---|---|
| `unwrap()` / `expect()` on HAL/radio in production | Hide recoverable failures; panic loops on-device |
| Logging credentials | Secret leakage over USB-UART / USB Serial/JTAG |
| Hardcoded GPIO numbers | Board-specs are the source of truth |
| `std`, `idf.py`, `main.c`, CMake, `sdkconfig` | This is rust-on-metal, not ESP-IDF |
| FreeRTOS tasks | Embassy tasks only |
| Heap after init without CodingSpec justification | Unbounded RAM growth on a no_std target |
