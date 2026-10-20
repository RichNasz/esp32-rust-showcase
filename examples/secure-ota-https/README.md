<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# HTTPS OTA Slot Flip — ESP32-C6

> HTTPS download into the inactive OTA slot, CRC check, reboot. Not Secure Boot or flash-encryption enrollment.

## Implementation status

**Firmware generated.** Crate generated 2026-10-20 (HTTPS slot flip, CRC, no Secure Boot, no stock-`espflash` auto-rollback). A host build is not a hardware pass.

A spec is Markdown a person writes; the agent generates the crate from it. See [Using With AI Agents](../../docs/guides/using-with-ai-agents.md).

`esp-hal` feature **`unstable`** is on because Wi-Fi and RMT need it on the `esp-hal-v1.2.1` pin. Those APIs can change. **`esp-radio` is an unstable beta.**

## Overview

One firmware image, two app slots. On boot the log and the onboard LED show which slot is running (`ota_0` or `ota_1`, including the flash offset). The device does **not** download at boot. It waits for a qualified button press.

A press is a new active-LOW edge after the line has been seen released. A level that is already low when the wait starts is not a press. On DevKitC, GPIO 0 held low across reset is the serial-download strap, not an OTA request. The trigger is DevKitC BOOT (GPIO 0) or XIAO BOOT (GPIO 9). GPIO 9 is not an RTC pin. This example never sleeps, so that is fine.

On a qualified press the firmware joins a WPA2-Personal network (about **15 s** cap) and performs one HTTP/1.1 GET over **TLS 1.3**. DHCP is capped at about **10 s**. DNS runs only when the URL host is not an IPv4 literal. The body streams into the **inactive** slot. It is not buffered in RAM. The body is an `espflash save-image` app image with an 8-byte little-endian prefix: image length, then the IEEE CRC32 of those image bytes. The first image byte must be the ESP app magic `0xE9`. The inactive slot is activated only after the length, the magic, and the CRC all match. Then the firmware reboots. The next boot logs the other slot. A second success flips back.

A failure returns to the button wait and does **not** reboot and does **not** activate a slot. That includes association timeout, a certificate that does not chain to the embedded CA, a hostname mismatch, a handshake failure, a short body, a bad length, a bad magic, or a bad CRC. Certificate failure names the reason class (untrusted, handshake, timeout) and does not print key material. The password is never logged.

The trust anchor is one embedded CA. `embedded-tls` **0.19.0** with `std` off and feature `rustpki` checks a server certificate that this CA signed **directly** (no intermediate). Verification is not optional. The crate speaks TLS 1.3 only. It will not handshake with a TLS 1.2-only server. TLS 1.3 satisfies the TLS 1.2 minimum in `shared-specs/security-baseline.md`. CRC32 is a transfer check. It is not a signature and not Secure Boot. This demo does not enroll Secure Boot, burn eFuses, or turn on flash encryption. The bootloader shipped with current `espflash` has **no** OTA auto-rollback.

Pin numbers live in generated `src/board.rs`. There is no external LED and no external button.

DevKitC WS2812 (GPIO 8, RMT, GRB, 800 kHz NRZ, cap 64/255): blue for `ota_0` (R=0, G=0, B=64), amber for `ota_1` (R=64, G=36, B=0). An unknown slot is three red blinks of 200 ms. During erase the LED pulses about every 64 KiB (40 ms off, then the slot colour). Failure is three red blinks, then the slot colour. A GPIO level on GPIO 8 does not light the LED.

XIAO GPIO 15 is active LOW. One short blink means `ota_0` (120 ms on, 180 ms off). Two blinks mean `ota_1`. Three blinks mean an unknown slot. An error is three blinks of 400 ms. The LED is off while idle after the indication. Rapid blinks mean a transfer is in progress. Do not run the RMT encoder on this board.

## Prerequisites

