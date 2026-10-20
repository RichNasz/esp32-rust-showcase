<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# 03 — Run Your First Example

This is task A: flash an example that is already in the tree. No agent is required. The crate is `examples/blinky/`. You need the toolchain from [01 — Toolchain Setup](01-toolchain-setup.md).

## 1. Open only the example folder

In VS Code or Cursor: **File → Open Folder… → select `examples/blinky/`**.

This folder is a standalone Cargo package. rust-analyzer needs this crate's `Cargo.toml` and `.cargo/config.toml` (target `riscv32imac-unknown-none-elf`, runner `espflash`). Do not open the repository root as the active workspace while you build the example.

## 2. Connect the DevKitC

Plug USB-C into the DevKitC **left** USB-C port (the UART-C / CH343P bridge). `espflash` uses that port.

## 3. Build, lint, and flash

From `examples/blinky/`:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

The WS2812 on GPIO 8 breathes through a colour sequence (about a 4-second cycle). A plain GPIO level does not light a WS2812; the crate drives it with RMT.

## Optional: XIAO ESP32-C6

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

The XIAO user LED is GPIO 15, active LOW (simple GPIO, not a WS2812).

## If the LED or the log looks wrong

Do not edit `src/`. Behavior is written in `examples/blinky/specs/`. A **spec** is Markdown a person writes; the agent writes the Rust. How to change it is [Using With AI Agents](using-with-ai-agents.md).
