---
name: esp32-sdd-full-project-generator
description: The primary SDD code-generation skill. Reads FunctionalSpec.md + CodingSpec.md and emits a complete, buildable no_std rust-on-metal Cargo crate (esp-generate style) for ESP32-C6 — src/main.rs, src/board.rs, Cargo.toml, build.rs, .cargo/config.toml, and a generated README.
version: "2.0.0"
author: Richard Naszcyniec
keywords: sdd, esp32, project-generator, rust, esp-hal, no_std, cargo, codegen, full-project
---

<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     ================================================ -->

# ESP32 SDD Full Project Generator

## When to use
After `FunctionalSpec.md` and `CodingSpec.md` exist for an example. This is the primary code-generation step — it produces a complete, buildable `no_std` `esp-hal` crate from spec inputs alone.

Do **not** emit ESP-IDF projects. Do **not** emit `main.c`, `CMakeLists.txt`, `sdkconfig`, or `sdkconfig.defaults`.

## Mandatory Directory Layout (PERMANENT — NEVER DEVIATE)

Every generated example **must** match this exact structure, defined in `specs/example-project-structure-spec.md`:

```
examples/<kebab-name>/
├── specs/                          ← Human-only (FunctionalSpec.md + CodingSpec.md + TestSpec.md)
│   ├── FunctionalSpec.md
│   ├── CodingSpec.md
│   └── TestSpec.md                 ← Human-written testing requirements
├── src/                            ← 100% agent-generated
│   ├── main.rs                     ← #![no_std] #![no_main]; // AGENT-GENERATED on line 1
│   └── board.rs                    ← GPIO / LED / button from board-specs (never hardcode pins in main.rs)
├── Cargo.toml
├── build.rs
├── .cargo/config.toml              ← target = riscv32imac-unknown-none-elf, runner = espflash flash --monitor
├── README.md                       ← Generated (includes Testing section from TestSpec.md)
├── .gitignore                      ← Generated (must exclude target/ and binaries)
└── target/                         ← gitignored
```

Examples are **standalone Cargo packages**, not workspace members. There is no root `Cargo.toml` workspace.

Do not create files outside this layout. Do not place spec files at the example root — they belong in `specs/`. Do not add extra top-level files unless they are in this list or the allowed extras below.

**Allowed extras** (only when the FunctionalSpec requires them):

- `src/<subsystem>.rs` (or `src/<subsystem>/mod.rs`) for additional modules
- `partitions.csv` and/or `espflash.toml` for custom / OTA partition tables (`shared-specs/partition-table-profiles.md`)
- `certs/` for an HTTPS OTA trust anchor (a CA PEM). No private key in that directory
- nested `lp-core/` crate directory when the FunctionalSpec requires LP-core firmware
- `Cargo.lock` (binary crate lockfile)

**Forbidden** (FAIL the generation if you emit any of these): `main.c`, `CMakeLists.txt`, `sdkconfig`, `sdkconfig.defaults`, `Kconfig.projbuild`, `idf_component.yml`, `main/` as an ESP-IDF component, `rust-toolchain.toml` (channel lives in `shared-specs/rust-toolchain.md`, not per example).

Seed crate files from `templates/main.rs.template`, `templates/Cargo.toml.template`, `templates/build.rs.template`, `templates/.cargo/config.toml.template`, `templates/board.rs.template`, and `templates/gitignore.template`. Copy crate pins from `shared-specs/rust-toolchain.md` — do not invent versions.

Do **not** seed, write, or overwrite `specs/FunctionalSpec.md` (or any file under `specs/`). Only `esp32-sdd-functional-spec-creator` writes `FunctionalSpec.md`. This generator **reads** existing specs as inputs. `templates/FunctionalSpec.template.md` is not a seed for this skill.

## Radio pin

Copy the crate table and **Radio pin** section of `shared-specs/rust-toolchain.md`. Do not restate a shorter pin list here or in a domain skill.

