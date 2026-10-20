<!-- See shared-specs/AIGenLessonsLearned.md — read before generating. -->

# Coding Specification — HTTPS OTA Slot Flip (ESP32-C6)

## Architecture

Embassy firmware that stays awake. Entry is async main from `esp-rtos`. After
`esp_hal::init`, start the Embassy executor on TIMG0 + `FROM_CPU_INTR0` (C6) and
initialize a heap (justified below). Read the running app slot from the partition table,
log it, drive the slot indication on the onboard LED, then wait for a qualified button
press. A press runs one pipeline: Wi-Fi STA, one HTTPS GET, stream the image into the
inactive OTA slot, check the envelope, activate that slot, reboot. Any failure tears the
radio down if it is up, shows the error indication, and returns to the wait. Do not reboot
on failure.

Top-level wiring lives in `src/main.rs`. Board GPIO, LED kind, polarity, and the trigger
button live only in generated `src/board.rs`. Split the HTTP/TLS reader and the slot
writer into extra modules if `main.rs` would pass about 500 lines.

There is one firmware image. Slot identity comes from the bootloader metadata, not from
two separately authored programs.

## Board Abstraction

`src/board.rs` is generated from `board-specs/`. It exposes LED GPIO / kind / polarity and
the runtime trigger GPIO (DevKitC GPIO 0, XIAO GPIO 9). `compile_error!` if both board
features are enabled, or if neither is. Application code must not hardcode pin numbers.
Chip feature is `esp32c6` only.

- **WS2812** (default `board-devkitc-c6`): RMT 800 kHz NRZ, GRB, cap 64/255. Blue for
  `ota_0`, amber for `ota_1`, blink during transfer, red on failure.
- **GPIO LED** (`board-xiao-c6`): active LOW. One short blink for `ota_0`, two for
  `ota_1`, rapid blink during transfer, a slower pattern on failure. No RMT on this board.

## Partition and slot flip

Use profile `dual-ota` (`shared-specs/partition-table-profiles.md`). Emit `partitions.csv`
and an espflash flash-size setting: 8 MB on DevKitC-C6, 4 MB on XIAO C6. The first
installed image must boot from `ota_0` at the profile offset. Do not invent a factory
partition and do not emit `sdkconfig`.

Slot updates go through `esp-bootloader-esp-idf`'s OTA updater plus `esp-storage` flash
access. That is the same building block as the `esp-hal` 1.2.1 `examples/ota/update`
example: read the table, write the inactive app partition, activate it. Reason: `otadata`
stays in the format the IDF second-stage bootloader already understands. This crate is
image metadata and partition helpers, not an ESP-IDF checkout.

Write the image in flash-sized chunks while the body arrives. Do not hold the image in
RAM. Activate the next slot only after the received length and IEEE CRC32 match the
8-byte little-endian prefix (length, then CRC32 of the image bytes). On mismatch, leave
the current boot selection in place.

The stock `espflash` bootloader is **not** built with OTA auto-rollback. Recording the
new image state is still appropriate so a rollback-capable bootloader can see it, but
this example must not depend on automatic revert, and it must not look for
`CONFIG_BOOTLOADER_APP_ROLLBACK_ENABLE`. Do not call ESP-IDF OTA C APIs.

## Radio, HTTP, and TLS

This is a **radio** example. Copy the crates.io radio pin from
`shared-specs/rust-toolchain.md`: `esp-hal` `=1.2.1`, `esp-radio` (features `esp32c6`,
`unstable`, `wifi`) `=1.0.0-beta.1`, `esp-rtos` (features `esp32c6`, `embassy`,
`esp-radio`) `=0.4.0`, `esp-alloc` `=0.11.0`, `esp-bootloader-esp-idf` `=0.6.0`, and
`esp-storage` `=0.10.0`. Do not git-pin those HP crates. Mixing a git `esp-hal` with
crates.io `esp-storage` builds two HALs.

Wi-Fi STA uses `esp-radio` + `embassy-net` 0.9 with DHCPv4, same shape as the BME280
example. Association is capped at about 15 seconds. DNS only if the URL host is not a
literal IPv4 address.

TLS is `embedded-tls` **0.19.0** with default `std` disabled. Reason: a `no_std` TLS 1.3
client with a caller-supplied trust anchor, and no mbedTLS / ESP-IDF. The crate handles
one record at a time; a record buffer larger than 16 KiB is unnecessary. The lab server
must offer TLS 1.3. Verify the server certificate against the embedded CA. Do not provide
a "skip verify" path.

