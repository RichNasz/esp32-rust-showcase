# Testing Specification — Hardware AES Secure Storage (ESP32-C6)

## Testing Philosophy

Automated testing using native Cargo / rustc tools is strongly preferred.
Additional tools are only allowed with explicit justification and documented benefits.
Manual testing is used only when no practical automated alternative exists.

Do not specify `idf.py`, ESP-IDF Unity, or CMake-based harnesses.

---

## Automated Tests

### T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)

**Tool**: `cargo` (native; rustup stable + `riscv32imac-unknown-none-elf` + `rust-src`)
**What it verifies**: The crate compiles cleanly for the default board (DevKitC-C6, WS2812 / RMT + hardware AES + `esp-storage`) with no `rustc` or clippy warnings.

```sh
cd examples/hardware-aes-secure-storage
cargo build --release
cargo clippy --release -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)

**Tool**: `cargo` (native)
**What it verifies**: The GPIO LED + AES + flash path compiles cleanly when the XIAO board feature is selected.

```sh
cd examples/hardware-aes-secure-storage
cargo build --release --no-default-features --features board-xiao-c6
cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

### T-A3 — Flash via espflash runner (DevKitC-C6)

**Tool**: `cargo run` with the crate runner (`espflash flash --monitor --partition-table partitions.csv`)
**What it verifies**: The image flashes; serial log shows key generated or loaded, `Self-test: PASS`, and a hardware AES throughput line.

```sh
cd examples/hardware-aes-secure-storage
cargo run --release
```

Pass criterion: flash succeeds; monitor shows `Self-test: PASS` and a throughput figure.

Hardware required for the flash step. The *build* half of this crate is covered by T-A1 without a board.

Use the **left** USB-C UART-C port on DevKitC-C6.

---

## Manual Tests

*Manual tests are used only when no practical automated alternative exists. Each step explains
why automation is not applicable.*

### T-M1 — Self-test and LED on real hardware (ESP32-C6-DevKitC-1-N8)

**Why manual**: Confirms the hardware AES accelerator is invoked on C6 silicon (simulators
may take a software path) and that the WS2812 pass pattern is visible.

Hardware required: Espressif ESP32-C6-DevKitC-1-N8, USB-C on the **left** UART-C port
(UART bridge; typically `/dev/cu.usbserial-*` on macOS).

Steps:

1. From `examples/hardware-aes-secure-storage/`: `cargo run --release`
2. Confirm serial shows the key was generated and saved (first flash) or loaded (later reset).
3. Confirm `Self-test: PASS` and a log line that ciphertext differs from plaintext.
4. Confirm a hardware AES throughput line of **≥ 2 MB/s**.
5. Observe the onboard RGB LED: three slow green blinks, then dark.

Pass criterion: PASS logged, throughput ≥ 2 MB/s, green 3-blink visible.

---

### T-M2 — Key persistence across resets

**Why manual**: Flash persistence requires physical write/read cycles.

Hardware required: same as T-M1 (DevKitC UART-C port so the monitor stays connected).

Steps:

1. First boot after a full flash: confirm the log says the key was generated and saved.
2. Press RST (do not power-cycle if you want the same flash contents; RST is enough).
3. Confirm the log says the key was loaded from storage (not generated).
4. Confirm `Self-test: PASS` still appears (decrypt works with the persisted key).

Pass criterion: key loads after reset; self-test still passes.

---

### T-M3 — GPIO LED pass pattern (Seeed XIAO ESP32-C6)

**Why manual**: Active-LOW polarity and blink timing require human observation.

Hardware required: Seeed XIAO ESP32-C6, USB-C cable.

Steps:

1. From `examples/hardware-aes-secure-storage/`: `cargo run --release --no-default-features --features board-xiao-c6`
2. Confirm serial shows the GPIO LED path (not WS2812 / RMT) on GPIO 15, `Self-test: PASS`.
3. Observe the blue user LED: three slow blinks, then off. Polarity is active-LOW.

Pass criterion: correct polarity, three blinks, PASS in the log.
