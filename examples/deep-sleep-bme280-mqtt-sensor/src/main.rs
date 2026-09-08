// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! BME280 → Wi-Fi STA → MQTT → deep sleep for ESP32-C6. Pins from `board`.

use core::ptr::addr_of_mut;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Event, Input, InputConfig, Pull, WakeupConfig};
use esp_hal::rtc_cntl::sleep::{LowPower, RtcSleepConfig};
use esp_hal::rtc_cntl::{WakeupSource, wakeup_cause};
use esp_hal::time::{Duration as HalDuration, Instant};
use esp_hal::timer::timg::TimerGroup;

mod board;
mod led;
mod mqtt;
mod net;
mod sensor;

const SLEEP_SECS: u64 = 15;
const UART_DRAIN_MS: u64 = 20;
const PERSIST_MAGIC: u32 = 0xC6B2_80E0;
const PERSIST_MAGIC_IDX: usize = 0;
const PERSIST_COUNT_IDX: usize = 1;
const PERSIST_CHECK_IDX: usize = 2;

#[esp_hal::ram(unstable(rtc_fast, persistent))]
static mut BOOT_PERSIST: [u32; 3] = [0; 3];

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // DRAM heap for the Wi-Fi blob + embassy-net. `ram(reclaimed)` needs the
    // ESP-IDF-compatible app descriptor; this example does not ship one.
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    esp_println::logger::init_logger(log::LevelFilter::Info);

    let boot_count = take_boot_count();
    log_wakeup(boot_count);
    log::info!(
        "BME280 MQTT node — {}, LED GPIO {}, I2C {}/{}, wake GPIO {}, sleep {} s",
        board::LED_KIND,
        board::LED_GPIO,
        board::I2C_SDA_GPIO,
        board::I2C_SCL_GPIO,
        board::WAKE_GPIO,
        SLEEP_SECS
    );

    let sample = sensor::read_bme280(
        peripherals.I2C0,
        board::take_i2c_sda(),
        board::take_i2c_scl(),
    )
    .await;

    let mut success = false;
    if let Some(ref s) = sample {
        log::info!(
            "T={}.{} C  H={}.{} %  P={}.{} hPa",
            s.temp_centi / 100,
            (s.temp_centi % 100).unsigned_abs() / 10,
            s.hum_milli / 1000,
            (s.hum_milli / 100) % 10,
            s.pressure_pa / 10000,
            (s.pressure_pa / 1000) % 10
        );
        let c = net::creds();
        if net::wifi_configured(&c) {
            success = net::publish_sample(spawner, peripherals.WIFI, s, boot_count, &c).await;
        } else {
            log::warn!("Wi-Fi SSID not configured (compile-time env); skipping MQTT");
        }
    }

    led::show_result(
        #[cfg(feature = "board-devkitc-c6")]
        peripherals.RMT,
        board::take_status_led(),
        success,
    )
    .await;

    arm_gpio_wakeup(board::take_wake_button());
    log::info!("Sleeping for {SLEEP_SECS} s");
    Timer::after(Duration::from_millis(UART_DRAIN_MS)).await;

    let mut lp = LowPower::new(peripherals.LPWR);
    lp.set_wakeup_deadline(Instant::now() + HalDuration::from_secs(SLEEP_SECS));
    lp.sleep_deep(RtcSleepConfig::deep());
}

fn persist_state() -> &'static mut [u32; 3] {
    unsafe { &mut *addr_of_mut!(BOOT_PERSIST) }
}

fn take_boot_count() -> u32 {
    let state = persist_state();
    let magic = state[PERSIST_MAGIC_IDX];
    let count = state[PERSIST_COUNT_IDX];
    let check = state[PERSIST_CHECK_IDX];
    let valid = magic == PERSIST_MAGIC && check == (magic ^ count);
    let next = if valid { count.wrapping_add(1) } else { 1 };
    state[PERSIST_MAGIC_IDX] = PERSIST_MAGIC;
    state[PERSIST_COUNT_IDX] = next;
    state[PERSIST_CHECK_IDX] = PERSIST_MAGIC ^ next;
    next
}

fn log_wakeup(boot_count: u32) {
    let cause = wakeup_cause();
    if cause.is_empty() {
        log::info!("First boot — counter starts at {boot_count} (resets on power-cycle)");
    } else if cause.contains(WakeupSource::Timer) {
        log::info!("Wake #{boot_count} — timer");
    } else if cause.contains(WakeupSource::Gpio) {
        log::info!("Wake #{boot_count} — GPIO");
    } else {
        log::warn!("Wake #{boot_count} — unexpected wakeup {cause:?}");
    }
}

fn arm_gpio_wakeup(pin: esp_hal::gpio::AnyPin<'static>) {
    let cfg = InputConfig::default().with_pull(Pull::Up);
    let mut button = Input::new(pin, cfg);
    button.listen(Event::LowLevel);
    let wake_cfg = WakeupConfig::default().with_low_power_path(true);
    if button.apply_wakeup_config(&wake_cfg).is_err() {
        log::warn!("GPIO {} is not an RTC wake pad", board::WAKE_GPIO);
    }
}