HTTP is one HTTP/1.1 GET written in-crate over that TLS stream, then a streamed body.
Reason: the same tradeoff as the MQTT example's small encoder — one request does not
justify a general HTTP client. The URL is final; do not follow redirects. Bound the
overall transfer (on the order of 60 seconds, or end-of-body, whichever comes first) so
a stalled server returns to the wait.

Heap is justified for the Wi-Fi blob, `embassy-net`, and TLS records. Size it for those,
not for a full OTA slot. `build-std` includes `alloc` when the radio stack needs it.
`#[ram(reclaimed)]` is allowed only when the app descriptor is linked. The image must
emit `esp_app_desc!()`. `build.rs` emits `-Tlinkall.x` last.

## Credentials and trust anchor

SSID, password, and the OTA URL are compile-time environment variables (`env!` / Cargo
`[env]`), with placeholders so a clean build does not need a lab network. A placeholder
SSID skips association and stays in the wait. The CA PEM is embedded at build time from
a path the environment or the example tree supplies. Ship a placeholder CA that does not
authenticate a real server, and document that the operator replaces it. Never log the
password.

## Button

The trigger GPIO comes from `board.rs`. Treat the line as a press only after a release
followed by a new active-LOW edge, debounced with Embassy time (tens of milliseconds).
Reason: DevKitC GPIO 0 is the download strap if it is low at reset. Ignore a level that
is already low when the wait begins.

## Key Constraints

- No deep sleep, no LP core, no second firmware image, no external LED board.
- Do not activate a slot on certificate failure, short read, length mismatch, or CRC mismatch.
- Do not reboot on those failures.
- No `std`. No FreeRTOS. No `sdkconfig`. No Kconfig. No `esp_https_ota`.
- Embassy entry: `#[esp_rtos::main] async fn main(spawner: Spawner) -> !`.
- Zero `unwrap()` / `expect()` on fallible HAL, radio, TLS, flash, or OTA calls in the
  transfer path. Init may halt if clocks, heap, or RMT cannot start.
- `esp-hal` feature `unstable` is required for Wi-Fi and RMT. Document that, and that
  `esp-radio` is an unstable beta.

## Preferred Crates and APIs

- Crates.io `esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1`, `esp-rtos` `=0.4.0`,
  `esp-alloc` `=0.11.0`, `esp-bootloader-esp-idf` `=0.6.0`, and `esp-storage` `=0.10.0`
  (reason: one HAL, and the OTA updater lives on the bootloader crate).
- `embassy-net` 0.9 (reason: same STA stack as the other Wi-Fi example).
- `embedded-tls` 0.19.0, no `std` (reason: TLS 1.3 trust-anchor check without ESP-IDF).
- `esp-hal` RMT for the DevKitC WS2812; `embassy-time` for debounce, blinks, and timeouts.
- `log` + `esp-println` (`log-04`).

## Non-Functional Requirements

- Logging: INFO for slot, association result without the PSK, coarse progress, CRC pass,
  and activate. WARN for timeouts and CRC or certificate rejection. Never print the PSK.
- Concurrency: a net-stack runner task plus main is enough. Blink from the download path
  or from one short-lived Embassy task. No reconnect loop that ignores the timeout.
- Error recovery: every failed stage returns to the button wait with the radio down.
- The README must tell the operator how to build an image with `espflash save-image`, wrap
  it in the 8-byte length-and-CRC prefix, serve it with TLS 1.3, and replace the
  placeholder CA and Wi-Fi environment values. It must state that auto-rollback is not
  provided by stock `espflash`.

## Gotchas

- Activating a truncated image and rebooting looks like a dead board. The CRC gate is
  the mitigation. Do not activate first and check later.
- GPIO 0 low at reset on DevKitC enters the ROM download mode. The qualified-press rule
  exists so a stuck button does not also fire OTA the moment the app starts.
- `embedded-tls` 0.19.0 will not handshake with a TLS 1.2-only server. Say that in the README.
- A single trust anchor is not a browser CA store. A public URL signed by a different root
  fails closed. That is success for the negative test, not a bug.
- XIAO has 4 MB flash. The `dual-ota` profile fits. An 8 MB-only layout does not.
- Mixing crates.io `esp-storage` or `esp-bootloader-esp-idf` with git `esp-hal` duplicates
  the HAL and fails the build in confusing ways.
- WS2812 brightness above 64/255 is painfully bright; a GPIO toggle on GPIO 8 does not light the LED.
- DevKitC logs belong on the left USB-C UART-C port.
