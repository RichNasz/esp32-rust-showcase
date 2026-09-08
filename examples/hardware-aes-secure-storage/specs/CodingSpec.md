<!-- See shared-specs/AIGenLessonsLearned.md — read before generating. -->

# Coding Specification — Hardware AES Secure Storage (ESP32-C6)

## Architecture

Single Embassy firmware. Entry is an async main provided by `esp-rtos`; after
`esp_hal::init`, start the Embassy executor on TIMG0 + `FROM_CPU_INTR0` (C6). Main
wires peripherals, starts the AES work-queue backend, loads or creates the key,
runs the round-trip self-test, logs throughput, indicates pass/fail on the LED,
then idles. There is no radio and no deep sleep.

Top-level wiring lives in `src/main.rs`. Board GPIO / LED kind / polarity live only
in generated `src/board.rs`. Domain modules:

- **Crypto** — hardware AES-256-CBC encrypt/decrypt, PKCS#7 pad/unpad, 64 KiB
  throughput measurement.
- **Store** — `esp-storage` flash persistence for the key sector and the blob
  sector. Not IDF NVS.
- **LED** — pass/fail indication (WS2812 vs GPIO).

Two compile-time LED paths, selected by mutually exclusive Cargo board features:

- **WS2812 path** (default `board-devkitc-c6`): RMT generates 800 kHz NRZ.
- **GPIO LED path** (`board-xiao-c6`): simple active-LOW LED. Do not run the RMT
  encoder on this board.

## Board Abstraction

`src/board.rs` is generated from `board-specs/`. It exposes LED GPIO, LED kind
(WS2812 vs simple GPIO), and polarity. `compile_error!` if both `board-devkitc-c6`
and `board-xiao-c6` are enabled, or if neither is. Application code must not
hardcode pin numbers. Chip feature is `esp32c6` only.

## Crypto

Enable `esp-hal` feature `unstable` — AES, FLASH, and RMT require it on this pin.

Use the ESP32-C6 AES accelerator (`esp_hal::aes`). On the 1.2 line the work-queue
backend plus a CBC context is the multi-block path; key length is taken from the
32-byte key, not a mode enum. Hardware AES is required; do not substitute a
pure-software crate.

CBC with a unique IV per blob is mandatory. Generate a fresh IV from the hardware
RNG for every write, including every self-test. Store the IV with the ciphertext
(IV is not a secret in CBC, but do not treat it as a key). Never reuse an IV with
the same key.

Pad any plaintext to the 16-byte block boundary with PKCS#7 before encrypt. Reject
invalid padding on decrypt. Self-test plaintext must not be a multiple of the block
size so padding is actually exercised.

The 64 KiB throughput buffer is static (the Embassy main stack cannot hold it).
Time the encrypt with the HAL monotonic clock (busy-wait AES does not advance
Embassy time). Log an integer MB/s figure; no heap, no `f32` formatting crate.
Target ≥ 2 MB/s on C6.

Start the AES backend once and keep it alive for every encrypt/decrypt. Dropping
it leaves queued operations incomplete.

Never log the AES key, key checksum material, or raw key bytes.

## Storage

Use `esp-storage` `FlashStorage` constructed from the FLASH singleton. Feature
`esp32c6` on `esp-storage` (crates.io 0.10.0). This is raw flash, not IDF NVS.

Persist at fixed offsets inside the dedicated `storage` partition from the
partition table (base 0x110000): one 4 KiB sector for the key record, the next
sector for the blob (IV + ciphertext length + ciphertext). Detect a valid key by
magic plus a checksum so erased flash (0xFF) is first boot, not a loaded key.

The Embassy main stack is small. Prefer NOR erase + aligned `write_nor` / `read_nor`
so the driver does not place a 4 KiB sector bounce buffer on the stack. Align
records to four bytes.

First boot: fill a 32-byte key from the hardware RNG, write it, log that it was
generated and saved. Later boots: load it and log that it was loaded. Do not
regenerate a valid key.

Self-test failure (crypto, padding, compare, or flash I/O) is a logged FAIL plus
the fail LED pattern. Do not panic.

## WS2812 path

