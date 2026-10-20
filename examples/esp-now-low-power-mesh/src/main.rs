// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! ESP-NOW sensor-to-gateway pair for ESP32-C6. The directory name is historical.
//! Pins come from `board`. One image, two compile-time roles.

use embassy_executor::Spawner;
#[cfg(feature = "role-sensor")]
use embassy_time::{Duration, Instant, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;

mod board;
mod frame;
mod led;
mod radio;

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(all(feature = "role-sensor", feature = "role-gateway"))]
compile_error!(
    "role features are mutually exclusive: enable only one of role-sensor, role-gateway"
);

#[cfg(not(any(feature = "role-sensor", feature = "role-gateway")))]
compile_error!("select a role feature: role-sensor (default) or role-gateway");

#[cfg(feature = "role-sensor")]
const SLEEP_SECS: u64 = 15;
#[cfg(feature = "role-sensor")]
const ACTIVE_WINDOW: Duration = Duration::from_secs(3);
#[cfg(feature = "role-sensor")]
const UART_DRAIN_MS: u64 = 20;
#[cfg(feature = "role-sensor")]
const PERSIST_MAGIC: u32 = 0xC6E5_4E01;
#[cfg(feature = "role-sensor")]
const PERSIST_MAGIC_IDX: usize = 0;
#[cfg(feature = "role-sensor")]
const PERSIST_COUNT_IDX: usize = 1;
#[cfg(feature = "role-sensor")]
const PERSIST_CHECK_IDX: usize = 2;

#[cfg(feature = "role-sensor")]
#[esp_hal::ram(unstable(rtc_fast, persistent))]
static mut BOOT_PERSIST: [u32; 3] = [0; 3];

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // DRAM heap for the Wi-Fi / ESP-NOW blob. About 72 KiB, same order as the
    // other radio examples. No image-sized buffers.
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    esp_println::logger::init_logger(log::LevelFilter::Info);

    let _ = spawner;
    log::info!(
        "ESP-NOW {} — channel {} — {} GPIO {}",
        role_name(),
        radio::CHANNEL,
        board::LED_KIND,
        board::LED_GPIO
    );

    let mut led = led::StatusLed::new(
        #[cfg(feature = "board-devkitc-c6")]
        peripherals.RMT,
        board::take_status_led(),
    )
    .await;

    #[cfg(feature = "role-sensor")]
    run_sensor(peripherals.WIFI, peripherals.LPWR, &mut led).await;

    #[cfg(feature = "role-gateway")]
    radio::gateway_forever(peripherals.WIFI, &mut led).await;
}

#[cfg(feature = "role-sensor")]
async fn run_sensor(
    wifi: esp_hal::peripherals::WIFI<'static>,
    lpwr: esp_hal::peripherals::LPWR<'static>,
    led: &mut led::StatusLed,
) -> ! {
    use esp_hal::rtc_cntl::sleep::{LowPower, RtcSleepConfig};
    use esp_hal::time::{Duration as HalDuration, Instant as HalInstant};

    let boot_count = take_boot_count();
    log_wakeup(boot_count);
    let deadline = Instant::now() + ACTIVE_WINDOW;
    radio::sensor_once(wifi, led, boot_count, deadline).await;

    arm_gpio_wakeup(board::take_wake_button());
    log::info!("Sleeping for {SLEEP_SECS} s");
    Timer::after(Duration::from_millis(UART_DRAIN_MS)).await;

    // Timer is always armed. GPIO 0 is additional. Deep sleep does not return.
    let mut lp = LowPower::new(lpwr);
    lp.set_wakeup_deadline(HalInstant::now() + HalDuration::from_secs(SLEEP_SECS));
    lp.sleep_deep(RtcSleepConfig::deep());
}

#[cfg(feature = "role-sensor")]
fn role_name() -> &'static str {
    "sensor"
}

#[cfg(feature = "role-gateway")]
fn role_name() -> &'static str {
    "gateway"
}

#[cfg(feature = "role-sensor")]
fn persist_state() -> &'static mut [u32; 3] {
    use core::ptr::addr_of_mut;
    // Single-core; called once per reset from `main` before any other user of this static.
    unsafe { &mut *addr_of_mut!(BOOT_PERSIST) }
}

#[cfg(feature = "role-sensor")]
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

#[cfg(feature = "role-sensor")]
fn log_wakeup(boot_count: u32) {
    use esp_hal::rtc_cntl::{WakeupSource, wakeup_cause};

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

#[cfg(feature = "role-sensor")]
fn arm_gpio_wakeup(pin: esp_hal::gpio::AnyPin<'static>) {
    use esp_hal::gpio::{Event, Input, InputConfig, Pull, WakeupConfig};

    let cfg = InputConfig::default().with_pull(Pull::Up);
    let mut button = Input::new(pin, cfg);
    button.listen(Event::LowLevel);
    let wake_cfg = WakeupConfig::default().with_low_power_path(true);
    if button.apply_wakeup_config(&wake_cfg).is_err() {
        log::warn!("GPIO {} is not an RTC wake pad", board::WAKE_GPIO);
    }
}
