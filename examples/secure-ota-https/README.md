<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-full-project-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# HTTPS OTA Slot Flip — ESP32-C6

> TLS 1.3 download into the inactive OTA slot, IEEE CRC32 check, then reboot.

## Implementation status

**Firmware generated** as of 2026-10-20. `Cargo.toml` and `src/` are in this folder. `cargo build --release` and `cargo clippy --release -- -D warnings` completed with no warnings for the default DevKitC-C6 features and for `board-xiao-c6`. That is a host compile result. It is not a hardware test.

`specs/showcase-master-spec.md` still lists this example as **Specs only**. Humans own that catalog. The FunctionalSpec in this folder repeats that older note.

`esp-hal` feature **`unstable`** is on because Wi-Fi and RMT need it on the `esp-hal-v1.2.1` pin. Those APIs can change. **`esp-radio` is an unstable beta.**

## Overview

One firmware image, two app slots. On boot the log and the onboard LED show which slot is running. A qualified BOOT press joins a WPA2-Personal network, performs one HTTP/1.1 GET over TLS 1.3, and streams the body into the inactive slot. The body is an `espflash save-image` app image with an 8-byte little-endian prefix: image length, then the IEEE CRC32 of those image bytes. The first image byte must be the ESP app magic `0xE9`. The inactive slot is activated only after the length, the magic, and the CRC all match. A failure returns to the button wait and does not reboot.

The trust anchor is one embedded CA. `embedded-tls` 0.19.0 feature `rustpki` checks a server certificate that this CA signed directly (no intermediate). Verification is not optional. The crate speaks TLS 1.3 only. CRC32 is a transfer check. It is not a signature and not Secure Boot.

Pin numbers live in generated `src/board.rs`.

## Prerequisites

- rustup **stable** (MSRV 1.95.0; see `shared-specs/rust-toolchain.md`)
- rustup component `rust-src`
- rustup target `riscv32imac-unknown-none-elf`
- `espflash` and/or `cargo-espflash`
- Hardware: **ESP32-C6-DevKitC-1-N8** (required default) or XIAO ESP32-C6
- For a real slot flip: a 2.4 GHz WPA2-Personal access point, and a TLS 1.3 server whose certificate is signed directly by the CA you embed

ESP-IDF is not required. Do not install `espup` or the `esp` rustup channel. A RISC-V C compiler is not required. The certificate check uses RustCrypto (`rustpki`), so the build does not need `ring` or `riscv32-unknown-elf-gcc`.

## Build & Flash

From this directory:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo clippy --release -- -D warnings
cargo run --release
```

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

The runner is `espflash flash --monitor --partition-table partitions.csv`. Flash size is detected from the chip. DevKitC-1-N8 is 8 MB. XIAO ESP32-C6 is 4 MB. The `dual-ota` table ends at `0x360000` and fits both. When auto-detect is wrong, pass `--flash-size 8mb` on the DevKitC and `--flash-size 4mb` on the XIAO. There is no factory app partition. The first image must boot from `ota_0` at offset `0x20000`.

DevKitC logs and flash go through the **left** USB-C UART-C port. XIAO uses USB Serial/JTAG. This example does not deep-sleep, so that port stays up.

A clean tree builds with placeholders: SSID `unset`, URL `https://192.0.2.1/firmware.bin`, and `certs/placeholder-ca.pem`. An empty or `unset` SSID skips association and stays in the wait. The shell environment overrides `[env]` in `.cargo/config.toml`. Do not commit a real password.

```sh
WIFI_SSID=lab \
WIFI_PASSWORD=secret \
OTA_URL=https://192.0.2.1/firmware.bin \
cargo run --release
```

`build.rs` reads `OTA_CA_PATH` (default `certs/placeholder-ca.pem`) and `OTA_NOW_UNIX` (default `1792504800`, which is 2026-10-20 14:00:00 UTC). The PEM must be a certificate. A file that contains `PRIVATE KEY` fails the build. The placeholder CA is a self-signed EC P-256 certificate, CN `esp32-rust-showcase-placeholder-ca`, valid from 2026-10-20 13:14:40 UTC through 2036-10-17 13:14:40 UTC. It does not authenticate a public server. Point `OTA_CA_PATH` at your lab CA and set `OTA_NOW_UNIX` inside that certificate's validity window. The server certificate must be signed by that CA itself. An intermediate in the chain is rejected.

