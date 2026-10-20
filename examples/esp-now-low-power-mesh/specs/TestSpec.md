# Testing Specification — ESP-NOW Sensor to Gateway (ESP32-C6)

## Testing Philosophy

Automated testing using native Cargo / rustc tools is strongly preferred.
Additional tools are only allowed with explicit justification and documented benefits.
Manual testing is used only when no practical automated alternative exists.

Do not specify `idf.py`, ESP-IDF Unity, or CMake-based harnesses.

---

## Automated Tests

### T-A1 — Zero-warning build (DevKitC-C6 sensor, default features)

**Tool**: `cargo` (native; rustup stable + `riscv32imac-unknown-none-elf` + `rust-src`)
**What it verifies**: Default features (`board-devkitc-c6`, `role-sensor`) compile cleanly
with crates.io `esp-radio` `=1.0.0-beta.1` (ESP-NOW) and placeholder channel / MAC env values.

```sh
cd examples/esp-now-low-power-mesh
cargo build --release
cargo clippy --release -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A2 — Zero-warning build (DevKitC-C6 gateway)

**Tool**: `cargo` (native)
**What it verifies**: The always-on receive role compiles. `--no-default-features` is
required so `role-sensor` is not also enabled.

```sh
cd examples/esp-now-low-power-mesh
cargo build --release --no-default-features --features board-devkitc-c6,role-gateway
cargo clippy --release --no-default-features --features board-devkitc-c6,role-gateway -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A3 — Zero-warning builds (XIAO ESP32-C6, both roles)

**Tool**: `cargo` (native)
**What it verifies**: The GPIO LED path compiles for each role.

```sh
cd examples/esp-now-low-power-mesh
cargo build --release --no-default-features --features board-xiao-c6,role-sensor
cargo clippy --release --no-default-features --features board-xiao-c6,role-sensor -- -D warnings
cargo build --release --no-default-features --features board-xiao-c6,role-gateway
cargo clippy --release --no-default-features --features board-xiao-c6,role-gateway -- -D warnings
```

Pass criterion: all four commands exit 0 with no warnings.

---

### T-A4 — Role features are mutually exclusive

**Tool**: `cargo` (native)
**What it verifies**: Both roles together, and a board with no role, fail at compile time.

```sh
cd examples/esp-now-low-power-mesh
cargo build --release --no-default-features --features board-devkitc-c6,role-sensor,role-gateway
cargo build --release --no-default-features --features board-devkitc-c6
```

Pass criterion: both commands fail to compile. The default build (T-A1) still succeeds.

---

## Manual Tests

*Manual tests are used only when no practical automated alternative exists.*

### T-M1 — Two-board unicast (DevKitC-C6)

**Why manual**: ESP-NOW delivery and deep-sleep retention need two radios and a real RTC domain.

Hardware: two Espressif ESP32-C6-DevKitC-1-N8 boards. Sensor on the **left** USB-C UART-C
port so the monitor survives sleep. Gateway on either USB-C port.

Steps:

1. Flash the gateway:
   `cargo run --release --no-default-features --features board-devkitc-c6,role-gateway`
   from `examples/esp-now-low-power-mesh/`. Copy the logged station MAC.
2. Rebuild the sensor with `ESPNOW_CHANNEL` matching the gateway and
   `ESPNOW_GATEWAY_MAC` set to that MAC. Flash with `cargo run --release` on the second board
   (default features) after exporting those variables the way the README describes.
3. Watch at least five sensor wakes. Confirm the sensor boot count increments by 1, the
   gateway prints that count and the sender MAC, and the sensor RGB LED flashes once
   (green, visible, cap 64/255) then stays dark during the ~15 s sleep.
4. Confirm the sensor monitor stays connected across sleep.

Pass criterion: five matching counts on the gateway, no skipped sensor counts, LED dark
during sleep, no hang.

---

### T-M2 — Unconfigured sensor does not hang (DevKitC-C6)

**Why manual**: The skip path is a timed sleep on hardware. The placeholder MAC is the
default build.

Steps:

1. Flash the default sensor image without overriding `ESPNOW_GATEWAY_MAC`.
2. Confirm the log says the gateway MAC is unset, the LED shows the failure colour, and
   the device sleeps within about 3 seconds. Repeat for two cycles.

Pass criterion: no hang, counter still increments, failure indication visible.

---

### T-M3 — XIAO sensor LED (optional)

**Why manual**: Active-LOW polarity and the USB drop on sleep are visual / host-port behaviour.

Hardware: Seeed XIAO ESP32-C6 as the sensor, any C6 gateway from T-M1 on the same channel.

Steps:

1. `cargo run --release --no-default-features --features board-xiao-c6,role-sensor`
   with a real gateway MAC.
2. Expect the USB monitor to exit on the first sleep. Watch GPIO 15: one blink per
   successful send, off during sleep.

Pass criterion: blink matches a successful send observed on the gateway. The USB drop is
not a firmware failure.