Use `esp_hal::rmt` (`PulseCode` newtype on the 1.2 line). Encode 800 kHz NRZ
(T0H ~300 ns / T0L ~900 ns, T1H ~900 ns / T1L ~300 ns) and a reset/latch of at
least 50 µs low, preferably in the RMT buffer. Hold the line LOW after a frame.
Pack colours as GRB, not RGB.

Write black once at init. Cap every channel at 64/255. Pass is green at that cap;
fail is red at that cap. Three blinks: ~500 ms on/off for pass, ~100 ms on/off for
fail. End dark. Time waits with Embassy, not busy-wait. Log GPIO, GRB order, and
that RMT is driving NRZ at INFO.

## GPIO LED path

Drive the XIAO LED at the polarity documented in its board-spec (active LOW).
Three blinks at the same pass/fail cadences as the WS2812 path. No PWM, no LEDC,
no RMT.

## Key Constraints

- AES-256-CBC only; unique IV per write; PKCS#7 padding.
- 64 KiB throughput encrypt; log MB/s; ≥ 2 MB/s is the FunctionalSpec bar.
- No heap. No `std`. No FreeRTOS. No `sdkconfig`. No IDF NVS. No mbedTLS.
- Non-radio crates.io pins: `esp-hal` 1.2.1, `esp-rtos` 0.4.0, `esp-storage`
  0.10.0, `esp-bootloader-esp-idf` 0.6.0. Do not git-pin the radio stack.
- The image must include an ESP-IDF-compatible application descriptor
  (`esp-bootloader-esp-idf`). Current `espflash` will not flash without it.
- `build.rs` emits `-Tlinkall.x` last.
- Embassy entry: `#[esp_rtos::main] async fn main(spawner: Spawner) -> !`. Do not
  use blocking `#[esp_hal::main]`.
- Zero `unwrap()` / `expect()` on fallible HAL, AES, or flash in the demo path.
  Init may halt if clocks or RMT cannot start.
- `.cargo/config.toml` runner must pass `--partition-table partitions.csv`.

## Preferred Crates and APIs

- `esp-hal` AES work-queue / CBC context (reason: hardware accelerator; key length
  from the key; software AES would miss the point of the example).
- `esp-storage` `FlashStorage` (reason: flash-backed blobs without IDF NVS).
- `esp-hal` RNG (reason: IV and first-boot key material; no software seed).
- `esp-hal` RMT for WS2812 (reason: hardware NRZ; a GPIO level produces no light).
- `embassy-time` for LED cadence (reason: cooperative delay).
- `log` + `esp-println` (`log-04`) for INFO operational events.

## Non-Functional Requirements

- Logging: INFO for key generated/loaded, self-test result, throughput, LED init.
  ERROR for flash/AES failures. Never log the key.
- Concurrency: one Embassy task is enough; do not spawn extra tasks.
- Error recovery: self-test failure is FAIL + LED + idle, not abort.
- Memory: static 64 KiB bench buffer; stack-bounded self-test buffers.

## Gotchas

- IV uniqueness is a hard CBC requirement. A fixed IV is a security defect
  (`security-baseline.md`).
- AES, FLASH, and RMT live behind `esp-hal` `unstable` on this pin — document
  that in the README.
- `FlashStorage::write` / `read` always allocate a 4 KiB stack bounce; the
  Embassy main stack is 8 KiB. Use aligned NOR operations instead.
- A GPIO level on the DevKitC WS2812 produces no light — indistinguishable from
  an unsoldered LED jumper until NRZ + brightness ≥ room-light visibility are
  confirmed.
- Channel values above 64/255 are painfully bright; 8/255 is invisible in room
  light. A single 500 ms blink is easy to miss — use three.
- XIAO LED is active LOW: forgetting polarity looks like “stuck on” or “never on”.
- Do not mix crates.io `esp-rtos` 0.4.0 with git `esp-hal`. This example is
  non-radio.
- Plaintext flash keys are a demo limitation, not a bug. Do not “fix” them by
  hardcoding a key in `src/`.
- The AES backend must remain started for the lifetime of encrypt/decrypt calls.