That table is the only version list. It already requires `esp-bootloader-esp-idf` and `esp_app_desc!()` on every example. Radio examples use the crates.io exact pin in that file (`esp-hal` `=1.2.1`, `esp-radio` `=1.0.0-beta.1`, `esp-rtos` `=0.4.0`, `esp-alloc` `=0.11.0` when the CodingSpec justifies a heap, and `esp-storage` `=0.10.0` when the example writes flash). Do not git-pin those HP crates. The nested LP crate still git-pins `esp-lp-hal`. Do not mix a git `esp-hal` into an HP radio crate.

Enable `esp-hal` feature `unstable` in addition to `esp32c6` when the example uses radio or sleep / RTC APIs that require it. Document that in the README. Radio READMEs must say `esp-radio` is an unstable beta.

## ESP-NOW unicast

When the FunctionalSpec is ESP-NOW (the directory may still be named `esp-now-low-power-mesh`):

- Generate one sensor-to-gateway unicast pair. The historical directory name is not a mesh. Do not add a routing table, a relay, or an AP scan.
- Two compile-time roles when the spec says so. The gateway stays awake. The sensor sends one datagram, tears the radio down, then hands sleep to `esp32-deep-sleep-engineer`.
- Copy the radio pin. Enable `esp-radio` features `esp-now` and the Wi-Fi driver feature that pin requires. Do not enable `embassy-net`. Do not associate to an AP.
- Keep the payload in one ESP-NOW frame (250 bytes maximum) unless the spec sets a smaller limit.

## Raw IEEE 802.15.4

When the FunctionalSpec is raw IEEE 802.15.4:

- Generate node/peer data frames. Do not add Thread, Zigbee, or a mesh.
- Copy the radio pin. Enable `esp-radio` feature `ieee802154`. Do not enable `wifi`: this driver does not coexist with Wi-Fi (`shared-specs/cargo-conventions.md`).
- MAC header types come from the crates.io `ieee802154` version named in the CodingSpec, with default features disabled.
- The catalog example does not sleep. Do not add a sleep path unless the spec does.
- Do not add Thread, Zigbee, or Matter crates to this example. Thread is `thread-udp-pair`. Matter over Wi-Fi is `matter-wifi-onoff`.

## Thread UDP pair

When the FunctionalSpec is a two-role Thread lab link (`thread-udp-pair`):

- One image. Mutually exclusive `role-leader` and `role-child`. Default is `role-child`. Both roles, or neither, fail at compile time.
- Copy the radio pin and the Thread pin in `rust-toolchain.md`. Enable `esp-radio` feature `ieee802154`. Do not enable `wifi`.
- `openthread` 0.4.0 features `esp-radio`, `isupper`, `log`, and `udp`. The leader feature also enables `openthread/ftd`. The child does not, so it links the prebuilt MTD archive. `ftd` makes `openthread-sys` compile OpenThread; the host then needs Clang, CMake, and Ninja. This crate's `build.rs` does not invoke CMake. It still emits `-Tlinkall.x` last.
- Link `tinyrlibc` 0.5 with default features off and features `utoa` and `strtoul`.
- Both roles load the same `THREAD_DATASET` lab TLV. RAM settings. Keep the TRNG source alive. No border router, no Matter, no Zigbee, no CSL, no deep sleep.

## Matter over Wi-Fi

When the FunctionalSpec is a Matter on/off light over Wi-Fi (`matter-wifi-onoff`):

- One role. Copy the radio pin and the Matter pin in `rust-toolchain.md`. Enable `esp-radio` features `wifi`, `ble`, and `coex`. Do not enable `ieee802154`.
- Git `rs-matter-embassy` at the rev in that pin, default features off, features `esp`, `log`, `trouble`, `embassy-net`, and `rustcrypto`. Do not use crates.io `rs-matter-embassy` 0.0.0. Do not use ESP-IDF or C++ `esp-matter`.
- Link `tinyrlibc` 0.5 with default features off and features `memchr`, `utoa`, and `strtoul`.
- Concurrent BLE and Wi-Fi commissioning. Test attestation only. RAM key store. Log the manual pairing code and the QR payload from the test commissioning data. The LED follows the on/off attribute. No Matter OTA and no Thread.
- If the linked app exceeds the 1 MB factory slot, emit one single-app `partitions.csv` that fits in 4 MB and point the espflash runner at it.

