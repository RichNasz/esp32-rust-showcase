<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Hardware AES Secure Storage — ESP32-C6

> Hardware AES and flash-backed storage (plaintext key demo).

## Implementation status

**Firmware generated.** Crate present since the 2026-09-08 generation. Plaintext key demo, not production key storage. This label is not a hardware pass.

A spec is Markdown a person writes; the agent generates the crate from it. See [Using With AI Agents](../../docs/guides/using-with-ai-agents.md).

`esp-hal` feature **`unstable`** is on because the AES accelerator, the FLASH singleton, and RMT (WS2812) are unstable on the 1.2.1 pin. Those APIs may change with an `esp-hal` minor release. This example does not use ESP-IDF mbedTLS, Kconfig, sdkconfig, or IDF NVS.

## Overview

On every boot the firmware encrypts a known payload with on-chip **AES-256-CBC**, stores a fresh IV plus the ciphertext in a dedicated flash partition, reads it back, decrypts it, and checks the round-trip. The log says `Self-test: PASS` or `Self-test: FAIL`, and that the ciphertext differs from the plaintext. A 64 KiB hardware-AES benchmark then logs throughput in MB/s. The target on RISC-V C6 is **≥ 2 MB/s**.

The AES-256 key is created once, on first boot, from the hardware RNG, and written with `esp-storage`. Later resets load that key. It is **plaintext in flash** on purpose. The key is never printed. This is the hardware crypto API plus flash persistence, not production key storage. Production use needs flash encryption / eFuse protection (`shared-specs/security-baseline.md`).

After the self-test the status LED blinks three times, then stays dark. Pass is slow (~500 ms on/off): DevKitC WS2812 green (G=64), XIAO the active-LOW user LED. Fail is fast (~100 ms on/off): DevKitC red (R=64). A failure logs `Self-test: FAIL`, shows the fail pattern, and idles. It does not panic. Plaintext is PKCS#7-padded to the 16-byte block. Each write uses a new random IV. IV reuse with the same key is forbidden.

Pin numbers live in generated `src/board.rs`. Do not hard-code GPIOs in `main.rs`.

## Prerequisites

- **Board:** Espressif **ESP32-C6-DevKitC-1-N8** (default) or Seeed **XIAO ESP32-C6**. No extra parts.
- **USB:** DevKitC **left** USB-C (UART-C bridge, `espflash` on that port). XIAO USB-C is native USB Serial/JTAG.
- **Toolchain:** rustup **stable** (MSRV 1.95.0), `rust-src`, target `riscv32imac-unknown-none-elf`, and `espflash` / `cargo-espflash`. Install steps: [01 — Toolchain Setup](../../docs/guides/01-toolchain-setup.md).

ESP-IDF is not a prerequisite. Do not install `espup` or the `esp` rustup channel.

## Supported Boards

Chip ≠ board. v1 chip feature `esp32c6` is the SoC. Boards select pins and USB. Not `idf.py set-target`.

Auto-discovered from `board-specs/` for ESP32-C6 with a user LED. Both catalog boards pass. Default board: Espressif ESP32-C6-DevKitC-1-N8.

| Board | board-spec | Chip feature | Board feature | LED GPIO | LED polarity | LED type | USB console |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | [espressif/esp32-c6-devkitc-1-n8.md](../../board-specs/espressif/esp32-c6-devkitc-1-n8.md) | `esp32c6` | `board-devkitc-c6` | GPIO 8 | — | WS2812 RGB | UART bridge (`espflash` on the UART-C port) |
| Seeed XIAO ESP32-C6 | [seeed/xiao-esp32c6.md](../../board-specs/seeed/xiao-esp32c6.md) | `esp32c6` | `board-xiao-c6` | GPIO 15 | active LOW | Simple GPIO | USB Serial/JTAG |

A GPIO level alone produces no visible output — use an RMT (or equivalent `esp-hal`) driver to generate the 800 kHz NRZ waveform.

RTC GPIOs on C6 are 0–7 only. GPIO 9 (XIAO BOOT) is not an RTC pin. This example does not sleep, so that limit does not change behaviour.

## Per-Board Behavior

| Board | LED feedback |
| --- | --- |
| Espressif ESP32-C6-DevKitC-1-N8 *(default)* | WS2812 colour: green = pass, red = fail, off = idle (three blinks, cap 64/255) |
| Seeed XIAO ESP32-C6 | On / off, active LOW (three blinks, then off) |

