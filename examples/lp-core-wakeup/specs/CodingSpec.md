<!-- See shared-specs/AIGenLessonsLearned.md — read before generating. -->

# Coding Specification — LP-Core GPIO Wakeup (ESP32-C6)

## Architecture

Two images. The HP crate is Embassy firmware on `esp-hal`. The nested `lp-core/` crate is
a tiny `esp-lp-hal` program. The HP `build.rs` compiles `lp-core/` for
`riscv32imac-unknown-none-elf` and exposes the image path to the HP binary. No CMake and
no extra top-level project files beyond the nested crate the validator already allows.

HP entry is async main from `esp-rtos`. After `esp_hal::init`, start the executor on
TIMG0 + `FROM_CPU_INTR0`. Each HP reset (power-on, LP wake, or timer wake) runs main
once: classify the wake, update the persistent counter, blink, **load and start the LP
image**, arm the backup RTC timer, enter deep sleep. Deep sleep does not return.

The LP image loops on LP GPIO 0. On an active-LOW edge it writes one handshake word into
shared LP RAM and wakes the HP core. Then it may halt or wait; the HP core reloads the
image before the next sleep anyway.

Top-level HP wiring lives in `src/main.rs`. Pins live only in `src/board.rs`. The LP
crate does not include `board.rs`; the wake GPIO number is the board's GPIO 0, passed
by the HP configuration of the pad (the LP image uses that same pad index). Fail
generation if a spec or board path asks the LP core to use GPIO 9.

## Board Abstraction

`src/board.rs` exposes LED GPIO / kind / polarity and the LP wake GPIO. That wake GPIO
is **0** on both v1 boards. `compile_error!` if both board features are on, or neither is.
Chip feature `esp32c6` only.

- **WS2812** (`board-devkitc-c6`): RMT 800 kHz NRZ, GRB, cap 64/255. Three green blinks
  on LP-GPIO wake, one slower amber blink on timer wake, black before sleep.
- **GPIO LED** (`board-xiao-c6`): active LOW, same timings, binary. No RMT.

## LP image

Git-pin `esp-lp-hal` from tag `esp-hal-v1.2.1` (in-tree 0.3.0) with feature `esp32c6`.
Reason: the LP HAL in that tag matches the HP 1.2.1 line. Do not use an S2/S3 ULP
toolchain or ULP assembly.

Keep the image small. No heap, no Embassy, no `esp-radio`, no `esp-println` requirement.
Entry is the `esp-lp-hal` entry point. Line 1 of `lp-core/src/main.rs` carries the
agent-generated marker when the generator writes it.

Handshake: one word in C6 LP RAM (`0x5000_0000` region) that the LP image writes and the
HP image reads. Choose an address that does not collide with the LP image `.bss` or stack.
Loading LP code can clear LP RAM, so the word is meaningful only after the LP core has
run. Document the address in an HP comment that points at this spec. A distinctive
pattern means "GPIO edge seen". Zero or garbage means "LP core did not report an edge".

The HP core muxes GPIO 0 to the LP pad and sets the pull-up **before** `load_lp_code`.
The LP GPIO driver assumes that mux is already done.

## HP sleep ordering

This order is a constraint, not an API recipe. Radio teardown does not apply (no radio).

The LP image is loaded and started, and LP wakeup of the HP core is enabled, **before**
HP `sleep_deep`. The backup RTC timer is armed for about 20 seconds from sleep entry so
the sleep call has a wakeup source even if LP wake were not enabled. Panic only when no
HP wakeup source is armed.

After wake, prefer the HP wakeup cause to choose the blink pattern. Read the handshake
word as confirmation. A timer wake must not increment the GPIO counter even if stale LP
RAM still holds an old handshake word — clear or ignore a stale word when the cause is
the timer. A GPIO/LP cause increments the counter.

Persistent counter: `#[esp_hal::ram(persistent)]` or `#[ram(unstable(...))]` on the HP
crate, magic plus checksum, lost on power-cycle. Not NVS, not flash, not LP RAM (LP RAM
is reloaded).

`esp-hal` feature `unstable` on the HP crate: sleep, RTC, RMT, and `load_lp_code`.
Drive the LED dark before sleep. Brief UART drain after the last log line.

There is no public per-pad hold API this application must call. Do not emit sdkconfig.

