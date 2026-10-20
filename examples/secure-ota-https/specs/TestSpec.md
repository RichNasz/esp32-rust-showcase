# Testing Specification — HTTPS OTA Slot Flip (ESP32-C6)

## Testing Philosophy

Automated testing using native Cargo / rustc tools is strongly preferred.
Additional tools are only allowed with explicit justification and documented benefits.
Manual testing is used only when no practical automated alternative exists.

Do not specify `idf.py`, ESP-IDF Unity, or CMake-based harnesses.

---

## Automated Tests

### T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)

**Tool**: `cargo` (native; rustup stable + `riscv32imac-unknown-none-elf` + `rust-src`)
**What it verifies**: The crate compiles cleanly for DevKitC-C6 with the git-pinned radio
stack, `embedded-tls` without `std`, and placeholder Wi-Fi / URL / CA values. No `rustc`
or clippy warnings.

```sh
cd examples/secure-ota-https
cargo build --release
cargo clippy --release -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings. The build must not require a live
access point or a real password in `src/`.

---

### T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)

**Tool**: `cargo` (native)
**What it verifies**: The GPIO-LED path and the 4 MB board feature compile cleanly.

```sh
cd examples/secure-ota-https
cargo build --release --no-default-features --features board-xiao-c6
cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
```

Pass criterion: both commands exit 0 with no warnings.

---

## Manual Tests

*Manual tests are used only when no practical automated alternative exists. Each step explains
why automation is not applicable.*

### T-M1 — Slot flip on a real board (ESP32-C6-DevKitC-1-N8)

**Why manual**: The boot slot, Wi-Fi association, TLS handshake, flash write, and WS2812
colour are hardware and network behaviour. A host build cannot certify them.

Hardware and lab setup:

- Espressif ESP32-C6-DevKitC-1-N8, **left** USB-C UART-C port.
- A 2.4 GHz WPA2-Personal AP.
- A TLS 1.3 HTTPS server whose certificate chains to the CA embedded in the firmware.
- An enveloped image: `espflash save-image` of this example for `esp32c6`, then an 8-byte
  little-endian prefix (image length, IEEE CRC32 of the image). Serve that body at the
  configured URL with no redirect.

Steps:

1. Override the placeholder SSID, password, URL, and CA via the documented environment
   variables. Do not commit those values.
2. From `examples/secure-ota-https/`: `cargo run --release`.
3. Confirm the log names `ota_0` and the RGB LED is blue (cap 64/255, visible in room light).
4. Press BOOT (GPIO 0) only after the log says it is waiting. Confirm a blink during transfer.
5. After reboot, confirm the log names `ota_1` and the LED is amber.
6. Press BOOT again and confirm the device returns to `ota_0` (blue).

Pass criterion: each successful press changes the logged slot and the LED colour. The
monitor shows CRC success before reboot. The password does not appear in the log.

---

### T-M2 — Reject a bad certificate and a bad CRC (DevKitC-C6)

**Why manual**: Negative TLS and CRC behaviour depends on a live server and on flash not
changing boot selection.

Steps:

1. Point the URL at a TLS 1.3 server whose certificate does **not** chain to the embedded CA.
   Press BOOT. Confirm the log reports a certificate or handshake failure, the LED shows
   the error pattern, and a reboot does not happen. The slot log is unchanged if the device
   is reset by hand afterwards.
2. Restore a trusted server, but serve a body whose CRC prefix does not match the bytes.
   Press BOOT. Confirm the log reports a CRC or length failure and the boot slot does not change.

Pass criterion: neither case activates a new slot.

---

### T-M3 — XIAO blink count (Seeed XIAO ESP32-C6)

**Why manual**: Active-LOW blink count is visual. USB Serial/JTAG is usable here because
this example does not deep-sleep.

Steps:

1. From `examples/secure-ota-https/`: `cargo run --release --no-default-features --features board-xiao-c6`
2. Confirm one short blink when the log says `ota_0`, and two short blinks after a successful flip to `ota_1`.

Pass criterion: blink count matches the slot. GPIO 15 active LOW (LED off while idle after the indication).