Build the payload from a release ELF of this same example. The saved file starts with `0xE9`:

```sh
espflash save-image --chip esp32c6 \
  target/riscv32imac-unknown-none-elf/release/secure_ota_https \
  app.bin
python3 - <<'PY'
import struct, zlib
from pathlib import Path
img = Path("app.bin").read_bytes()
crc = zlib.crc32(img) & 0xFFFFFFFF
Path("firmware.bin").write_bytes(struct.pack("<II", len(img), crc) + img)
print(f"length={len(img)} crc=0x{crc:08x}")
PY
```

Serve `firmware.bin` at `OTA_URL` over TLS 1.3 with no redirect. The CRC covers the app image only. `zlib.crc32(b"123456789")` is `0xCBF43926`.

> **Never edit generated source files directly.** To change behaviour, update `specs/FunctionalSpec.md` or `specs/CodingSpec.md` and regenerate.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/secure-ota-https/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected Serial Output

These are the lines the firmware logs. They are not a transcript from a board.

### Placeholder build (SSID unset), DevKitC-C6

```
secure_ota_https: HTTPS OTA — ESP32-C6-DevKitC-1-N8, LED GPIO 8 (Ws2812), trigger GPIO 0
secure_ota_https: Booted ota_0 at offset 0x20000
secure_ota_https: Recorded image state Valid
secure_ota_https: initialized otadata; boot selection stays ota_0
secure_ota_https: WS2812 RMT init on GPIO 8 (GRB order, 800 kHz NRZ, max brightness 64/255)
secure_ota_https: Waiting for BOOT press
secure_ota_https: Wi-Fi SSID not configured (compile-time env); staying in wait
```

`Recorded image state Valid` appears when the booted image state is New or PendingVerify. The `initialized otadata` line appears when `otadata` is empty and the device is running `ota_0`. The password is never logged.

The WS2812 shows the slot: blue for `ota_0` (R=0, G=0, B=64), amber for `ota_1` (R=64, G=36, B=0). An unknown slot is three red blinks of 200 ms. Brightness stays at or under 64/255. During erase the LED pulses about every 64 KiB (40 ms off, then the slot colour).

### After a successful press

```
secure_ota_https: Starting Wi-Fi STA
secure_ota_https: Wi-Fi associated ssid=lab
secure_ota_https: Got IP 192.0.2.50/24
secure_ota_https: HTTPS GET 192.0.2.1:443/firmware.bin
secure_ota_https: envelope length=846384 crc=0x........
secure_ota_https: Erasing inactive ota_1
secure_ota_https: download 65536/846384
secure_ota_https: CRC ok; activated ota_1 at offset 0x1a0000
secure_ota_https: Rebooting
```

The next boot logs `ota_1` and the LED is amber. Another successful press returns to `ota_0`.

A rejected certificate logs `TLS certificate rejected`. A bad length, magic, or CRC logs `image CRC, length, or magic rejected`. The device stays on the current slot and shows the error pattern (three red blinks).

### Seeed XIAO ESP32-C6

```
secure_ota_https: HTTPS OTA — XIAO ESP32-C6, LED GPIO 15 (GpioActiveLow), trigger GPIO 9
secure_ota_https: GPIO LED on GPIO 15 (active low)
```

GPIO 15 is active LOW. One short blink means `ota_0` (120 ms on, 180 ms off). Two blinks mean `ota_1`. Three blinks mean an unknown slot. An error is three blinks of 400 ms. The LED is off while idle after the indication. BOOT is GPIO 9.

## Key Concepts

- Radio crates are git-pinned together at tag `esp-hal-v1.2.1`: `esp-hal`, `esp-radio`, `esp-rtos`, `esp-alloc`, `esp-bootloader-esp-idf`, and `esp-storage`. `esp-radio` is an unstable beta. Wi-Fi and RMT use `esp-hal` feature `unstable`.
- Embassy entry is `#[esp_rtos::main]`. One `embassy-net` 0.9 stack (DHCPv4) stays up across BOOT presses. Association is capped at 15 seconds and DHCP at 10 seconds. DNS runs only when the URL host is not an IPv4 literal.
- `embedded-tls` 0.19.0 with `std` off and feature `rustpki`. TLS 1.3, one trust anchor, hostname check, lab clock. Record buffers live in a static `Scratch`, not on the main-task future.
- OTA uses `esp-bootloader-esp-idf`: write the inactive app slot, then `activate_next_partition`. Every image calls `esp_app_desc!()`. Current `espflash` refuses an image without that descriptor.
- Profile `dual-ota` in `partitions.csv` / `espflash.toml`. Slots are 1536 KiB each. No factory partition.
- The main stack is the leftover RWDATA. `ESP_HAL_CONFIG_ENSURE_MAIN_STACK_MINIMUM` is 32768 so the link fails if that leftover is smaller. The 80 KiB heap is for the Wi-Fi blob and `embassy-net`.
- DevKitC status is an RMT WS2812 (GRB, 800 kHz NRZ, 80 MHz clock, divider 8). XIAO status is a GPIO blink count.