## HTTPS OTA (CRC slot flip)

When the FunctionalSpec downloads an image over HTTPS and activates the inactive slot after a CRC check:

- Do **not** invoke `esp32-hardware-crypto-configurer`. CRC is not AES, not a signature, and not Secure Boot. Follow `shared-specs/security-baseline.md` and `shared-specs/AIGenLessonsLearned.md`.
- Use partition profile `dual-ota` (`shared-specs/partition-table-profiles.md`). Copy the radio pin, including crates.io `esp-storage` `=0.10.0`.
- TLS is `embedded-tls` from the `rust-toolchain.md` table, with default `std` disabled. Verify the server certificate. Embed the CA PEM from `certs/`. Ship a placeholder CA that does not authenticate a public server. Refuse a PEM that contains a private key. Do not commit passwords or put them in `src/`.
- Activate the inactive slot only after length and IEEE CRC32 match. A CRC failure leaves the current boot selection in place. Stock `espflash` does not auto-rollback. Do not emit `CONFIG_BOOTLOADER_APP_ROLLBACK_ENABLE`, `esp_https_ota`, or `esp_ota_mark_app_valid_cancel_rollback`.

## Workflow (follow exactly)
MANDATORY: First read and follow `shared-specs/AIGenLessonsLearned.md` before doing anything else.

1. Read `examples/<name>/specs/FunctionalSpec.md`, `examples/<name>/specs/CodingSpec.md`, and `examples/<name>/specs/TestSpec.md` (if it exists).
   - CodingSpec.md is high-level architectural guidance (constraints, preferred crates, trade-offs, gotchas). It does not prescribe implementation — use your best judgement for all code-level decisions not covered by the spec.
2. Read the relevant `board-specs/<vendor>/<board>.md` file identified in the FunctionalSpec. v1 boards: Espressif ESP32-C6-DevKitC-1-N8 (default, feature `board-devkitc-c6`) and Seeed XIAO ESP32-C6 (feature `board-xiao-c6`).
3. Read any `shared-specs/` files referenced in either spec (`coding-conventions.md`, `cargo-conventions.md`, `rust-toolchain.md`, plus domain specs as needed).
4. Generate the full crate using the mandatory layout above:
   - `Cargo.toml` — package `name` is the kebab folder with hyphens → underscores; `edition = "2024"`; `publish = false`; license MIT; feature `esp32c6`; mutually exclusive board features `board-devkitc-c6` (default) and `board-xiao-c6`. Copy dependency pins from `rust-toolchain.md` / `cargo-conventions.md`.
   - `src/main.rs` — line 1 `// AGENT-GENERATED — DO NOT EDIT BY HAND`, then `#![no_std]` `#![no_main]`. Invoke `esp_bootloader_esp_idf::esp_app_desc!()` in this file (required; current `espflash` refuses to flash without the ESP-IDF-compatible application descriptor). Embassy entry is `#[esp_rtos::main] async fn main(spawner: Spawner) -> !`. Do **not** use `#[esp_hal::main]` for Embassy — `esp-hal` 1.2.1 `#[esp_hal::main]` is blocking `fn main() -> !`. After `esp_hal::init`, call `esp_rtos::start` on TIMG0 + `FROM_CPU_INTR0` on C6. Top-level wiring only.
   - `src/board.rs` — generated from board-specs. `compile_error!` if both board features are enabled or if neither is enabled. Application code must not hardcode pin numbers.
   - `build.rs` — linker / `esp-hal` build-time config; embed the LP-core image when applicable. Do not shell out to CMake. Emit `println!("cargo:rustc-link-arg=-Tlinkall.x");` **last** so the crate links (`linkall.x` must be the last linker script).
   - `.cargo/config.toml` — `target = "riscv32imac-unknown-none-elf"`, `runner = "espflash flash --monitor"`, `build-std = ["core"]` (add `"alloc"` only when the CodingSpec justifies a heap).
   - `.gitignore` — must exclude `target/`, `*.bin`, `*.elf`, `*.map`.
