<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-10-20 | Agent: Grok
     ================================================ -->

# Board Spec — <!-- FILL: Vendor Board/Module Name -->

## Module / SoC

| Item | Value |
| --- | --- |
| SoC | <!-- FILL: e.g. ESP32-C6 (RISC-V HP + LP cores, up to 160 MHz) --> |
| Module | <!-- FILL: e.g. ESP32-C6-WROOM-1, or "Bare SoC (no module)" --> |
| Flash | <!-- FILL: e.g. 8 MB SPI NOR (quad-SPI) on DevKitC-C6, 4 MB on XIAO C6 --> |
| PSRAM | <!-- FILL: None. v1 C6 boards have no PSRAM. Do not write an S3 OPI size. --> |
| Antenna | <!-- FILL: e.g. PCB trace / ceramic / U.FL connector --> |
| Form factor | <!-- FILL: e.g. DevKitC DIP or XIAO 21 × 17.8 mm. Not Feather. --> |
| Certification | <!-- FILL: e.g. FCC / CE, or "—" if unknown --> |

## Power Management

| Item | Detail |
| --- | --- |
| 3V3 rail | <!-- FILL: e.g. ~500 mA from the onboard LDO. Name the part from the schematic. Not the HUZZAH32 AP2112. --> |
| USB input | <!-- FILL: e.g. USB-C, 5 V --> |
| Battery input | <!-- FILL: e.g. JST 1.25 mm LiPo connector with onboard charger, or "None" --> |
| Deep sleep current | <!-- FILL: e.g. DevKitC ~7–15 µA, XIAO ~15 µA. Use the vendor figure. --> |
| Active current | <!-- FILL: e.g. CPU-only ~25 mA, or the C6 datasheet Wi-Fi TX figure. Do not copy the Xtensa ~68 mA number. --> |

> **Power note for SDD projects**: <!-- FILL: Note whether deep sleep is viable for battery operation, reference the esp32-deep-sleep-engineer skill if so. -->

## Onboard Peripherals

| Item | GPIO | Polarity | Notes |
| --- | --- | --- | --- |
| <!-- FILL: e.g. User LED --> | <!-- FILL: GPIO N --> | <!-- FILL: Active HIGH / Active LOW --> | <!-- FILL: Simple GPIO vs WS2812. WS2812: GRB, 800 kHz NRZ, esp-hal RMT, brightness cap 64/255 — a GPIO level produces no visible output. Active-LOW: invert in generated src/board.rs --> |
| <!-- FILL: e.g. Boot button --> | <!-- FILL: GPIO N --> | <!-- FILL: Active LOW --> | <!-- FILL: also a strapping pin — do not drive at boot --> |

## GPIO & Pin Rules / Warnings

### Strapping Pins (do not drive during reset/boot)

| GPIO | Strapping function | Safe default |
| --- | --- | --- |
| <!-- FILL: GPIO N --> | <!-- FILL: e.g. Boot mode selector --> | <!-- FILL: e.g. Leave floating or pull HIGH --> |

### ADC Restrictions

<!-- FILL: ESP32-C6 has ADC1 only. There is no ADC2, so Wi-Fi does not block ADC.
Do not write an ESP32 or S3 "ADC2 is disabled during Wi-Fi" rule as this board's behavior. -->

### USB Serial / Console

<!-- FILL: Note whether the board uses a USB-UART bridge (e.g. CH343P, CP2104) or native USB.
There is no sdkconfig. Firmware logs via esp-println; flash/monitor via espflash.

Classify as exactly one of:
- UART bridge (`espflash` on the UART-C port) — name the bridge chip and TX/RX GPIOs
- USB Serial/JTAG
- USB CDC

NOTE: Always specify a language on fenced code blocks (e.g. ```toml, ```sh, ```rust). -->

### Reserved / Internal Pins (do not use in user code)

| GPIO | Reason |
| --- | --- |
| <!-- FILL: GPIO N–M --> | <!-- FILL: e.g. Internal SPI flash interface --> |

## Complete Pin Mapping Table

### User-Accessible Pads

| Label | GPIO | ADC | SPI | I2C | UART | Notes |
| --- | --- | --- | --- | --- | --- | --- |
| <!-- FILL: D0 / A0 --> | GPIO <!-- N --> | <!-- ADC1_CHn or — --> | <!-- — --> | <!-- — --> | <!-- — --> | <!-- FILL --> |

### Internal / Special GPIOs

| GPIO | Function | Driveable by user? |
| --- | --- | --- |
| <!-- FILL: GPIO N --> | <!-- FILL: e.g. Onboard LED (active LOW) --> | <!-- FILL: Yes / No --> |

## Board Advantages for SDD Projects

<!-- FILL: Bullet list of hardware features that are relevant when authoring a FunctionalSpec.md.
Typical items to consider:
- Deep sleep floor and battery life implications
- PSRAM: v1 C6 boards have none. Do not plan JPEG or ML frame buffers in on-chip PSRAM
- Antenna options and range trade-offs
- Built-in LiPo charger
- ADC1-safe pads during Wi-Fi
- Form factor advantages (size, breadboard compatibility)
-->

- <!-- FILL -->

## Cargo chip / board features

Board specs in this repository are **ESP32-C6** only. Do not fill this template for another SoC.

| Item | Value |
| --- | --- |
| SoC | ESP32-C6 |
| Chip Cargo feature | `esp32c6` |
| Board Cargo feature | <!-- FILL: `board-devkitc-c6` or `board-xiao-c6` --> |
| RTC / LP GPIOs | <!-- FILL: C6 = GPIO 0–7 only; GPIO 9 is not RTC --> |

```toml
# Example v1 C6 board: enable one board feature (default is board-devkitc-c6)
# cargo run --release
# cargo run --release --no-default-features --features board-xiao-c6
```

**Firmware notes (required on v1 C6 boards):**

- Generated `src/board.rs` is the only place pin numbers belong.
- Console: `esp-println` + `espflash`. UART-bridge ports survive deep sleep; USB Serial/JTAG and USB CDC drop.
- Sleep / RTC / radio need `esp-hal` feature `unstable`.
- `esp_bootloader_esp_idf::esp_app_desc!()` is image metadata, not an ESP-IDF install.
