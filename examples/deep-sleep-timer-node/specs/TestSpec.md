# Testing Specification — Deep Sleep Timer Node (ESP32-C6)

## Testing Philosophy

Automated testing using native Cargo / rustc tools is strongly preferred.
Additional tools are only allowed with explicit justification and documented benefits.
Manual testing is used only when no practical automated alternative exists.

Do not specify `idf.py`, ESP-IDF Unity, or CMake-based harnesses.

---

## Automated Tests

### T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)

**Tool**: `cargo` (native; rustup stable + `riscv32imac-unknown-none-elf` + `rust-src`)
**What it verifies**: The crate compiles cleanly for the default board (DevKitC-C6, WS2812 / RMT + RTC sleep path) with no `rustc` or clippy warnings.

```sh
cd examples/deep-sleep-timer-node
cargo build --release
cargo clippy --release -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)

**Tool**: `cargo` (native)
**What it verifies**: The GPIO LED + RTC sleep path compiles cleanly when the XIAO board feature is selected.

```sh
cd examples/deep-sleep-timer-node
cargo build --release --no-default-features --features board-xiao-c6
cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A3 — Flash via espflash runner (DevKitC-C6)

**Tool**: `cargo run` with the crate runner (`espflash flash --monitor`)
**What it verifies**: The image flashes; serial log shows first boot or a timer wake, LED-driver init, and sleep entry.

```sh
cd examples/deep-sleep-timer-node
cargo run --release
```

Pass criterion: flash succeeds; monitor shows boot/wake log and sleep entry.

Hardware required for the flash step. The *build* half of this crate is covered by T-A1 without a board.

Use the **left** USB-C UART-C port so the monitor stays connected through deep sleep. Native USB Serial/JTAG will drop the port on the first sleep; that is not a firmware failure.

---

## Manual Tests

*Manual tests are used only when no practical automated alternative exists. Each step explains
why automation is not applicable.*

### T-M1 — Boot counter persistence through real deep sleep

**Why manual**: Persistent RAM depends on the hardware RTC domain staying live through a true
deep-sleep state. A host build cannot certify retention.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8, USB-C on the **left** UART-C port
(UART bridge; typically `/dev/cu.usbserial-*` on macOS) so the monitor stays connected.

Steps:

1. From `examples/deep-sleep-timer-node/`: `cargo run --release`
2. Observe several consecutive wakeups (at least five). Confirm the boot counter increments
   by 1 each cycle with no gaps, and that timer wakeup is logged after the first boot.
3. Confirm the monitor stays connected through the silent sleep interval (~5 s).
4. Power-cycle the board (disconnect USB, reconnect) and flash/monitor again if needed.
5. Confirm the log shows first boot and the counter has reset.

Pass criterion: counter increments monotonically across sleep; resets only on power-cycle;
monitor stays connected across sleep on the UART-C port.

---

### T-M2 — LED heartbeat (Seeed XIAO ESP32-C6)

**Why manual**: Visual on/off timing and active-LOW polarity require real hardware.

Hardware required: Seeed XIAO ESP32-C6, USB-C cable.

The USB Serial/JTAG port drops when the chip enters deep sleep; `espflash monitor` will
exit. Observe the LED directly.

Steps:

1. From `examples/deep-sleep-timer-node/`: `cargo run --release --no-default-features --features board-xiao-c6`
2. Observe the blue user LED on GPIO 15.
3. Confirm a single brief blink per ~5 s wake; LED off during sleep.

Pass criterion: single brief blink per cycle, off during sleep, correct active-LOW polarity.

---

### T-M3 — WS2812 random-colour heartbeat (ESP32-C6-DevKitC-1-N8)

**Why manual**: Colour variation, GRB order, and 64/255 room-light visibility require
observing the physical LED. A GPIO-level bug looks the same as an unsoldered jumper.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8. Serial is optional; use the **left**
USB-C UART-C port if you also want T-M1 logs.

Steps:

1. From `examples/deep-sleep-timer-node/`: `cargo run --release`
2. Observe 5+ consecutive wakeups by watching the onboard RGB LED.
3. Confirm the colour is not identical every cycle (a 6-colour palette making one colour
   forever means the RNG is not applied).
4. Confirm each blink is clearly visible in room light (not 8/255-dim) and that the LED
   is dark between blinks and during the ~5 s sleep.

Pass criterion: colour varies; blinks visible; LED off during sleep.
