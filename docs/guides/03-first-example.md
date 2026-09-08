<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# 03 — Run Your First Example

Build and flash `blinky` on the required board: **ESP32-C6-DevKitC-1-N8**.

## 1. Open only the example folder

In VS Code or Cursor: **File → Open Folder… → select `examples/blinky/`**.

This folder is a complete standalone Cargo package. rust-analyzer needs this crate's `Cargo.toml` and `.cargo/config.toml` (RISC-V target + `espflash` runner). Do not open the repository root as the active workspace while developing the example.

## 2. Connect hardware

Plug USB-C into the DevKitC **left** (UART-C) port. That port is a UART bridge; `espflash monitor` stays connected even in later deep-sleep examples.

## 3. Build, lint, flash

From `examples/blinky/`:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

The WS2812 on GPIO 8 should breathe through a colour sequence over a 4-second period.

## Optional: XIAO ESP32-C6

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

The XIAO user LED is GPIO 15, active LOW, simple GPIO (not WS2812).

## If something looks wrong

Do **not** edit generated `src/` or `Cargo.toml`. Improve `examples/blinky/specs/` and regenerate with `esp32-sdd-full-project-generator`.

Next: [04 — Create a New Example](04-new-example.md).