- **Board:** Espressif **ESP32-C6-DevKitC-1-N8** (default, 8 MB flash) or Seeed **XIAO ESP32-C6** (4 MB flash). No extra LEDs, buttons, or sensors.
- **USB:** DevKitC **left** USB-C (UART-C bridge, `espflash` on that port). XIAO USB-C is native USB Serial/JTAG. This example does not deep-sleep, so the console stays up on either port.
- **Toolchain:** rustup **stable** (MSRV 1.95.0), `rust-src`, target `riscv32imac-unknown-none-elf`, and `espflash` / `cargo-espflash`. Install steps: [01 — Toolchain Setup](../../docs/guides/01-toolchain-setup.md). A RISC-V C compiler is not required. Certificate checks use RustCrypto (`rustpki`), so the build does not need `ring` or `riscv32-unknown-elf-gcc`.
- **For a real slot flip (not for the host build):** a 2.4 GHz WPA2-Personal access point, and a TLS 1.3 server whose certificate is signed directly by the CA you embed. The URL must be the final URL. Redirects are out of scope.

ESP-IDF is not a prerequisite. Do not install `espup` or the `esp` rustup channel. There is no `esp_https_ota`, no Kconfig, and no `sdkconfig`.

## Supported Boards

Chip ≠ board. v1 chip feature `esp32c6` is the SoC. Boards select pins and USB. Not `idf.py set-target`.

Auto-discovered from `board-specs/` for ESP32-C6 with Wi-Fi and a user LED. Both catalog boards pass. Default board: Espressif ESP32-C6-DevKitC-1-N8.

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

The trigger is a press after boot, not a sleep wake. DevKitC BOOT is GPIO 0. XIAO BOOT is GPIO 9.

## Per-Board Behavior

| Board | LED feedback |
| --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | WS2812: blue = `ota_0`, amber = `ota_1`, red blinks = error (cap 64/255) |
| Seeed XIAO ESP32-C6 | On / off, active LOW: one blink = `ota_0`, two blinks = `ota_1` |

The first difference you will see is the LED language, and which USB socket to open. DevKitC paints a colour on the WS2812. A GPIO write to GPIO 8 does not light it. XIAO counts active-LOW blinks on GPIO 15 and then turns the LED off. This example does not deep-sleep, so neither console drops. Flash and log the DevKitC on the **left** USB-C UART-C port. XIAO uses USB Serial/JTAG. DevKitC flash is 8 MB. XIAO flash is 4 MB. The same `dual-ota` table fits both.

## Build & Flash

From this directory:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

The zero-warning check in Testing also runs release-profile clippy:

```sh
cargo clippy --release -- -D warnings
```

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

The runner is `espflash flash --monitor --partition-table partitions.csv`. Flash size is detected from the chip. DevKitC-1-N8 is 8 MB. XIAO ESP32-C6 is 4 MB. When auto-detect is wrong, pass `--flash-size 8mb` on the DevKitC and `--flash-size 4mb` on the XIAO. There is no factory app partition. The first image must boot from `ota_0` at offset `0x20000`. `ota_1` is at `0x1A0000`. Each slot is 1536 KiB. The table, including the `storage` partition, ends at `0x360000`. Profile name: `dual-ota` (`shared-specs/partition-table-profiles.md`, `partitions.csv`, `espflash.toml`).

| Name | Type | SubType | Offset | Size |
| --- | --- | --- | --- | --- |
| nvs | data | nvs | 0x9000 | 0x6000 |
| otadata | data | ota | 0xF000 | 0x2000 |
| phy_init | data | phy | 0x11000 | 0x1000 |
| ota_0 | app | ota_0 | 0x20000 | 1536K |
| ota_1 | app | ota_1 | 0x1A0000 | 1536K |
| storage | data | nvs | 0x320000 | 0x40000 |

A clean tree builds with placeholders: SSID `unset`, URL `https://192.0.2.1/firmware.bin`, and `certs/placeholder-ca.pem`. An empty or `unset` SSID skips association and stays in the wait. The shell environment overrides `[env]` in `.cargo/config.toml`. Do not commit a real password. Never put secrets in `src/`.

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

Serve `firmware.bin` at `OTA_URL` over TLS 1.3 with no redirect. The CRC covers the app image only. `zlib.crc32(b"123456789")` is `0xCBF43926`. Do not activate a slot and check the CRC afterwards. A truncated image that was activated looks like a dead board, and stock `espflash` will not roll it back.

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/secure-ota-https/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected behavior

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