## Limitations

| This example is not | Why |
|---|---|
| Signed production updates | CRC32 is a transfer check. Secure Boot v2 burns eFuses and is out of scope |
| Automatic rollback | The bootloader shipped with current `espflash` has no OTA auto-rollback. A bad image that was activated stays selected until something writes `otadata` again |
| A browser CA store | One embedded CA. `rustpki` accepts only a leaf that this CA signed directly |
| TLS 1.2 | `embedded-tls` 0.19.0 is TLS 1.3 only |
| An ESP-IDF OTA app | No `esp_https_ota`, no Kconfig, no `sdkconfig` |
| A two-program LED rig | One firmware, two slots |

Writing flash disables the cache while Wi-Fi interrupts can still run. That is an accepted residual risk of streaming the image with the radio up. The image is not buffered in RAM.

The placeholder CA and the lab clock are for a bench server. A public URL signed by another root fails closed.

## Testing

Testing philosophy: automated Cargo builds first. Manual tests are only for behaviour that needs a board and a network. Hardware tests below have not been run in this generation.

### Automated Tests

1. **T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)**

   From `examples/secure-ota-https/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings. No live access point and no real password in `src/`. Verified on this tree.

2. **T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass: both commands exit 0 with no warnings. Verified on this tree.

### Manual Tests — hardware required

**T-M1 — Slot flip on a real board (ESP32-C6-DevKitC-1-N8)**

Why manual: the boot slot, Wi-Fi association, TLS handshake, flash write, and WS2812 colour are hardware and network behaviour.

Hardware and lab setup: DevKitC-1-N8 on the **left** USB-C UART-C port, a 2.4 GHz WPA2-Personal AP, a TLS 1.3 server whose certificate is signed by the embedded CA, and an enveloped image served at the configured URL with no redirect.

1. Override SSID, password, URL, and CA with the environment variables above. Do not commit those values.
2. From `examples/secure-ota-https/`: `cargo run --release`.
3. Confirm the log names `ota_0` and the RGB LED is blue (cap 64/255).
4. Press BOOT (GPIO 0) only after the log says it is waiting. Confirm a blink during transfer.
5. After reboot, confirm the log names `ota_1` and the LED is amber.
6. Press BOOT again and confirm the device returns to `ota_0` (blue).

Pass: each successful press changes the logged slot and the LED colour. The monitor shows CRC success before reboot. The password does not appear in the log.

**T-M2 — Reject a bad certificate and a bad CRC (DevKitC-C6)**

Why manual: negative TLS and CRC behaviour depends on a live server and on flash leaving the boot selection unchanged.

1. Point the URL at a TLS 1.3 server whose certificate does not chain to the embedded CA. Press BOOT. Confirm a certificate or handshake failure, the error LED pattern, and no reboot. Reset by hand and confirm the slot log is unchanged.
2. Restore a trusted server, but serve a body whose CRC prefix does not match the bytes. Press BOOT. Confirm a CRC or length failure and an unchanged boot slot.

Pass: neither case activates a new slot.

**T-M3 — XIAO blink count (Seeed XIAO ESP32-C6)**

Why manual: the active-LOW blink count is visual. USB Serial/JTAG stays up because this example does not deep-sleep.

1. From `examples/secure-ota-https/`: `cargo run --release --no-default-features --features board-xiao-c6`.
2. Confirm one short blink when the log says `ota_0`, and two short blinks after a successful flip to `ota_1`.

Pass: blink count matches the slot. GPIO 15 is active LOW and the LED is off while idle after the indication.

## Spec Files

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md)
- [specs/CodingSpec.md](specs/CodingSpec.md)
- [specs/TestSpec.md](specs/TestSpec.md)