The first difference you will see is the LED, not a sleep current. This example does not deep-sleep, so the console stays up on both boards. On DevKitC a write to GPIO 8 does not light the WS2812 — the firmware drives RMT (GRB, 800 kHz NRZ, brightness at or under 64/255). On XIAO the blue user LED is active LOW on GPIO 15, with no RMT. Flash the DevKitC on the **left** USB-C UART-C port. XIAO uses USB Serial/JTAG.

## Build & Flash

From this directory:

```sh
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

The zero-warning check in Testing is the release-profile clippy invocation (`cargo clippy --release -- -D warnings`).

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

Non-radio pins (crates.io, not the git radio set): `esp-hal` 1.2.1, `esp-rtos` 0.4.0, `esp-storage` 0.10.0 with feature `esp32c6`. Do not mix crates.io `esp-rtos` 0.4.0 with a git `esp-hal`. There is no heap.

The runner passes `--partition-table partitions.csv`. The `storage` data partition (subtype `0x40`) is 64 KiB at offset **0x110000**, immediately after a 1 MiB factory app. One 4 KiB sector holds the AES key; the next holds the self-test blob. The table fits 4 MB (XIAO) and 8 MB (DevKitC-N8).

| Name | Type | SubType | Offset | Size |
| --- | --- | --- | --- | --- |
| nvs | data | nvs | 0x9000 | 0x6000 |
| phy_init | data | phy | 0xF000 | 0x1000 |
| factory | app | factory | 0x10000 | 1 MiB |
| storage | data | 0x40 | 0x110000 | 64 KiB |

### Opening in VS Code / Cursor

This folder is a complete standalone Cargo package. Open **only this folder**
(not the repository root) so rust-analyzer uses this crate.

**File → Open Folder… → select `examples/hardware-aes-secure-storage/`**

This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
(RISC-V target + `espflash` runner). Do not open the top-level repository
folder as your active workspace while developing this example.

## Expected behavior

These are the lines the firmware logs. They are not a transcript from a board. The throughput figure below is the shape of the log. The pass bar on C6 silicon is **≥ 2 MB/s**. The AES key is never printed.

### First boot (ESP32-C6-DevKitC-1-N8)

```
hardware_aes_secure_storage: AES storage — WS2812 RGB LED, GPIO 8
hardware_aes_secure_storage: key generated, saved to storage
hardware_aes_secure_storage: ciphertext differs from plaintext
hardware_aes_secure_storage: Self-test: PASS
hardware_aes_secure_storage: hardware AES throughput 6.553 MB/s
hardware_aes_secure_storage: WS2812 RMT init on GPIO 8 (GRB order, max brightness 64/255)
```

Three slow green blinks, then dark.

### After reset

```
hardware_aes_secure_storage: key loaded from storage
hardware_aes_secure_storage: Self-test: PASS
```

### Seeed XIAO ESP32-C6 (active LOW, GPIO 15)

```
hardware_aes_secure_storage: AES storage — GPIO LED (active-LOW), GPIO 15
hardware_aes_secure_storage: key generated, saved to storage
hardware_aes_secure_storage: Self-test: PASS
hardware_aes_secure_storage: GPIO LED on GPIO 15 (GPIO LED (active-LOW))
```

Three slow blinks on the blue user LED, then off. Fail on either board is three fast blinks (DevKitC red) and `Self-test: FAIL`, then idle.

## Key Concepts

- **Standalone Cargo crate** — `#![no_std]` `#![no_main]`; Embassy entry via `esp-rtos`; no workspace at the repo root
- **`esp-hal` AES** — hardware accelerator (`esp_hal::aes`, feature `unstable`); AES-256-CBC; key length from the 32-byte key; not a software AES crate
- **`esp-storage`** — `FlashStorage` from the FLASH singleton; key and ciphertext in the `storage` partition; not IDF NVS
- **Unique IV per write** — hardware RNG; IV stored with the blob; PKCS#7 padding
- **Round-trip self-test** on boot; LED pass/fail (three blinks, cap 64/255); failure does not panic
- **Throughput log** — 64 KiB encrypt; RISC-V C6 target ≥ 2 MB/s; no heap and no `f32` formatting crate
- **Security model** — example key is stored as generated plaintext in flash; not a full HSM; no `const` key in `src/`
- **Crates.io pin** — `esp-hal` 1.2.1 + `esp-rtos` 0.4.0 + `esp-storage` 0.10.0; `unstable` for AES, FLASH, and RMT

## Limitations

