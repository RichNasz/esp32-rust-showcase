<!-- ================================================
     HUMAN-WRITTEN — GOLD STANDARD INSTITUTIONAL MEMORY
     ================================================ -->

# AIGenLessonsLearned.md — Gold Standard Lessons Learned

**MANDATORY FIRST STEP**: Every time you generate, regenerate, fix, or debug ANY code in this repository, read this file first.

## Core Rules (Never Violate)
- Humans only ever edit `specs/`, `shared-specs/`, and `examples/*/specs/{Functional,Coding,Test}Spec.md`.
- Never edit generated files (`src/`, `Cargo.toml`, `build.rs`, `.cargo/config.toml`, README.md, etc.) by hand.
- If something is wrong, improve the spec — never patch the generated code.
- Always respect the permanent example structure in `specs/example-project-structure-spec.md`.
- Always reference `board-specs/` files instead of hard-coding pins.
- There is no `sdkconfig`. Do not generate one, do not document menuconfig, and do not "fix" missing Kconfig symbols.
- New agent sessions load SDD rules from root `AGENTS.md` (Grok auto-loads it). Keep that file short; it points here and at `skills/`.

## Key Lessons Learned from Real AI Generation (Updated Sep 2026)

- Precise board references + `board-specs/*.md` dramatically reduce hallucinations.
- Explicit power budgets, wakeup sources, and peripheral constraints in FunctionalSpec.md are essential for ESP32-C6.
- When switching boards (DevKitC-C6 ↔ XIAO C6), change only the Cargo board feature and regenerate — GPIO lives in generated `src/board.rs`.
- Include watchdog, error handling, and deep-sleep current targets in every spec that sleeps.
- Sine-wave breathing, debouncing, and state machines work best when the algorithm is fully described in the CodingSpec.
- The first error is almost always a missing constraint in the spec, not a bad AI.

- **ESP32-C6 RTC GPIOs are 0–7 only.** GPIO 9 (XIAO boot button) is **not** an RTC GPIO and cannot wake the chip from deep sleep. DevKitC-C6 boot is GPIO 0 (RTC-capable). When a board's only physical button is not RTC-capable, use timer-only wakeup (or an external button on LP GPIO 0–7). Document the limitation; do not invent a software workaround. (Mar 2026, retargeted Sep 2026)

- **Never hardcode pins.** Read the board-spec Onboard Peripherals table. Generated `src/board.rs` is the only place pin numbers belong.

- **WS2812 status LED: GRB colour order, 800 kHz NRZ, ~300 ns / ~900 ns bit times, brightness capped at 64/255, at least 3 blinks for pass/fail.** A GPIO level alone produces no visible output — use RMT (or the `esp-hal` equivalent). Brightness 8/255 is invisible in room light; a single 500 ms blink is easy to miss. The symptom ("LED didn't light up") is indistinguishable from a hardware fault until brightness and the NRZ waveform are ruled out. (Apr 2026)

- **USB-UART vs USB Serial/JTAG vs USB CDC consoles behave differently in deep sleep.**
  - UART-bridge boards (ESP32-C6-DevKitC-1-N8 **left** USB-C, CH343P → UART0) keep the serial port alive across sleep. Monitor shows silence during sleep, output resumes on wake.
  - Native USB Serial/JTAG (DevKitC-C6 **right** USB-C; XIAO ESP32-C6) and USB CDC drop the `/dev/cu.usbmodem*` port when the USB peripheral is powered down for deep sleep. `espflash monitor` / `cargo run` exits on the first sleep entry. There is no firmware workaround that preserves the USB connection during true deep sleep.
  Direct readers to the DevKitC-C6 UART-bridge port for counter-persistence tests; USB-native boards are LED-only observation. (Apr 2026, retargeted Sep 2026)

- **Embassy `into_async` belongs on the core that owns the driver.** Do not convert a peripheral to async on one executor and then use it from another task/core. Do not keep a blocking handle and an async handle to the same peripheral. (Sep 2026)

