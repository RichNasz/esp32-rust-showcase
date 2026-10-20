<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# examples/

Eight progressive rust-on-metal ESP32-C6 examples. They are ordered by complexity — start with `blinky` and work forward. As of 2026-10-20, **all eight have generated crates**. `specs/showcase-master-spec.md` still lists `secure-ota-https`, `esp-now-low-power-mesh`, and `lp-core-wakeup` as Specs only; humans own that catalog. A specs-only folder contains `specs/` and a README until `esp32-sdd-full-project-generator` runs. The tree below is the target layout. `lp-core-wakeup` also has the allowed nested `lp-core/` crate.

**Chip ≠ board.** v1 chip is ESP32-C6 (`esp32c6`) only — not `idf.py set-target` portability. **Required board:** ESP32-C6-DevKitC-1-N8. XIAO ESP32-C6 is also supported.

## Learning Progression

Start at example 1 and work forward — each example introduces new hardware features and builds on patterns from the previous one.

| # | Example | What You Learn | Key ESP32-C6 Feature | Status | Complexity |
|---|---|---|---|---|---|
| 1 | [blinky](blinky/) | SDD pipeline, multi-board Cargo features, WS2812 vs GPIO LED | RMT / GPIO LED | Firmware generated | ★☆☆☆☆ |
| 2 | [deep-sleep-timer-node](deep-sleep-timer-node/) | Deep sleep, RTC timer wakeup, persistent RAM | RTC sleep + `#[esp_hal::ram(persistent)]` | Firmware generated | ★★☆☆☆ |
| 3 | [deep-sleep-bme280-mqtt-sensor](deep-sleep-bme280-mqtt-sensor/) | I2C sensor, Wi-Fi STA, MQTT publish, duty-cycled sleep | I2C + `esp-radio` Wi-Fi + MQTT | Firmware generated | ★★★☆☆ |
| 4 | [hardware-aes-secure-storage](hardware-aes-secure-storage/) | Hardware AES, flash-backed blobs (plaintext key) | `esp-hal` AES + `esp-storage` | Firmware generated | ★★★☆☆ |
| 5 | [ble-beacon-deep-sleep](ble-beacon-deep-sleep/) | BLE advertising, radio teardown, demo duty cycle | `esp-radio` BLE + deep sleep | Firmware generated | ★★★★☆ |
| 6 | [secure-ota-https](secure-ota-https/) | HTTPS slot flip, TLS 1.3, CRC before activate. Not Secure Boot | Dual OTA partitions + `embedded-tls` | Firmware generated | ★★★★☆ |
| 7 | [esp-now-low-power-mesh](esp-now-low-power-mesh/) | ESP-NOW unicast pair (directory name is historical, not a mesh) | `esp-radio` ESP-NOW + sensor deep sleep | Firmware generated | ★★★★☆ |
| 8 | [lp-core-wakeup](lp-core-wakeup/) | LP-core firmware, GPIO 0 wakeup from HP deep sleep | `esp-lp-hal` + GPIO 0–7 | Firmware generated | ★★★★★ |

## Example Structure

Every example folder follows this exact, permanently locked layout (see `specs/example-project-structure-spec.md`):

```
examples/<kebab-name>/
├── specs/                          ← human-only
│   ├── FunctionalSpec.md
│   ├── CodingSpec.md
│   └── TestSpec.md
├── src/                            ← 100% agent-generated
│   └── main.rs                     ← #![no_std] #![no_main]; extra modules as needed
├── Cargo.toml
├── build.rs
├── .cargo/config.toml              ← target = riscv32imac-unknown-none-elf, runner = espflash
├── README.md
├── .gitignore
└── target/                         ← gitignored
```

Examples are **standalone Cargo packages**, not members of a repository-level workspace. There is no root `Cargo.toml` workspace, no `sdkconfig`, no `main.c`, and no `CMakeLists.txt`.

Board selection is via mutually exclusive Cargo features: default `board-devkitc-c6`, optional `board-xiao-c6`. Chip feature `esp32c6` only in v1. Generated `src/board.rs` holds GPIO/LED/button constants derived from `board-specs/` — never hardcode pins in `main.rs`.

## Building an Example

### Command line

```sh
cd examples/blinky
cargo build --release
cargo clippy -- -D warnings
cargo run --release
```

Optional XIAO ESP32-C6:

```sh
cargo run --release --no-default-features --features board-xiao-c6
```

### VS Code / Cursor

Each example folder is a **complete, standalone Cargo package**. For the best experience:

1. **File → Open Folder…** and select the specific example folder (e.g. `examples/blinky/`)
2. rust-analyzer uses that crate's `Cargo.toml` and `.cargo/config.toml`
3. Build/flash: `cargo run --release`

> Do **not** open the repository root as your active workspace when developing an example.

See [docs/guides/using-with-ai-agents.md](../docs/guides/using-with-ai-agents.md) for full IDE setup details.

## Generating a New Example

1. Create the example folder and its `specs/` subdirectory.
   > Need to add a new board first? See [docs/guides/creating-new-board-specs.md](../docs/guides/creating-new-board-specs.md).
2. Create spec files (say: **"Activate esp32-sdd-functional-spec-creator skill"**) — output goes in `specs/FunctionalSpec.md`.
3. Add `specs/CodingSpec.md`.
4. Say: **"Activate esp32-sdd-full-project-generator skill"**

> `specs/FunctionalSpec.md`, `specs/CodingSpec.md`, and `specs/TestSpec.md` are the only files humans write. Everything else is agent-generated and must not be edited by hand.
