// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! HP core for LP-GPIO wakeup. Pins come from `board`. The LP image is loaded
//! before every deep sleep.

use core::ptr::addr_of_mut;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::gpio::{Input, InputConfig, Pull};
use esp_hal::load_lp_code;
use esp_hal::lp_core::{LpCore, LpCoreWakeupSource};
use esp_hal::rtc_cntl::sleep::{LowPower, RtcSleepConfig};
use esp_hal::rtc_cntl::{WakeupSource, wakeup_cause};
use esp_hal::time::{Duration as HalDuration, Instant};
use esp_hal::timer::timg::TimerGroup;

mod board;
mod led;

esp_bootloader_esp_idf::esp_app_desc!();

const SLEEP_SECS: u64 = 20;
const UART_DRAIN_MS: u64 = 20;
const PERSIST_MAGIC: u32 = 0xC6A5_1C00;
const PERSIST_MAGIC_IDX: usize = 0;
const PERSIST_COUNT_IDX: usize = 1;
const PERSIST_CHECK_IDX: usize = 2;

/// CodingSpec handshake word in C6 LP RAM (`0x5000_0000` region).
/// Chosen so it does not sit in the LP image or on the LP stack.
/// `LpCore::new` clears LP RAM, so read this before constructing `LpCore`.
const HANDSHAKE_ADDR: usize = 0x5000_2000;
const HANDSHAKE_GPIO_EDGE: u32 = 0x4750_494F;

const LP_IMAGE_SHA: &str = env!("LP_CORE_ELF_SHA");

#[esp_hal::ram(unstable(rtc_fast, persistent))]
static mut BOOT_PERSIST: [u32; 3] = [0; 3];

#[derive(Clone, Copy)]
enum WakeClass {
    FirstBoot,
    LpGpio,
    Timer,
    Other,
}

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    esp_println::logger::init_logger(log::LevelFilter::Info);

    let handshake = read_handshake();
    let class = classify(wakeup_cause(), handshake);
    let count = record_count(class);
    log_cycle(class, count);

    let mut status = led::StatusLed::new(peripherals.RMT, board::take_status_led()).await;
    match class {
        WakeClass::LpGpio => status.gpio_wake().await,
        WakeClass::Timer | WakeClass::Other => status.timer_wake().await,
        WakeClass::FirstBoot => status.off().await,
    }
    status.off().await;

    arm_lp_and_sleep(peripherals.LP_CORE, peripherals.LPWR).await;
}

fn read_handshake() -> u32 {
    unsafe { (HANDSHAKE_ADDR as *const u32).read_volatile() }
}

fn classify(cause: esp_hal::rtc_cntl::WakeupReason, handshake: u32) -> WakeClass {
    let lp = cause.contains(WakeupSource::LpCore);
    let timer = cause.contains(WakeupSource::Timer);
    let saw_edge = handshake == HANDSHAKE_GPIO_EDGE;

    if cause.is_empty() {
        if saw_edge {
            log::warn!("handshake set on first boot; ignoring it");
        }
        WakeClass::FirstBoot
    } else if lp {
        if !saw_edge || timer {
            log::warn!(
                "wakeup {cause:?} disagrees with handshake {handshake:#010x}; trusting LP-core cause"
            );
        }
        WakeClass::LpGpio
    } else if timer {
        if saw_edge {
            log::warn!(
                "timer wake but handshake {handshake:#010x} still set; not counting a press"
            );
        }
        WakeClass::Timer
    } else {
        log::warn!("unexpected wakeup {cause:?}; not counting a press");
        WakeClass::Other
    }
}

fn persist_state() -> &'static mut [u32; 3] {
    // Single-core. Called once per reset from `main` before any other user of this static.
    unsafe { &mut *addr_of_mut!(BOOT_PERSIST) }
}

fn record_count(class: WakeClass) -> u32 {
    let state = persist_state();
    let magic = state[PERSIST_MAGIC_IDX];
    let count = state[PERSIST_COUNT_IDX];
    let check = state[PERSIST_CHECK_IDX];
    let valid = magic == PERSIST_MAGIC && check == (magic ^ count);
    let current = if valid { count } else { 0 };
    let next = match class {
        WakeClass::LpGpio => current.wrapping_add(1),
        WakeClass::FirstBoot | WakeClass::Timer | WakeClass::Other => current,
    };
    state[PERSIST_MAGIC_IDX] = PERSIST_MAGIC;
    state[PERSIST_COUNT_IDX] = next;
    state[PERSIST_CHECK_IDX] = PERSIST_MAGIC ^ next;
    next
}

fn log_cycle(class: WakeClass, count: u32) {
    match class {
        WakeClass::FirstBoot => {
            log::info!("First boot — GPIO count {count} (resets on power-cycle)");
        }
        WakeClass::LpGpio => log::info!("LP-GPIO wake — GPIO count {count}"),
        WakeClass::Timer => log::info!("Timer wake — GPIO count {count} unchanged"),
        WakeClass::Other => log::info!("Other wake — GPIO count {count} unchanged"),
    }
    log::info!("Status LED {}, GPIO {}", board::LED_KIND, board::LED_GPIO);
}

async fn arm_lp_and_sleep(
    lp_core: esp_hal::peripherals::LP_CORE<'static>,
    lpwr: esp_hal::peripherals::LPWR<'static>,
) -> ! {
    let Some(lp_pin) = prepare_wake_pin() else {
        log::error!("LP wake pad is not an LP GPIO");
        halt().await
    };

    // `new` stops the core and clears LP RAM. Load the image after that.
    let mut lp_core = LpCore::new(lp_core);
    lp_core.stop();
    // `load_lp_code!` forgets the LP pad so the mux survives. The pad type has no Drop.
    #[allow(clippy::forget_non_drop)]
    {
        let code = include!(concat!(env!("OUT_DIR"), "/load_lp.rs"));
        code.run(&mut lp_core, LpCoreWakeupSource::HpCpu, lp_pin);
    }
    lp_core.enable_wakeup();

    log::info!("LP image loaded ({})", &LP_IMAGE_SHA[..8]);
    log::info!(
        "Sleeping for {SLEEP_SECS} s — LP GPIO {} or RTC timer",
        board::LP_WAKE_GPIO
    );
    Timer::after(Duration::from_millis(UART_DRAIN_MS)).await;

    let mut power = LowPower::new(lpwr);
    power.set_wakeup_deadline(Instant::now() + HalDuration::from_secs(SLEEP_SECS));
    power.sleep_deep(RtcSleepConfig::deep());
}

fn prepare_wake_pin()
-> Option<esp_hal::gpio::lp_io::LowPowerInput<'static, { board::LP_WAKE_GPIO }>> {
    let pin = board::take_wake_gpio();
    let input = Input::new(pin, InputConfig::default().with_pull(Pull::Up));
    // `into_lp` clears the pull. The LP pad needs it held before the image runs.
    let lp_pin = input.into_lp::<{ board::LP_WAKE_GPIO }>().ok()?;
    lp_pin.pullup_enable(true);
    Some(lp_pin)
}

async fn halt() -> ! {
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