- **Radio requires `esp-hal` feature `unstable`.** Wi-Fi, BLE, ESP-NOW, and IEEE 802.15.4 use crates.io `esp-radio` `=1.0.0-beta.1` with crates.io `esp-hal` `=1.2.1`. The crate is an unstable beta. Document that honestly in every radio example README. Do not pretend those APIs are frozen. Do not git-pin HP radio crates at tag `esp-hal-v1.2.1` (that tree still versions `esp-radio` as beta.0). Do not use yanked `esp-radio` 1.0.0-beta.2. (Sep 2026, pin moved Oct 2026)

- **Tear down `esp-radio` before deep sleep.** Stop advertising / STA / ESP-NOW and deinitialize the radio stack before entering deep sleep. Skipping teardown leaves the radio powered and the sleep current in milliamps instead of microamps. (Sep 2026)

- **Load the LP-core image before the HP core sleeps.** The LP core cannot fetch its image from flash after the HP core has powered down the flash. Program LP RAM first, then enter HP deep sleep. (Sep 2026)

- **Use `#[esp_hal::ram(persistent)]` (or the equivalent `#[ram(unstable(persistent))]` form on the pinned crate) for boot counters and other state that must survive deep sleep.** There is no sdkconfig RTC-memory option. Types must be safe to persist (bit-pattern / zeroable as required by the attribute). Consider a checksum; a reset that races initialization can leave random bytes in the static. (Sep 2026)

- **Chip ≠ board.** The SoC is Cargo chip feature `esp32c6` (v1 only). The board is pins/USB/LED via `board-specs/` and `board-devkitc-c6` / `board-xiao-c6`. rust-on-metal is **not** ESP-IDF `idf.py set-target`: an S3 or classic ESP32 spec (touch, ULP, Xtensa, `espup`) must not be copied onto C6. Expanding the hardware bar means a new chip feature, a `rust-toolchain.md` row, and new example specs — not a flag flip. Multi-board on the **same** chip is allowed. (Sep 2026)

- **Do not emit or "fix" sdkconfig.** This repository has no `sdkconfig`, `Kconfig.projbuild`, or `idf.py`. Chip, board, and optional behaviour are Cargo features plus generated `src/board.rs`. (Sep 2026)

- **Every firmware image must emit an ESP-IDF-compatible application descriptor.** Current `espflash` refuses to flash without it (`ESP-IDF App Descriptor missing`). Depend on crates.io `esp-bootloader-esp-idf` `=0.6.0` with feature `esp32c6`, and invoke `esp_bootloader_esp_idf::esp_app_desc!()` in `src/main.rs`. This crate is **not** ESP-IDF: it only writes image metadata the 2nd-stage bootloader and `espflash` already expect. Omitting it looks like a flasher/port problem until the descriptor is present. (Sep 2026, radio pin moved Oct 2026)

- **The stock `espflash` bootloader does not auto-rollback OTA.** Slot changes go through `esp-bootloader-esp-idf`'s OTA updater (write the inactive app slot, update `otadata`). Do not spec `CONFIG_BOOTLOADER_APP_ROLLBACK_ENABLE`, `esp_https_ota`, or `esp_ota_mark_app_valid_cancel_rollback`. A CRC failure must not activate the new slot. Setting the image state is still useful for a rollback-capable bootloader, but the default flasher will not revert a bad image by itself. (Oct 2026)

- **Do not restore retired S3 / ESP-IDF behavior.** `secure-ota-https` is an HTTPS slot flip on C6, not Secure Boot enrollment and not a three-LED S3 demo. `esp-now-low-power-mesh` is a two-role unicast pair, not a Kconfig mesh. `lp-core-wakeup` is C6 LP-core GPIO 0–7, not capacitive touch. Raw IEEE 802.15.4 is example `ieee802154-raw-frames` (node/peer data frames). It is not Thread, not Zigbee, and not a mesh. Thread is `thread-udp-pair`. Matter over Wi-Fi is `matter-wifi-onoff`. Do not fold either into the raw frame example. The directory names are historical labels for that behavior. A catalog rename is allowed when `specs/showcase-master-spec.md`, the example path, and this lesson change together. A rename does not change the behavior in this paragraph. (Oct 2026, narrowed Oct 2026)