`Recorded image state Valid` appears when the booted image state is New or PendingVerify. The `initialized otadata` line appears when `otadata` is empty and the device is running `ota_0`. The password is never logged. The WS2812 stays blue for `ota_0`.

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

The length and address in that block are the shape of a successful log, not a measured download from this documentation pass. The next boot logs `ota_1` and the LED is amber. Another successful press returns to `ota_0` (blue). The LED blinks during the transfer.

A rejected certificate logs `TLS certificate rejected`. A bad length, magic, or CRC logs `image CRC, length, or magic rejected`. The device stays on the current slot and shows the error pattern (three red blinks on DevKitC). No reboot.

### Seeed XIAO ESP32-C6

```
secure_ota_https: HTTPS OTA — XIAO ESP32-C6, LED GPIO 15 (GpioActiveLow), trigger GPIO 9
secure_ota_https: GPIO LED on GPIO 15 (active low)
```

One short blink when the log says `ota_0`. Two short blinks after a flip to `ota_1`. The LED is off while idle after the indication. BOOT is GPIO 9.

## Key Concepts

- Radio crates are git-pinned together at tag `esp-hal-v1.2.1`: `esp-hal`, `esp-radio` (features `esp32c6`, `unstable`, `wifi`), `esp-rtos` (features `esp32c6`, `embassy`, `esp-radio`), `esp-alloc`, `esp-bootloader-esp-idf`, and `esp-storage`. `esp-radio` is an unstable beta. Wi-Fi and RMT use `esp-hal` feature `unstable`. Do not mix crates.io copies of those crates with git `esp-hal`.
- Embassy entry is `#[esp_rtos::main]`. One `embassy-net` 0.9 stack (DHCPv4) stays up across BOOT presses. Association is capped at 15 seconds and DHCP at 10 seconds. DNS runs only when the URL host is not an IPv4 literal. A failed stage returns to the button wait with the radio down.
- `embedded-tls` 0.19.0 with `std` off and feature `rustpki`. TLS 1.3 only, one trust anchor, hostname check, lab clock (`OTA_NOW_UNIX`). Record buffers live in a static `Scratch`, not on the main-task future. A leaf must be signed by the embedded CA itself.
- OTA uses `esp-bootloader-esp-idf`: read the table, write the inactive app slot, then `activate_next_partition` only after length, magic `0xE9`, and IEEE CRC32 match. Every image calls `esp_app_desc!()`. Current `espflash` refuses an image without that descriptor. Stock `espflash` does not auto-rollback.
- Profile `dual-ota`. Slots are 1536 KiB each. No factory partition. First boot is `ota_0` at `0x20000`.
- The main stack is the leftover RWDATA. `ESP_HAL_CONFIG_ENSURE_MAIN_STACK_MINIMUM` is 32768 so the link fails if that leftover is smaller. The 80 KiB heap is for the Wi-Fi blob and `embassy-net`.
- DevKitC status is an RMT WS2812 (GRB, 800 kHz NRZ, 80 MHz clock, divider 8, max brightness 64/255). XIAO status is a GPIO blink count, active LOW.
- Qualified press: release, then a new active-LOW edge. GPIO 0 already low at the start of the wait is ignored.

## Limitations

| This example is NOT | Why |
| --- | --- |
| Signed production updates | CRC32 is not a signature. Secure Boot v2 is an irreversible eFuse step and is out of scope |
| Automatic rollback | The bootloader shipped with current `espflash` is not built with OTA auto-rollback. A bad image that was activated stays selected until something writes `otadata` again |
| A full WebPKI client | One embedded trust anchor via `embedded-tls`. Expiry and revocation beyond what that crate checks are not a lab requirement |
| TLS 1.2 | `embedded-tls` 0.19.0 is TLS 1.3 only |
| An ESP-IDF OTA app | No `esp_https_ota`, no Kconfig, no `sdkconfig`, no FreeRTOS tasks |
| A two-program LED rig | One firmware, two slots. The slot is shown by the onboard LED and the log |

It is also not a browser CA store. `rustpki` accepts only a leaf that this CA signed directly (no intermediate). A public URL signed by another root fails closed. That is the negative test, not a bug.

