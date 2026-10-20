<!-- See shared-specs/AIGenLessonsLearned.md — read before generating. -->

# Coding Specification — ESP-NOW Sensor to Gateway (ESP32-C6)

## Architecture

One crate, two compile-time roles selected by mutually exclusive Cargo features
`role-sensor` and `role-gateway`. Default features are `board-devkitc-c6` and
`role-sensor`. `compile_error!` if both roles are set or neither is. Board features stay
mutually exclusive the same way. `--no-default-features` drops both the board and the
role, so every non-default command must name the board feature and the role feature.

Embassy entry from `esp-rtos` after `esp_hal::init`, executor on TIMG0 + `FROM_CPU_INTR0`.
Initialize a heap for the radio blob, then branch on the role feature.

**Sensor** is single-shot, same shape as the other sleepers: persist the boot count, log
the wakeup cause, send one datagram or skip if the gateway MAC is unset, blink, tear the
radio down, arm wakeup, deep-sleep. Deep sleep does not return.

**Gateway** is a long-lived receive loop. It logs its station MAC once, then waits for
datagrams and blinks. It must not call deep sleep.

Top-level wiring lives in `src/main.rs`. Pins live only in `src/board.rs`. A small
payload module is appropriate so the frame layout has one definition for both roles.

## Board Abstraction

`src/board.rs` exposes LED GPIO / kind / polarity and the RTC wake GPIO (GPIO 0 on both
boards). Application code must not hardcode pin numbers. Chip feature is `esp32c6` only.

- **WS2812** (`board-devkitc-c6`): RMT 800 kHz NRZ, GRB, cap 64/255. Sensor: one green
  flash on success, red on failure or skipped MAC, black before sleep. Gateway: one blue
  flash per accepted frame.
- **GPIO LED** (`board-xiao-c6`): active LOW, binary blinks with the same meanings
  (success once, failure a double blink). No RMT.

## Persistence (sensor only)

Boot counter in persistent RAM via `#[esp_hal::ram(persistent)]` or the
`#[ram(unstable(...))]` form on this pin. Magic plus checksum so a raced reset is first
boot. Increment after HAL init, before radio work. The gateway does not need this static.
Do not use NVS or flash for the counter.

## Sleep and wakeup (sensor only)

`esp-hal` feature `unstable` is required for sleep, RTC, RMT, and the radio.

Arm an RTC timer for about 15 seconds from sleep entry, and RTC GPIO 0 active LOW with
a pull-up. GPIO 9 is not a wake source. Enter deep sleep only after the radio is torn
down. Panic only if no wakeup source is armed. Unrecognised causes log a warning and
still run the send path.

**Tear down `esp-radio` before sleep.** Leaving ESP-NOW up keeps the radio in milliamps.
Drive the LED dark before sleep. Drain the UART briefly after the last log line.

The active window is capped at about 3 seconds from wake (send wait included). On expiry,
tear down and sleep.

## ESP-NOW

Git-pin `esp-hal`, `esp-radio`, `esp-rtos`, `esp-alloc`, and `esp-bootloader-esp-idf`
from tag `esp-hal-v1.2.1`. Enable `esp-radio` features `esp32c6`, `unstable`, and
`esp-now` (and the Wi-Fi driver feature that `esp-now` requires on this pin). Do not
enable `embassy-net`. Do not associate to an AP. Do not scan.

Reason: ESP-NOW is a peer datagram on the 802.11 radio. A TCP stack and an AP session
are the MQTT example's problem, not this one. `esp-now` is an unstable beta API. Say so
in the README.

Heap is justified for the radio blob, on the order of the other radio examples (about
72 KiB). No image-sized buffers. The image must emit `esp_app_desc!()`. `build.rs` emits
`-Tlinkall.x` last.

Both roles take the channel from compile-time `ESPNOW_CHANNEL` (default `1` so the crate
builds). Document that both images must match.

The sensor takes `ESPNOW_GATEWAY_MAC` as hex text. The all-zero placeholder skips the
send. The gateway reads its own station MAC at runtime and logs it. The sensor adds that
MAC as the unicast peer. The gateway accepts frames from any sender and filters on the
magic in the payload.

The payload is a fixed byte layout shared by both roles: boot count and a constant magic.
It must stay under the 250-byte ESP-NOW maximum. The send wait is about 200 ms; then
success or failure, never an unbounded block.

No relay table, no beacon timer, no light sleep, no per-role `sdkconfig`.

## Key Constraints

- Sensor sleep: 15 seconds from sleep entry. Active cap: about 3 seconds.
- Gateway: no deep sleep.
- No `std` beyond what the radio heap requires (`alloc` via `esp-alloc` only). No FreeRTOS.
  No Kconfig. No `sdkconfig`.
- Embassy entry: `#[esp_rtos::main] async fn main(spawner: Spawner) -> !`.
- Zero `unwrap()` / `expect()` on fallible radio or HAL calls in the send/receive path.
  Init may halt if the radio or heap cannot start.
- Placeholder env values must compile.

## Preferred Crates and APIs

- Git-pinned `esp-hal` / `esp-radio` / `esp-rtos` / `esp-alloc` / `esp-bootloader-esp-idf`
  at `esp-hal-v1.2.1` (reason: `esp-now` is not on a crates.io release that matches HAL 1.2.1).
- `esp-radio` ESP-NOW (reason: first-party datagram API). Not an application mesh crate.
- `esp-hal` RTC / `LowPower` for the sensor sleep path (reason: same metal sleep API as
  the timer-node example).
- `esp-hal` ram persistent attribute for the sensor boot counter.
- `esp-hal` RMT for DevKitC WS2812; `embassy-time` for the send wait and blinks.
- `log` + `esp-println` (`log-04`).

## Non-Functional Requirements

- Logging: INFO for role, channel, gateway MAC (gateway), boot count, send result, and
  each accepted receive (sender MAC and count). WARN for the unset-MAC skip, send timeout,
  and unexpected wakeup cause. No secrets exist to log; still do not log anything that
  looks like a PSK.
- Concurrency: sensor stays on the main task. Gateway may use the driver's receive
  callback or an async receive, but LED updates and log lines must not run away unbounded.
  Do not spawn a second networking stack.
- Error recovery: sensor always reaches teardown and sleep. Gateway stays up across a
  malformed frame.
- README: state the historical directory name, the two-board lab procedure, the channel
  footgun, the unstable `esp-now` API, and that XIAO BOOT (GPIO 9) will not wake the sensor.

## Gotchas

- ESP-NOW is not a mesh. Generating relay roles, discovery beacons, or per-role sdkconfig
  files contradicts this spec.
- Channel mismatch looks like "the gateway is dead". Log the channel on both roles.
- The all-zero MAC must not be registered as a peer and then retried forever.
- Radio teardown is mandatory on the sensor. Skipping it makes the power budget false.
- GPIO 9 cannot wake the sensor. GPIO 0 can, including XIAO D0.
- Persistent RAM survives deep sleep and clears on power-cycle. Checksum required.
- DevKitC sensor logs: left USB-C. A USB Serial/JTAG monitor that exits on sleep is expected.
- Do not mix crates.io `esp-rtos` 0.4.0 with git `esp-hal`.
- WS2812 needs RMT and a 64/255 cap. A GPIO write on GPIO 8 stays dark.