- **Thread leader builds OpenThread from C.** crates.io `openthread` 0.4.0 ships a prebuilt MTD archive for `riscv32imac-unknown-none-elf` when the default `matter` feature set is unchanged. Feature `ftd` is not that profile, so `openthread-sys` runs CMake. The leader role needs Clang, CMake, and Ninja on the host. The example `build.rs` only emits `linkall.x`. Link `tinyrlibc` for the C `str*` symbols. (Oct 2026)

- **Matter over Wi-Fi uses git `rs-matter-embassy`, not ESP-IDF and not Matter over Thread.** crates.io `rs-matter-embassy` 0.0.0 is empty. Pin rev `2a806b0b1b521293b34a773618499e87402c843c`. Enable `esp-radio` features `wifi`, `ble`, and `coex`. Leave `ieee802154` off. Test attestation credentials only. A RAM key store means the device is commissioned again after every reset. The image can exceed the 1 MB factory slot; a larger single-app partition must still fit the XIAO's 4 MB flash. (Oct 2026)

- **The board catalog is ESP32-C6 only** (DevKitC-C6 and XIAO C6). Do not restore HUZZAH32, XIAO ESP32S3, XIAO ESP32-C5, the YEJMKJ S3 DevKit, or an empty LilyGo folder as hardware reference. Other SoCs stay as rows in `rust-toolchain.md` so generators refuse them. They do not get board-specs. (Oct 2026)

- **Do not restore ESP-IDF showcase fill-ins.** `templates/BoardSpec.template.md` examples are C6 only: module `ESP32-C6-WROOM-1` or a bare SoC, form factor DevKitC or XIAO, the onboard LDO (not the HUZZAH32 `AP2112`), PSRAM `None`, and active current from the C6 board spec. The Xtensa ~68 mA Wi-Fi figure is forbidden. The board-spec skill extracts I2C, SPI, UART, PWM, and ADC1. The C6 has no DAC and no capacitive touch. Call the low-power core the LP core. Do not say it runs ULP programs. A crate whose FunctionalSpec does not require LP-core firmware gets a linker-only `build.rs`. `dual-ota` keeps the bootloader `nvs` and `phy_init` rows and has no extra application-NVS `storage` partition. The documentation-generator title is this repository, not "ESP32 SDD Showcase", and it does not say "100% AI-first". (Oct 2026)

## Required Workflow When Something Goes Wrong
1. Read this file.
2. Check `shared-specs/coding-conventions.md`, `shared-specs/cargo-conventions.md`, `shared-specs/rust-toolchain.md`, and the relevant board-spec.
3. Improve the FunctionalSpec.md, CodingSpec.md, or TestSpec.md.
4. Regenerate with the full-project-generator skill.
5. Never manually edit generated code.

## CodingSpec.md Must Be High-Level Guidance (Not Implementation Instructions)

CodingSpec.md describes **what** and **why**. It should describe architecture, constraints, preferred crates and the reasoning behind them, non-functional requirements, trade-off decisions, and known gotchas.

Name the Rust constraints a generator gets wrong when they are missing. That includes the Embassy entry `#[esp_rtos::main]`, exact crates.io pins, radio teardown order, and partition offsets. Those are constraints.

Do not paste implementation bodies, full listings, `const` items that duplicate `src/`, or a numbered sequence of API calls. A low-level spec just duplicates code in prose and locks the implementation to the current generation's idioms.

The guiding principle: "The model we use today is the worst model we will ever use." A high-level spec benefits from better future models. (Feb 2026, narrowed Oct 2026)

## Template for Adding New Lessons
When you discover something important:
- Add it here under "Key Lessons Learned"
- Date it
- Make it actionable for future generations

This file is the living quality guardrail and institutional memory of the entire rust-on-metal ESP32 SDD showcase.