| This example is NOT | Why |
| --- | --- |
| Production key management | Key is plaintext in flash; no eFuse wrapping |
| Authenticated encryption | AES-CBC without HMAC / GCM; ciphertext is not integrity-protected |
| IDF NVS or mbedTLS | rust-on-metal `esp-hal` + `esp-storage` only |
| Flash encryption / Secure Boot enablement | Irreversible eFuse operations are out of scope |

Extension path: wrap the stored key with flash encryption / eFuse; add an HMAC or switch to an AEAD; keep using hardware AES.

## Testing

Testing philosophy: automated first, manual only when hardware observation is unavoidable.

### Automated Tests

1. **T-A1 — Zero-warning build (ESP32-C6-DevKitC-1-N8, default features)**

   From `examples/hardware-aes-secure-storage/`:

   ```sh
   cargo build --release
   cargo clippy --release -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This checks the DevKitC path (WS2812 / RMT, hardware AES, `esp-storage`). It does not need a board.

2. **T-A2 — Zero-warning build (Seeed XIAO ESP32-C6)**

   ```sh
   cargo build --release --no-default-features --features board-xiao-c6
   cargo clippy --release --no-default-features --features board-xiao-c6 -- -D warnings
   ```

   Pass criterion: both commands exit 0 with no warnings. This checks the GPIO LED + AES + flash path.

3. **T-A3 — Flash via the espflash runner (DevKitC-C6)**

   ```sh
   cargo run --release
   ```

   The runner is `espflash flash --monitor --partition-table partitions.csv`. Pass criterion: the image flashes and the monitor shows `Self-test: PASS` and a throughput figure. Use the DevKitC **left** USB-C UART-C port. The build half is T-A1 and does not need a board. The flash half needs hardware, which a host compile cannot attach.

### Manual — hardware required

1. **T-M1 — Self-test and LED (ESP32-C6-DevKitC-1-N8).** This cannot be automated: it checks that the AES accelerator runs on C6 silicon (a simulator may take a software path) and that the WS2812 pattern is visible.

   Hardware: DevKitC-1-N8, USB-C on the **left** UART-C port (often `/dev/cu.usbserial-*` on macOS).

   1. From `examples/hardware-aes-secure-storage/`: `cargo run --release`.
   2. Confirm the log says the key was generated and saved (first flash) or loaded (later reset).
   3. Confirm `Self-test: PASS` and that ciphertext differs from plaintext.
   4. Confirm a hardware AES throughput line of **≥ 2 MB/s**.
   5. Watch the RGB LED: three slow green blinks, then dark.

   Pass criterion: PASS logged, throughput ≥ 2 MB/s, green 3-blink visible. Steps 4 and 5 need the chip and your eyes. A host build cannot see the LED or the accelerator.

2. **T-M2 — Key persistence across reset.** This cannot be automated: persistence needs a real flash write and a second boot on the same chip.

   Hardware: same as T-M1 (UART-C so the monitor stays connected). This example does not sleep.

   1. First boot after a full flash: the log says the key was generated and saved.
   2. Press RST (RST keeps the flash contents; a power-cycle is not required).
   3. Confirm the log says the key was loaded from storage, not generated.
   4. Confirm `Self-test: PASS` still appears.

   Pass criterion: the key loads after reset and the self-test still passes. A host build cannot press RST or read that flash sector.

3. **T-M3 — GPIO LED pass pattern (Seeed XIAO ESP32-C6).** This cannot be automated: active-LOW polarity and blink timing are visual.

   Hardware: Seeed XIAO ESP32-C6, USB-C cable.

   1. From `examples/hardware-aes-secure-storage/`: `cargo run --release --no-default-features --features board-xiao-c6`.
   2. Confirm the log names the GPIO LED path (not WS2812 / RMT) on GPIO 15, and `Self-test: PASS`.
   3. Watch the blue user LED: three slow blinks, then off. Polarity is active LOW.

   Pass criterion: correct polarity, three blinks, PASS in the log. A camera or a person has to see GPIO 15. The host compiler cannot.

## Spec Files

Behaviour, crates, and checks are edited in the spec files. People do not hand-edit `src/` or this README. Change the spec, then ask the agent (not the shell) **Activate esp32-sdd-full-project-generator**. That overwrites the generated crate and leaves `specs/` in place. See [Using With AI Agents](../../docs/guides/using-with-ai-agents.md).

- [specs/FunctionalSpec.md](specs/FunctionalSpec.md) — observable behaviour (what you see on the LED and the serial log).
- [specs/CodingSpec.md](specs/CodingSpec.md) — architecture and constraints, with no code.
- [specs/TestSpec.md](specs/TestSpec.md) — how to check the example.