Writing flash disables the cache while Wi-Fi interrupts can still run. That is an accepted residual risk of streaming the image with the radio up. The image is not buffered in RAM.

The placeholder CA and the lab clock are for a bench server.

Extension path: keep the slot writer; add a real signature check only when the project takes on a rollback-capable bootloader and a key ceremony. Do not bolt Secure Boot onto this example in place.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

Host compiles of this crate have been checked (T-A1 and T-A2 below). That is not a hardware pass. Hardware tests below have not been run in this generation.

### Automated Tests

1. **T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)**

   From `examples/secure-ota-https/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. The build must not need a live access point or a real password in `src/`. Verified on this tree as a host compile, not a hardware pass.

2. **T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings (GPIO LED path and the 4 MB board feature). Verified on this tree as a host compile, not a hardware pass.

### Manual — hardware required

1. **T-M1 — Slot flip on a real board (ESP32-C6-DevKitC-1-N8).** This cannot be automated: the boot slot, Wi-Fi association, TLS handshake, flash write, and WS2812 colour are hardware and network behaviour. A host build cannot certify them.

   Hardware and lab setup: DevKitC-1-N8 on the **left** USB-C UART-C port, a 2.4 GHz WPA2-Personal AP, a TLS 1.3 server whose certificate is signed by the embedded CA, and an enveloped image (`espflash save-image` for `esp32c6`, then the 8-byte little-endian length and IEEE CRC32 prefix) served at the configured URL with no redirect.

   1. Override SSID, password, URL, and CA with the environment variables above. Do not commit those values.
   2. From `examples/secure-ota-https/`: `cargo run --release`.
   3. Confirm the log names `ota_0` and the RGB LED is blue (cap 64/255, visible in room light).
   4. Press BOOT (GPIO 0) only after the log says it is waiting. Confirm a blink during transfer.
   5. After reboot, confirm the log names `ota_1` and the LED is amber.
   6. Press BOOT again and confirm the device returns to `ota_0` (blue).

   Pass criterion: each successful press changes the logged slot and the LED colour. The monitor shows CRC success before reboot. The password does not appear in the log. Steps 3–6 need the board, the LED, and the network.

2. **T-M2 — Reject a bad certificate and a bad CRC (DevKitC-C6).** This cannot be automated: negative TLS and CRC behaviour depends on a live server and on flash leaving the boot selection unchanged. A host build cannot watch `otadata`.

   1. Point the URL at a TLS 1.3 server whose certificate does not chain to the embedded CA. Press BOOT. Confirm a certificate or handshake failure, the error LED pattern, and no reboot. Reset by hand and confirm the slot log is unchanged.
   2. Restore a trusted server, but serve a body whose CRC prefix does not match the bytes. Press BOOT. Confirm a CRC or length failure and an unchanged boot slot.

   Pass criterion: neither case activates a new slot. Each step needs the live server and the board.

3. **T-M3 — XIAO blink count (Seeed XIAO ESP32-C6).** This cannot be automated: the active-LOW blink count is visual. A host build cannot see GPIO 15. USB Serial/JTAG stays up because this example does not deep-sleep, but the blinks are still a hardware observation.

   1. From `examples/secure-ota-https/`: `cargo run --release --no-default-features --features board-xiao-c6`.
   2. Confirm one short blink when the log says `ota_0`, and two short blinks after a successful flip to `ota_1`.

   Pass criterion: blink count matches the slot. GPIO 15 is active LOW and the LED is off while idle after the indication.

## Spec Files

Behaviour, crates, and checks are edited in the spec files. People do not hand-edit `src/` or this README. Change the spec, then ask the agent (not the shell) **Activate esp32-sdd-full-project-generator**. That overwrites the generated crate and leaves `specs/` in place. See [Using With AI Agents](../../docs/guides/using-with-ai-agents.md).

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md) — observable behaviour (slot log, LED, download, CRC, reboot).
- [specs/CodingSpec.md](specs/CodingSpec.md) — architecture and constraints, with no code.
- [specs/TestSpec.md](specs/TestSpec.md) — how to check the example.