## Crates

HP crate, non-radio: crates.io `esp-hal` 1.2.1, `esp-rtos` 0.4.0, `esp-bootloader-esp-idf`
0.6.0, all with `esp32c6`. Do not git-pin the HP HAL just because `esp-lp-hal` is git-pinned.
The image must emit `esp_app_desc!()`. HP `build.rs` emits `-Tlinkall.x` last **and**
builds the nested LP crate.

LP crate: git `esp-lp-hal` only, plus its own `no_std` `no_main` binary. Do not give the
LP crate Embassy or `esp-rtos`.

No heap on either image.

## Key Constraints

- Wake pad: GPIO 0 only. GPIO 9 is a hard failure if requested.
- Timer backup: about 20 seconds from HP sleep entry.
- Blink: three ~200 ms blinks on LP-GPIO wake; one ~500 ms blink on timer wake. Embassy
  time on the HP core, not a busy-wait.
- Active window is the blink plus load. Keep it well under the 20 second timer so the
  timer is not already due when sleep starts. Arm the timer immediately before sleep.
- No `std`. No FreeRTOS. No `sdkconfig`. No touch driver. No second SoC.
- HP Embassy entry: `#[esp_rtos::main] async fn main(spawner: Spawner) -> !`.
- Zero `unwrap()` / `expect()` on fallible HAL calls in the wake path. Init may halt if
  clocks, RMT, or LP-image load cannot start. If the LP image path is missing at build
  time, fail the build rather than flash an HP image that sleeps with no LP program.
- DevKitC GPIO 0 is the download strap when held low at reset. The README must say to
  press BOOT only after the chip has booted. The LP core may see a stuck line; a stuck
  line should wake, blink, and sleep again rather than spin on the HP core. Debounce in
  the LP image enough to ignore sub-millisecond noise, without blocking the HP core.

## Preferred Crates and APIs

- crates.io `esp-hal` 1.2.1 on the HP crate (reason: non-radio pin; `load_lp_code` and
  `sleep_deep` exist on this line behind `unstable`).
- git `esp-lp-hal` 0.3.0 at tag `esp-hal-v1.2.1` for `lp-core/` (reason: matching LP HAL).
- `esp-hal` ram persistent attribute for the HP counter (reason: survives HP deep sleep;
  LP RAM does not, because the image is reloaded).
- `esp-hal` RMT for DevKitC WS2812.
- `embassy-time` for HP blinks.
- `log` + `esp-println` (`log-04`) on the HP image only.

## Non-Functional Requirements

- Logging: INFO for first boot, LP-GPIO wake with count, timer wake, LP image load, and
  sleep entry. WARN if handshake and wakeup cause disagree. No debug spam from a tight
  HP loop. The LP image does not need a serial log.
- Concurrency: one HP Embassy task. The LP core is the other execution context and does
  not share Embassy.
- Error recovery: a failed blink still proceeds to load and sleep. A missing wake source
  may halt; a missing button must not.
- README: GPIO map 0–7, GPIO 9 limitation, load-before-sleep, unmeasured LP-on current,
  unstable APIs, and which USB port keeps the monitor alive.

## Gotchas

- Loading the LP image after `sleep_deep` is impossible: flash is down, and `sleep_deep`
  does not return.
- LP RAM is not a substitute for `#[ram(persistent)]`. The reload clears handshake state.
  A stale handshake word must not count as a new press on a timer wake.
- GPIO 9 on XIAO cannot wake this design. Wiring the spec to the silkscreen BOOT button
  is a functional bug.
- DevKitC BOOT held during reset enters ROM download mode. Press after boot.
- S3 touch thresholds, `RTC_DATA_ATTR`, and `esp_sleep_enable_touchpad_wakeup` do not
  exist on this HAL and do not apply to C6.
- WS2812: RMT, GRB, cap 64/255. GPIO 8 as a plain output stays dark.
- XIAO LED is active LOW.
- USB Serial/JTAG drops on true deep sleep. Left-port DevKitC is the serial test bed.
- Do not mix a git HP `esp-hal` into this non-radio crate. Do not mix crates.io
  `esp-lp-hal` if it is not the 1.2.1 tag.
- `sleep_deep` panics if no wakeup source is armed or the timer deadline is already due.
