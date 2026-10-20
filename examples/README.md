<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# examples/

Eleven rust-on-metal ESP32-C6 examples, ordered so you can start with `blinky`. As of 2026-10-20, **eleven crates are generated** and **zero examples are specs only**. Every folder below contains a generated crate. The locked directory tree below is the required base, not the whole layout on disk. Extra files appear only when that example's FunctionalSpec requires them. **Firmware generated** means the crate is in the tree. It is not a hardware test result. Thread is not a border router. Matter over Wi-Fi uses test credentials. It is not Matter over Thread and not a certified product.

A **spec** is Markdown a person writes. The agent generates the crate from `examples/<name>/specs/`. The lesson is [Using an AI coding agent](../docs/guides/using-with-ai-agents.md).

**Chip ≠ board.** v1 chip is ESP32-C6 (`esp32c6`) only. That is not `idf.py set-target`. **Required board:** ESP32-C6-DevKitC-1-N8 (`board-devkitc-c6`). XIAO ESP32-C6 (`board-xiao-c6`) is optional.

## Learning progression

| # | Example | What you can observe | Status |
|---|---|---|---|
| 1 | [blinky](blinky/) | Breathing LED, multi-board (ESP32-C6-DevKitC-1-N8 + XIAO ESP32-C6) | Firmware generated |
| 2 | [deep-sleep-timer-node](deep-sleep-timer-node/) | RTC wakeup + persistence | Firmware generated |
| 3 | [deep-sleep-bme280-mqtt-sensor](deep-sleep-bme280-mqtt-sensor/) | BME280 readings over Wi-Fi MQTT, then deep sleep | Firmware generated |
| 4 | [hardware-aes-secure-storage](hardware-aes-secure-storage/) | Hardware AES and flash-backed storage (plaintext key demo) | Firmware generated |
| 5 | [ble-beacon-deep-sleep](ble-beacon-deep-sleep/) | Duty-cycled BLE beacon (demo window, not a multi-year budget) | Firmware generated |
| 6 | [secure-ota-https](secure-ota-https/) | HTTPS download into the inactive OTA slot, CRC check, reboot. Not Secure Boot or flash-encryption enrollment | Firmware generated |
| 7 | [esp-now-low-power-mesh](esp-now-low-power-mesh/) | ESP-NOW sensor-to-gateway unicast. The directory name is historical. This is not a multi-hop mesh | Firmware generated |
| 8 | [lp-core-wakeup](lp-core-wakeup/) | LP-core GPIO wakeup on RTC GPIO 0–7 (C6 has no capacitive touch) | Firmware generated |
| 9 | [ieee802154-raw-frames](ieee802154-raw-frames/) | Raw IEEE 802.15.4 data frames between a node and a peer. Not Thread, not Zigbee, not a mesh | Firmware generated |
| 10 | [thread-udp-pair](thread-udp-pair/) | Two-node Thread lab network. A leader and a child exchange one UDP payload. Not a border router and not Zigbee | Firmware generated |
| 11 | [matter-wifi-onoff](matter-wifi-onoff/) | Matter on/off light over Wi-Fi, commissioned over BLE with test credentials. Not Matter over Thread and not a certified product | Firmware generated |

## Example structure

Every example matches [`specs/example-project-structure-spec.md`](../specs/example-project-structure-spec.md):

```
examples/<kebab-name>/
├── specs/                          ← human-written
│   ├── FunctionalSpec.md           ← what you can observe
│   ├── CodingSpec.md               ← architecture and constraints, no code
│   └── TestSpec.md                 ← how to check
├── src/                            ← agent-written
│   └── main.rs                     ← #![no_std] #![no_main]; extra modules as needed
├── Cargo.toml                      ← agent-written
├── build.rs                        ← agent-written
├── .cargo/config.toml              ← agent-written; target = riscv32imac-unknown-none-elf, runner = espflash
├── README.md                       ← agent-written
├── .gitignore                      ← agent-written
└── target/                         ← gitignored
```

Examples are standalone Cargo packages. There is no root workspace, no `sdkconfig`, no `main.c`, and no `CMakeLists.txt`.

These paths are allowed only when the FunctionalSpec requires them:

- `src/board.rs` and other `src/<subsystem>.rs` modules
- `partitions.csv` and `espflash.toml` for a custom or OTA partition table
- `certs/` for an HTTPS OTA CA PEM. No private key in that directory
- nested `lp-core/` when the spec requires LP-core firmware
- `Cargo.lock`

Board selection uses Cargo features: default `board-devkitc-c6`, optional `board-xiao-c6`. Chip feature `esp32c6` only. Generated `src/board.rs` holds the pins. Do not hardcode pins in `main.rs`.

## Building blinky

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

In VS Code or Cursor, use **File → Open Folder…** and open `examples/blinky/` only, so rust-analyzer uses that crate.

## Adding an example

Edit `specs/showcase-master-spec.md`, then ask the agent to draft `FunctionalSpec.md`. The steps, and what each spec file is for, are in [Create a new example](../docs/guides/04-new-example.md). Read [Using an AI coding agent](../docs/guides/using-with-ai-agents.md) first if you have not used specs as the source of truth.
