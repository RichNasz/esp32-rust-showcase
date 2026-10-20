# Testing Specification — LP-Core GPIO Wakeup (ESP32-C6)

## Testing Philosophy

Automated testing using native Cargo / rustc tools is strongly preferred.
Additional tools are only allowed with explicit justification and documented benefits.
Manual testing is used only when no practical automated alternative exists.

Do not specify `idf.py`, ESP-IDF Unity, or CMake-based harnesses.

---

## Automated Tests

### T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)

**Tool**: `cargo` (native; rustup stable + `riscv32imac-unknown-none-elf` + `rust-src`)
**What it verifies**: The HP crate and the nested `lp-core/` image compile cleanly for
DevKitC-C6. `build.rs` must produce the LP image without CMake. No `rustc` or clippy
warnings on the HP package.

```sh
cd examples/lp-core-wakeup
cargo build --release
cargo clippy --release -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings. The build log shows the LP crate
being compiled. No `CMakeLists.txt` is created.

---

### T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)

**Tool**: `cargo` (native)
**What it verifies**: The active-LOW GPIO LED path and the same LP image compile when the
XIAO board feature is selected. The wake pad remains GPIO 0, not GPIO 9.

```sh
cd examples/lp-core-wakeup
cargo build --release --no-default-features --features board-xiao-c6
cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

## Manual Tests

*Manual tests are used only when no practical automated alternative exists.*

### T-M1 — GPIO wake and timer wake (ESP32-C6-DevKitC-1-N8)

**Why manual**: LP-core wake, persistent RAM, and deep sleep need the real RTC domain and
the onboard button.

Hardware: Espressif ESP32-C6-DevKitC-1-N8, **left** USB-C UART-C port.

Steps:

1. From `examples/lp-core-wakeup/`: `cargo run --release`.
2. Confirm a first-boot log and then sleep. Do not hold BOOT while resetting.
3. Wait for a timer wake (~20 s). Confirm one slow amber blink, a timer log line, and
   that the GPIO counter does not increment.
4. After the device sleeps again, press BOOT (GPIO 0) briefly. Confirm a wake well under
   a second, three green blinks, and a counter increment of 1.
5. Repeat the press twice more. Confirm the counter is monotonic.
6. Power-cycle the board. Confirm the next boot is a first boot and the counter reset.
7. Confirm the monitor stayed connected through the sleep intervals.

Pass criterion: timer wakes do not increment the counter; GPIO wakes increment it by 1;
LED patterns differ; monitor stays up on UART-C; blinks are visible in room light
(cap 64/255).

---

### T-M2 — GPIO 9 does not wake XIAO; timer and D0 do

**Why manual**: The XIAO BOOT key is the wrong pad. Only a person looking at the board
can confirm which button was pressed. USB Serial/JTAG drops on sleep, so the LED is the
instrument.

Hardware: Seeed XIAO ESP32-C6. Optional momentary button from D0 (GPIO 0) to GND.

Steps:

1. `cargo run --release --no-default-features --features board-xiao-c6`.
2. Expect the USB monitor to exit on the first sleep. That is not a failure.
3. Press the onboard BOOT button (GPIO 9) several times during sleep. Confirm the user
   LED does not run the three-blink pattern.
4. Confirm a single slow blink about every 20 seconds (timer path, active LOW).
5. If a button is wired to D0, press it during sleep. Confirm three blinks.

Pass criterion: GPIO 9 does nothing visible. Timer blinks continue. D0, when wired,
produces the three-blink pattern.
