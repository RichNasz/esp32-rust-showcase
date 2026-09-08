<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from specs/ using esp32-sdd-documentation-generator skill
     Date: 2026-03-08 | Agent: Claude Code
     ================================================ -->

# Board Spec — <!-- FILL: Vendor Board/Module Name -->

## Module / SoC

| Item | Value |
| --- | --- |
| SoC | <!-- FILL: e.g. ESP32-C6 (RISC-V HP + LP cores, up to 160 MHz) --> |
| Module | <!-- FILL: e.g. ESP32-WROOM-32 , or "Bare SoC (no module)" --> |
| Flash | <!-- FILL: e.g. 8 MB SPI NOR (quad-SPI) --> |
| PSRAM | <!-- FILL: e.g. 8 MB OPI PSRAM, or "None" --> |
| Antenna | <!-- FILL: e.g. PCB trace / ceramic / U.FL connector --> |
| Form factor | <!-- FILL: e.g. Feather, XIAO, DevKit, or module dimensions --> |
| Certification | <!-- FILL: e.g. FCC / CE, or "—" if unknown --> |

## Power Management

| Item | Detail |
| --- | --- |
| 3V3 rail | <!-- FILL: e.g. 500 mA max from AP2112 LDO --> |
| USB input | <!-- FILL: e.g. USB-C, 5 V --> |
| Battery input | <!-- FILL: e.g. JST 1.25 mm LiPo connector with onboard charger, or "None" --> |
| Deep sleep current | <!-- FILL: e.g. ~14 µA typical with all peripherals off --> |
| Active current | <!-- FILL: e.g. ~68 mA during Wi-Fi transmission --> |

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

<!-- FILL: List which ADC banks are available and any restrictions during Wi-Fi operation.
Example: ADC2 is disabled when Wi-Fi is active. ADC1 channels are always available. -->

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
- PSRAM size and use cases (ML buffers, JPEG frames, audio DSP)
- Antenna options and range trade-offs
- Built-in LiPo charger
- SoC vector extensions (S3 AIE for TFLite)
- ADC1-safe pads during Wi-Fi
- Form factor advantages (size, breadboard compatibility)
-->

- <!-- FILL -->

## Cargo chip / board features

v1 firmware generation consumes **ESP32-C6** only. Other SoCs may still be documented here as hardware reference — mark them **not in v1**.

| Item | Value |
| --- | --- |
| SoC | <!-- FILL: ESP32-C6 or other; non-C6 = not in v1 --> |
| Chip Cargo feature | <!-- FILL: `esp32c6` or "not in v1" --> |
| Board Cargo feature | <!-- FILL: `board-devkitc-c6` / `board-xiao-c6` / none --> |
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
