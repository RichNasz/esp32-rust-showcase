<!-- ================================================
     AGENT-GENERATED — DO NOT EDIT BY HAND
     Generated from vendor documentation via esp32-board-spec-generator skill
     Source: https://wiki.seeedstudio.com/xiao_esp32c6_getting_started/
     Date: 2026-09-08 | Agent: Grok
     ================================================ -->

# Board Spec — Seeed XIAO ESP32-C6

## Module / SoC

| Item | Value |
| --- | --- |
| SoC | ESP32-C6 (dual RISC-V: 160 MHz high-performance core + 20 MHz low-power core) |
| Module | Bare SoC (no separate module; SoC is mounted directly on XIAO PCB) |
| Flash | 4 MB SPI NOR (quad-SPI) |
| PSRAM | None |
| Antenna | Onboard ceramic antenna + U.FL connector (RF switch via GPIO14; switch enabled by GPIO3) |
| Form factor | XIAO (21 × 17.8 mm), single-sided mount, breadboard-compatible |
| Certification | FCC / CE / TELEC |

## Power Management

| Item | Detail |
| --- | --- |
| 3V3 rail | ~500 mA from onboard LDO (verify exact rating in Seeed schematic) |
| USB input | USB-C, 5 V |
| Battery input | JST 1.25 mm 2-pin connector + solder pads; onboard LiPo charger (charge indicated by red LED) |
| Deep sleep current | ~15 µA (with all peripherals off, LP core stopped) |
| Light sleep current | ~3.1 mA |
| Modem sleep current | ~30 mA |
| Active / Wi-Fi TX current | ~80 mA average; ~250 mA peak during Wi-Fi 6 transmission (per ESP32-C6 datasheet) |

> **Power note for SDD projects**: The ~15 µA deep sleep floor makes this board well-suited for battery-powered applications with infrequent wakeups. The LP core (20 MHz RISC-V) can run its own program from LP RAM in light sleep while the HP core is off. Use the `esp32-deep-sleep-engineer` skill when targeting battery operation. Note: the ESP32-C6 LP core is distinct from ESP32's ULP and uses dedicated LP peripherals (LP_UART, LP_I2C, LP_GPIO 0–7).

## Onboard Peripherals

| Item | GPIO | Polarity | Notes |
| --- | --- | --- | --- |
| User LED (blue) | GPIO 15 | **Active LOW** | Simple GPIO LED (not WS2812). LED ON when GPIO15 = LOW; OFF when HIGH. Invert in generated `src/board.rs`. GPIO15 is also a strapping pin — high-impedance at boot is safe (internal pull-up holds HIGH = LED off = correct strapping default) |
| Charge indicator LED | — | — | Red LED; hardware-controlled by LiPo charger IC; not GPIO-accessible |
| Boot button | GPIO 9 | Active LOW | Pulls GPIO9 LOW at reset → serial download mode. **Not an RTC GPIO** — cannot wake from deep sleep. Use timer-only wakeup or an external button on LP GPIO 0–7 |
| Reset button | EN / CHIP_PU | — | Hardware reset only |
| RF antenna switch (enable) | GPIO 3 (LP_GPIO3) | Active HIGH | Must be HIGH to activate the RF switch IC |
| RF antenna switch (select) | GPIO 14 | — | LOW = ceramic antenna (default); HIGH = U.FL external antenna |

## GPIO & Pin Rules / Warnings

### Strapping Pins (do not drive during reset/boot)

| GPIO | Strapping function | Safe default |
| --- | --- | --- |
| GPIO 8 | JTAG bypass (LOW = disable USB JTAG; HIGH = enable) | Leave floating (internal pull-up → HIGH = USB JTAG enabled) |
| GPIO 9 | Boot mode (LOW = serial download; HIGH = SPI flash boot) | Leave floating or pull HIGH. BOOT button pulls LOW for firmware download |
| GPIO 15 | SDIO boot log output (HIGH = output log; LOW = silent) | Leave floating — also drives onboard LED; internal state is safe at reset |

### ADC Channels

The ESP32-C6 has **ADC1 only** — there is no ADC2. This means:

- ADC measurements are **never blocked by Wi-Fi activity** (unlike ESP32/ESP32-S3 where ADC2 was unavailable during Wi-Fi).
- All ADC-capable pins (GPIO 0–6) use ADC1 channels and are safe to use alongside active Wi-Fi.

| GPIO | ADC channel |
| --- | --- |
| GPIO 0 (D0) | ADC1_CH0 |
| GPIO 1 (D1) | ADC1_CH1 |
| GPIO 2 (D2) | ADC1_CH2 |
| GPIO 3 | ADC1_CH3 (also used for RF switch enable — avoid for ADC if antenna switching needed) |
| GPIO 4 (MTMS) | ADC1_CH4 |
| GPIO 5 (MTDI) | ADC1_CH5 |
| GPIO 6 (MTCK) | ADC1_CH6 |

### USB Serial / Console

The ESP32-C6 uses a **built-in USB Serial/JTAG controller** — no external USB-UART bridge chip is present. Classify this board as **USB Serial/JTAG**. There is no `sdkconfig`. Firmware logs via `esp-println`; flash/monitor via `espflash` (`cargo run --release --no-default-features --features board-xiao-c6`).

The native USB port drops when USB is powered down for deep sleep; `espflash monitor` exits on first sleep entry. For multi-cycle serial logs use the DevKitC-C6 **left** UART-bridge port instead.

Do **not** emit `CONFIG_ESP_CONSOLE_*`. This is USB Serial/JTAG (same peripheral family as ESP32-C3), not USB CDC (ESP32-S3 USB OTG).

### Antenna Switch Pins (handle with care)