5. Invoke domain skills as sub-workflows when the FunctionalSpec requires them:
   - `esp32-deep-sleep-engineer` — deep sleep / RTC wakeup / persistent RAM
   - `esp32-ble-beacon-engineer` — BLE advertising, beaconing, or manufacturer data
   - `esp32-ws2812-led-engineer` — any referenced board-spec lists a WS2812 / SK6812 LED
   - `esp32-hardware-crypto-configurer` — hardware AES or flash-backed keys. A Security section, a CRC, or HTTPS OTA does not qualify
   - `esp32-lp-core-engineer` — LP-core firmware or LP-core wakeup (`lp-core-wakeup` and any spec that names the LP core)
   - ESP-NOW unicast, raw IEEE 802.15.4, Thread UDP pair, Matter over Wi-Fi, and HTTPS CRC slot-flip — the sections above. They are steps of this skill, not separate skills
6. Verify internal consistency: pin assignments match the board spec via `src/board.rs`, Cargo features match the chip/board, radio crates match the `rust-toolchain.md` radio pin (including `esp-bootloader-esp-idf`), all FunctionalSpec features are accounted for. Zero `unwrap()` / `expect()` on fallible HAL / radio in production paths (`coding-conventions.md`).
7. Generate `examples/<name>/README.md` with agent-generated header and all of the following sections in order:
   - **Overview** — one-paragraph summary of what the example demonstrates
   - **Prerequisites** — rustup stable (MSRV 1.95.0), `rust-src`, target `riscv32imac-unknown-none-elf`, `espflash` / `cargo-espflash`, ESP32-C6-DevKitC-1-N8 or XIAO ESP32-C6. Do **not** list ESP-IDF, `espup`, or the `esp` rustup channel.
   - **Build & Flash** — `cargo build --release`, `cargo clippy -- -D warnings`, `cargo run --release`, and the XIAO variant `cargo run --release --no-default-features --features board-xiao-c6`
   - **Opening in VS Code / Cursor** — the following exact subsection must appear in every generated README:

     ```
     ### Opening in VS Code / Cursor

     This folder is a complete standalone Cargo package. Open **only this folder**
     (not the repository root) so rust-analyzer uses this crate.

     **File → Open Folder… → select `examples/<name>/`**

     This gives rust-analyzer the crate's `Cargo.toml` and `.cargo/config.toml`
     (RISC-V target + `espflash` runner). Do not open the top-level repository
     folder as your active workspace while developing this example.
     ```

   - **Expected Serial Output** — representative log lines showing successful operation (`esp-println` / `log`)
   - **Key Concepts** — bullet list of the main `esp-hal` / Embassy / domain crates and patterns demonstrated
   - **Limitations** — include only if FunctionalSpec has a Limitations section (`documentation-generation-spec.md`)
   - **Testing** — generated from `specs/TestSpec.md` if present; omit section entirely if TestSpec.md is absent. Automated tests first (`cargo build --release`, `cargo clippy -- -D warnings`, `cargo run --release` / Wokwi), manual tests (labelled "Manual — hardware required") only for steps that cannot be automated. Numbered, beginner-friendly steps.
   - **Spec Files** — links to `specs/FunctionalSpec.md`, `specs/CodingSpec.md`, and `specs/TestSpec.md` (if present)

   If the example enables `esp-hal` `unstable` (radio or some sleep / RTC APIs), the README **must** state those APIs are unstable. Radio example READMEs must say `esp-radio` is an unstable beta.

8. Output: "Project generated. Build with `cargo run --release` from `examples/<name>/`."
