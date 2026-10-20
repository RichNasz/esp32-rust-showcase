// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! BME280 → Wi-Fi STA → MQTT → deep sleep for ESP32-C6. Pins from `board`.

use core::ptr::addr_of_mut;

use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use esp_alloc::heap_allocator;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::ram;
use esp_metadata_generated::memory_range;
use esp_hal::gpio::{Event, Input, InputConfig, Pull, WakeupConfig};
use esp_hal::rtc_cntl::sleep::{LowPower, RtcSleepConfig};
use esp_hal::rtc_cntl::{WakeupSource, wakeup_cause};
use esp_hal::time::{Duration as HalDuration, Instant as HalInstant};
use esp_hal::timer::timg::TimerGroup;

mod board;
mod led;
mod mqtt;
mod net;
mod sensor;

esp_bootloader_esp_idf::esp_app_desc!();

/// Wi-Fi blob plus embassy-net. Part of this budget is reclaimed DRAM.
const HEAP_SIZE: usize = 72 * 1024;
const RECLAIMED_RAM: usize =
    memory_range!("DRAM2_UNINIT").end - memory_range!("DRAM2_UNINIT").start;
const _: () = assert!(HEAP_SIZE > RECLAIMED_RAM);

const SLEEP_SECS: u64 = 15;
const ACTIVE_WINDOW: Duration = Duration::from_secs(15);
const UART_DRAIN_MS: u64 = 20;
const PERSIST_MAGIC: u32 = 0xC6B2_80E0;
const PERSIST_MAGIC_IDX: usize = 0;
const PERSIST_COUNT_IDX: usize = 1;
const PERSIST_CHECK_IDX: usize = 2;

#[esp_hal::ram(unstable(rtc_fast, persistent))]
static mut BOOT_PERSIST: [u32; 3] = [0; 3];

/// Remaining time until the wake Instant deadline, if the window is still open.
pub(crate) fn remaining(deadline: Instant) -> Option<Duration> {
    let now = Instant::now();
    (now < deadline).then(|| deadline - now)
}

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // Both regions join one heap. The descriptor above is what allows `ram(reclaimed)`.
    heap_allocator!(size: HEAP_SIZE - RECLAIMED_RAM);
    heap_allocator!(#[ram(reclaimed)] size: RECLAIMED_RAM);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    esp_println::logger::init_logger(log::LevelFilter::Info);

    let deadline = Instant::now() + ACTIVE_WINDOW;
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

    let _ = spawner;
    let success = run_cycle(peripherals.WIFI, sample.as_ref(), boot_count, deadline).await;

    led::show_result(
        #[cfg(feature = "board-devkitc-c6")]
        peripherals.RMT,
        board::take_status_led(),
        success,
        deadline,
    )
    .await;

    arm_gpio_wakeup(board::take_wake_button());
    log::info!("Sleeping for {SLEEP_SECS} s");
    Timer::after(Duration::from_millis(UART_DRAIN_MS)).await;

    let mut lp = LowPower::new(peripherals.LPWR);
    lp.set_wakeup_deadline(HalInstant::now() + HalDuration::from_secs(SLEEP_SECS));
    lp.sleep_deep(RtcSleepConfig::deep());
}

async fn run_cycle(
    wifi: esp_hal::peripherals::WIFI<'static>,
    sample: Option<&sensor::Sample>,
    boot_count: u32,
    deadline: Instant,
) -> bool {
    if remaining(deadline).is_none() {
        log::warn!("Active window expired; sleeping");
        return false;
    }
    let Some(s) = sample else {
        return false;
    };
    log_sample(s);
    let c = net::creds();
    if !net::wifi_configured(&c) {
        log::warn!("Wi-Fi SSID not configured (compile-time env); skipping MQTT");
        return false;
    }
    net::publish_sample(wifi, s, boot_count, &c, deadline).await
}

fn log_sample(s: &sensor::Sample) {
    let (pres_hpa, pres_tenth) = s.pressure_hpa_parts();
    log::info!(
        "T={}.{} C  H={}.{} %  P={}.{} hPa",
        s.temp_centi / 100,
        (s.temp_centi % 100).unsigned_abs() / 10,
        s.hum_milli / 1000,
        (s.hum_milli / 100) % 10,
        pres_hpa,
        pres_tenth
    );
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