| GPIO | Role | Notes |
| --- | --- | --- |
| GPIO 3 (LP_GPIO3) | RF switch enable | HIGH activates the switch IC; LOW disables it (ceramic antenna is default when switch is inactive) |
| GPIO 14 | Antenna select | LOW = ceramic (default); HIGH = U.FL external antenna |

If your application does not use the U.FL connector, leave GPIO3 and GPIO14 undriven — the ceramic antenna is active by default.

### Reserved / Internal Pins (do not use in user code)

| GPIO | Reason |
| --- | --- |
| GPIO 24–30 | Internal quad-SPI flash interface; not exposed on package pads |

## Complete Pin Mapping Table

### User-Accessible Pads (D0–D10 and JTAG header)

| Label | GPIO | ADC | SPI | I2C | UART | LP peripheral | Notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| D0 / A0 | GPIO 0 | ADC1_CH0 | — | — | — | LP_GPIO0 | General I/O; LP core accessible |
| D1 / A1 | GPIO 1 | ADC1_CH1 | — | — | — | LP_GPIO1 | General I/O; LP core accessible |
| D2 / A2 | GPIO 2 | ADC1_CH2 | — | — | — | LP_GPIO2 | General I/O; LP core accessible |
| D3 | GPIO 21 | — | — | — | — | — | Digital I/O; SDIO_DATA1 alt |
| D4 / SDA | GPIO 22 | — | — | I2C SDA | — | — | Default I2C SDA |
| D5 / SCL | GPIO 23 | — | — | I2C SCL | — | — | Default I2C SCL |
| D6 / TX | GPIO 16 | — | — | — | UART0 TX | — | Serial TX |
| D7 / RX | GPIO 17 | — | — | — | UART0 RX | — | Serial RX |
| D8 / SCK | GPIO 19 | — | SPI CLK | — | — | — | Default SPI clock |
| D9 / MISO | GPIO 20 | — | SPI MISO | — | — | — | Default SPI MISO; also BOOT button GPIO |
| D10 / MOSI | GPIO 18 | — | SPI MOSI | — | — | — | Default SPI MOSI |
| MTMS | GPIO 4 | ADC1_CH4 | — | — | — | — | JTAG pad; ADC-capable |
| MTDI | GPIO 5 | ADC1_CH5 | — | — | — | — | JTAG pad; ADC-capable |
| MTCK | GPIO 6 | ADC1_CH6 | — | — | — | — | JTAG pad; ADC-capable |
| MTDO | GPIO 7 | — | — | — | — | — | JTAG pad |

> **Note**: D9 is GPIO20, but the Boot button is on GPIO9 (internal pull-up; not the same as D9 pad).

### Internal / Special GPIOs

| GPIO | Function | Driveable by user? |
| --- | --- | --- |
| GPIO 9 | Boot button / strapping (download mode) | Only after boot completes |
| GPIO 15 | Onboard user LED (active LOW) / strapping | Yes — primary user LED after boot |
| GPIO 3 | RF switch enable (LP_GPIO3) | Yes — with care; affects antenna selection |
| GPIO 14 | RF antenna select | Yes — with care; HIGH = U.FL, LOW = ceramic |
| GPIO 24–30 | Internal SPI flash | No |

## Board Advantages for SDD Projects

- **ADC1-only architecture**: No ADC2 means all analog reads are safe during active Wi-Fi — no need to gate ADC around network activity (unlike ESP32 or ESP32-S3).
- **Dual RISC-V cores**: The 20 MHz LP core can run from LP RAM while the 160 MHz HP core is gated off — useful for sensor polling without full wakeup.
- **Wi-Fi 6 (802.11ax)**: Target-ready latency and TWT (Target Wake Time) for IoT power budgeting; reference this in FunctionalSpec when designing battery-efficient connected devices.
- **Zigbee + Thread (802.15.4)**: The C6 is the first XIAO with native Matter-over-Thread support — specify protocol in FunctionalSpec when building smart home mesh devices.
- **Tiny footprint** (21 × 17.8 mm): Same XIAO form factor as ESP32S3; share enclosure and PCB designs across XIAO family.
- **Dual antenna**: Ceramic built-in (default) + U.FL external; antenna selection via GPIO14/GPIO3 for range-sensitive or enclosure-mounted applications.
- **Deep sleep ~15 µA**: Suitable for multi-month LiPo operation with infrequent wakeups; combine with `esp32-deep-sleep-engineer` skill.
- **Built-in LiPo charger**: No external charging circuit needed; status visible from charge indicator LED.
- **No PSRAM**: Budget memory usage carefully in FunctionalSpec — 512 KB SRAM is the ceiling; avoid JPEG frame buffers or large ML models without external storage.

## Cargo chip / board features

v1 firmware generation consumes **ESP32-C6** only. This board is **also supported**.

| Item | Value |
| --- | --- |
| SoC | ESP32-C6 |
| Chip Cargo feature | `esp32c6` |
| Board Cargo feature | `board-xiao-c6` |
| RTC / LP GPIOs | GPIO 0–7 only; GPIO 9 (boot button) is **not** RTC and cannot wake from deep sleep |

```toml
# From examples/<name>/
# cargo run --release --no-default-features --features board-xiao-c6
```

**Firmware notes (rust-on-metal):**

- Generated `src/board.rs` is the **only** place these pin numbers belong (including RF switch GPIO 3 / GPIO 14). `main.rs` uses board APIs, not raw GPIO literals.
- Console is `esp-println` + `espflash`. Native USB Serial/JTAG **drops** in deep sleep — LED-only observation, or use DevKitC **left** UART-C for multi-cycle logs.
- Sleep / RTC / radio APIs need `esp-hal` feature `unstable` in addition to `esp32c6`.
- Every image still emits `esp_bootloader_esp_idf::esp_app_desc!()` (image metadata, not an ESP-IDF install).
