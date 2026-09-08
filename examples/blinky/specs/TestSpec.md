# Testing Specification — Blinky (WS2812 + GPIO breathing LED)

## Testing Philosophy

Automated testing using native Cargo / rustc tools is strongly preferred.
Additional tools are only allowed with explicit justification and documented benefits.
Manual testing is used only when no practical automated alternative exists.

Do not specify `idf.py`, ESP-IDF Unity, or CMake-based harnesses.

---

## Automated Tests

### T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)

**Tool**: `cargo` (native; rustup stable + `riscv32imac-unknown-none-elf` + `rust-src`)
**What it verifies**: The crate compiles cleanly for the default board (DevKitC-C6, WS2812 / RMT path) with no `rustc` or clippy warnings.

```sh
cd examples/blinky
cargo build --release
cargo clippy -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)

**Tool**: `cargo` (native)
**What it verifies**: The GPIO / LEDC path compiles cleanly when the XIAO board feature is selected.

```sh
cd examples/blinky
cargo build --release --no-default-features --features board-xiao-c6
cargo clippy --no-default-features --features board-xiao-c6 -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A3 — Flash via espflash runner (DevKitC-C6)

**Tool**: `cargo run` with the crate runner (`espflash flash --monitor`)
**What it verifies**: The image flashes; serial log shows WS2812 / RMT init and breathing loop start.

```sh
cd examples/blinky
cargo run --release
```

Pass criterion: flash succeeds; monitor shows LED-driver init and loop start.

Hardware required for the flash step. The *build* half of this crate is covered by T-A1 without a board.

---

## Manual Tests

*Manual tests are used only when no practical automated alternative exists. Each step explains
why automation is not applicable.*

### T-M1 — Visual WS2812 breathing (ESP32-C6-DevKitC-1-N8)

**Why manual**: WS2812 protocol errors produce no light or a solid-on LED. A build cannot
certify perceptual smoothness or colour order.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8, USB-C on the **left** UART-C port
(UART bridge; typically `/dev/cu.usbserial-*` on macOS).

Steps:

1. From `examples/blinky/`: `cargo run --release`
2. Confirm serial output mentions WS2812 / RMT init on GPIO 8, GRB order, and brightness cap 64/255.
3. Observe the onboard RGB LED.
4. Confirm it starts dark, then breathes through the colour sequence (red, green, blue, cyan, magenta, amber) over approximately 4 seconds per cycle, with no stuck-on or stuck-off.

Pass criterion: LED breathes smoothly through colours; serial log confirms RMT init.

---

### T-M2 — Visual GPIO breathing (Seeed XIAO ESP32-C6)

**Why manual**: Active-LOW polarity and fade quality require human observation.

Hardware required: Seeed XIAO ESP32-C6, USB-C cable.

Steps:

1. From `examples/blinky/`: `cargo run --release --no-default-features --features board-xiao-c6`
2. Confirm serial output shows the GPIO LED path (not WS2812 / RMT) on GPIO 15, active-LOW.
3. Observe the blue user LED on GPIO 15.
4. Confirm it starts dark, then brightens, and breathes smoothly over approximately 4 seconds per cycle.

Pass criterion: correct polarity, smooth breathing, no WS2812 init in the log.
