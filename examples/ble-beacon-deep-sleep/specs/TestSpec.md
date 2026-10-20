# Testing Specification — BLE Beacon + Deep Sleep (ESP32-C6)

## Testing Philosophy

Automated testing using native Cargo / rustc tools is strongly preferred.
Additional tools are only allowed with explicit justification and documented benefits.
Manual testing is used only when no practical automated alternative exists.

Do not specify `idf.py`, ESP-IDF Unity, or CMake-based harnesses.

---

## Automated Tests

### T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)

**Tool**: `cargo` (native; rustup stable + `riscv32imac-unknown-none-elf` + `rust-src`)
**What it verifies**: The crate compiles cleanly for the default board (DevKitC-C6, WS2812 /
RMT + RTC sleep + crates.io `esp-radio` `=1.0.0-beta.1` BLE + TrouBLE) with no `rustc` or clippy warnings.
Confirms the crates.io radio pin resolves.

```sh
cd examples/ble-beacon-deep-sleep
cargo build --release
cargo clippy --release -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)

**Tool**: `cargo` (native)
**What it verifies**: The GPIO LED + BLE + RTC sleep path compiles when the XIAO board
feature is selected.

```sh
cd examples/ble-beacon-deep-sleep
cargo build --release --no-default-features --features board-xiao-c6
cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A3 — Flash via espflash runner (DevKitC-C6)

**Tool**: `cargo run` with the crate runner (`espflash flash --monitor`)
**What it verifies**: The image flashes; serial log shows first boot or a wake, BLE MAC,
adv start/stop (or a recoverable skip), teardown, and sleep entry.

```sh
cd examples/ble-beacon-deep-sleep
cargo run --release
```

Pass criterion: flash succeeds; monitor shows boot/wake log, `BLE MAC:`, advertising
activity or a logged failure, and sleep entry.

Hardware required for the flash step. The *build* half of this crate is covered by T-A1
without a board.

Use the **left** USB-C UART-C port so the monitor stays connected through deep sleep.
Native USB Serial/JTAG will drop the port on the first sleep; that is not a firmware failure.

---

## Manual Tests

*Manual tests are used only when no practical automated alternative exists. Each step explains
why automation is not applicable.*

### T-M1 — BLE advertisement visible to scanner

**Why manual**: Verifying OTA BLE advertisement content requires a second device acting as
a scanner — not reproducible in a host build.

Hardware required: ESP32-C6-DevKitC-1-N8 (or XIAO), smartphone with nRF Connect or equivalent.

Steps:

1. From `examples/ble-beacon-deep-sleep/`: `cargo run --release` (add
   `--no-default-features --features board-xiao-c6` for XIAO).
2. Open nRF Connect → Scanner → start scanning.
3. On each device wake (~20 s period: 10 s advertise + 10 s sleep), confirm a burst is
   visible and the device appears as **`ESP32-SDD`**.
4. Inspect manufacturer data: company ID `0xFFFF`, magic `DE AD BE EF`, little-endian
   `boot_count` incrementing by 1 each wake.
5. Confirm serial `BLE MAC:` matches the address shown in the scanner.

Pass criterion: advertisement every cycle; name and manufacturer data match the spec.

---

### T-M2 — Active window / LED timing

**Why manual**: Advertising-window duration and LED polarity/colour are easiest to judge
from the physical LED and serial timestamps.

Hardware required: DevKitC-C6 (WS2812) or XIAO (GPIO 15).

Steps:

1. Observe the LED: **off** during init, **on** only while advertising (~10 s), **off**
   during teardown and sleep. DevKitC is solid blue at a visible 64/255; XIAO is active-LOW.
2. Serial timestamps: adv-start then adv-stop within the 10 s window; whole wake ≤ 12 s
   before sleep.

Pass criterion: LED dark except during advertising; active window ≤ 12 s.

---

### T-M3 — Button wakeup (DevKitC GPIO 0 / XIAO external GPIO 0)

**Why manual**: Physical RTC GPIO wakeup requires silicon.

Hardware required: DevKitC-C6 (BOOT = GPIO 0) or XIAO with an external active-LOW button on
D0 (GPIO 0). Do **not** use the XIAO BOOT key (GPIO 9).

Steps:

1. During deep sleep, press the RTC wake button.
2. Confirm advertisement and LED within 500 ms of the press being observed.
3. Confirm serial log shows a GPIO wakeup cause (not timer).

Pass criterion: GPIO 0 wakes the device. GPIO 9 must not be required.

---

### T-M4 — Radio teardown / sleep current (optional)

**Why manual**: Distinguishing milliamps (radio left on) from microamps (torn down) needs
a current meter or at least a thermal/USB-draw observation.

Steps:

1. After several cycles, confirm the device keeps cycling (not hung in advertising).
2. If a current meter is available, confirm sleep floor is tens of microamps, not
   milliamps.

Pass criterion: sleep current is microamps-class when measured; otherwise, cycles continue
without the LED stuck on.
