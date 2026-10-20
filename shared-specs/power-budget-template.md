# Power Budget Template

**Classification:** Human-authored shared spec
**Applies to:** All low-power examples (deep sleep, light sleep, LP-core / ULP)

There is no `sdkconfig`. Generators must not emit `CONFIG_PM_*`, `CONFIG_ESP_SLEEP_*`, `CONFIG_FREERTOS_*`, `CONFIG_ULP_*`, or any other Kconfig.

---

## How to Use This Template

Copy the tables below into the example's `FunctionalSpec.md` under a `## Power Budget` section. Fill in the values for the specific hardware. Leave rows that don't apply with `—`.

The generator reads these values to:

- Configure sleep wakeup sources and timing in Rust (`esp-hal` / Embassy). Do not emit sdkconfig.
- Validate that the estimated current draw meets the target.

---

## Power Budget Table

```markdown
## Power Budget

| Mode | Typical Current | Duration | Energy per Cycle |
|---|---|---|---|
| Active (CPU @ 160 MHz, C6 HP) | ___ mA | ___ ms | ___ µAh |
| Radio on (Wi-Fi / BLE / ESP-NOW Tx) | ___ mA | ___ ms | ___ µAh |
| Radio teardown (before sleep) | ___ mA | 5–10 ms | ___ µAh |
| Modem-sleep (Wi-Fi associated) | ___ mA | ___ ms | ___ µAh |
| Light sleep | ___ µA | ___ s | ___ µAh |
| Deep sleep (RTC only) | ___ µA | ___ s | ___ µAh |
| Deep sleep + LP-core (C6) | ___ µA | ___ s | ___ µAh |
| **Total per duty cycle** | — | ___ s | ___ µAh |
| **Estimated daily energy** | — | 24 h | ___ mAh |

**Battery target:** ___ mAh at ___ V → estimated life: ___ days (at ___ duty cycle)
**Sleep wakeup source:** RTC timer / RTC GPIO **0–7** / LP-core (C6 has no capacitive-touch wakeup)
**Wakeup interval:** ___ seconds
**Observation:** UART-bridge (DevKitC **left** USB-C) keeps `espflash monitor` across sleep; native USB Serial/JTAG drops the port
```

---

## Reference Current Draw

Use the **ESP32-C6** table for v1 FunctionalSpecs. Prefer the matching `board-specs/` Power Management row when it disagrees with these typicals. Do not copy current figures from another SoC.

### ESP32-C6 (v1 required — RISC-V HP 160 MHz + LP 20 MHz)

Typical figures from the C6 datasheet / v1 board-specs (DevKitC-C6, XIAO C6). Fill the FunctionalSpec from the **board you ship**, not this average.

| Mode | Typical |
|---|---|
| Active, HP @ 160 MHz, Wi-Fi 6 Tx | ~200–350 mA peak (DevKitC); ~80 mA avg / ~250 mA peak (XIAO wiki) |
| Modem sleep | ~30 mA (XIAO C6 wiki) |
| Light sleep | ~3.1 mA (XIAO C6 wiki) |
| Deep sleep, RTC | ~7 µA (RTC domain off) to ~15 µA (RTC domain active / XIAO all-off) |
| Deep sleep + LP-core running | higher than RTC-only; measure; do not assume classic ESP32 ULP numbers |

Radio left powered (forgot `esp-radio` teardown) reads **milliamps**, not microamps — treat that as a spec bug, not a datasheet row.

---

## Sleep configuration (Rust, not Kconfig)

There is no `sdkconfig`. Express sleep in `esp-hal` / `esp-rtos` / Embassy in the CodingSpec and generated firmware:

- **Deep sleep + timer** — `esp-hal` RTC timer wakeup (feature `unstable` on this HAL line). Deep sleep does not return; the chip resets on wake. Persist counters with `#[esp_hal::ram(persistent)]` (or `#[ram(unstable(persistent))]`). Prefer the internal RC RTC clock when an RTC XTAL’s extra current is not justified.
- **Light sleep** — idle the Embassy / `esp-rtos` executor; keep flash powered if resume latency matters. Not FreeRTOS tickless idle.
- **LP-core (ESP32-C6, v1)** — GPIO **0–7 only** (GPIO 9 is not RTC). Load the LP image into LP RAM **before** the HP core sleeps (`esp32-lp-core-engineer`). Then enable LP wakeup and enter HP deep sleep.
- **GPIO hold** — if an output must keep its level across sleep, apply `esp-hal` GPIO / RTC hold **after** radio teardown and **before** `sleep_deep`.
- **ULP / capacitive touch on other SoCs** — not in v1. C6 has no touch peripheral; do not copy `esp_sleep_enable_touchpad_wakeup` from S3 examples.

Tear down `esp-radio` (if used) and de-init SPI / I2C / UART before entering deep sleep. Include ~5–10 ms teardown in the active window of the energy budget.

---

## Battery Life Estimation Formula

```text
daily_mAh = (active_mA × active_s + sleep_µA/1000 × sleep_s) × cycles_per_day / 3600

days = battery_mAh × 0.8 / daily_mAh   # 80% usable capacity rule of thumb
```

Document both the formula and the parameter values in the `FunctionalSpec.md` so regeneration is fully reproducible.

---

## Design Constraints

- Active phase must complete all sensor reads, processing, and transmission before entering sleep. Never extend active time by waiting for peripherals that could be pre-configured.
- Minimize wake count. Consolidate multiple sensor reads into a single wake cycle rather than waking separately for each sensor.
- RTC GPIO must be used for wakeup from deep sleep — regular GPIO cannot wake from deep sleep. On ESP32-C6 that is GPIO **0–7 only**.
- Flash is powered down during HP deep sleep. The LP core cannot fetch its image from flash after that, so program LP RAM first. Do not plan on the LP/ULP core writing flash during sleep.
- De-initialize SPI, I2C, UART, and `esp-radio` before entering deep sleep via `esp-hal` RTC sleep. Do not call ESP-IDF `esp_deep_sleep_start()`.
