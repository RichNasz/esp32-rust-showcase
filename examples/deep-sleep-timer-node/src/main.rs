// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! RTC timer deep-sleep node for ESP32-C6. Pins and LED kind come from `board`.

use core::ptr::addr_of_mut;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::gpio::AnyPin;
use esp_hal::rtc_cntl::sleep::{LowPower, RtcSleepConfig};
use esp_hal::rtc_cntl::{WakeupSource, wakeup_cause};
use esp_hal::time::{Duration as HalDuration, Instant};
use esp_hal::timer::timg::TimerGroup;

mod board;

esp_bootloader_esp_idf::esp_app_desc!();

const SLEEP_SECS: u64 = 5;
const HEARTBEAT_MS: u64 = 100;
const UART_DRAIN_MS: u64 = 20;
const PERSIST_MAGIC: u32 = 0xC6A5_51EE;
const PERSIST_MAGIC_IDX: usize = 0;
const PERSIST_COUNT_IDX: usize = 1;
const PERSIST_CHECK_IDX: usize = 2;

#[cfg(feature = "board-devkitc-c6")]
const BRIGHTNESS_CAP: u8 = 64;

#[esp_hal::ram(unstable(rtc_fast, persistent))]
static mut BOOT_PERSIST: [u32; 3] = [0; 3];

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    esp_println::logger::init_logger(log::LevelFilter::Info);

    let _ = spawner;
    let boot_count = take_boot_count();
    log_wakeup(boot_count);

    let led_pin = board::take_status_led();
    log::info!(
        "Timer node — {}, GPIO {}, sleep {} s",
        board::LED_KIND,
        board::LED_GPIO,
        SLEEP_SECS
    );

    #[cfg(feature = "board-devkitc-c6")]
    heartbeat_ws2812(peripherals.RMT, led_pin).await;

    #[cfg(feature = "board-xiao-c6")]
    heartbeat_gpio(led_pin).await;

    log::info!("Sleeping for {SLEEP_SECS} s");
    Timer::after(Duration::from_millis(UART_DRAIN_MS)).await;

    let mut lp = LowPower::new(peripherals.LPWR);
    lp.set_wakeup_deadline(Instant::now() + HalDuration::from_secs(SLEEP_SECS));
    lp.sleep_deep(RtcSleepConfig::deep());
}

fn persist_state() -> &'static mut [u32; 3] {
    // Single-core; called once per reset from `main` before any other user of this static.
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
        log::info!("Wake #{boot_count} — timer wakeup");
    } else {
        log::warn!("Wake #{boot_count} — unexpected wakeup {cause:?}");
    }
}

impl core::fmt::Display for board::LedKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            #[cfg(feature = "board-devkitc-c6")]
            board::LedKind::Ws2812 => f.write_str("WS2812 RGB LED"),
            #[cfg(feature = "board-xiao-c6")]
            board::LedKind::GpioActiveLow => f.write_str("GPIO LED (active-LOW)"),
        }
    }
}

#[cfg(feature = "board-devkitc-c6")]
struct NrzTiming {
    t0h: u16,
    t0l: u16,
    t1h: u16,
    t1l: u16,
    reset_ticks: u16,
}

#[cfg(feature = "board-devkitc-c6")]
async fn heartbeat_ws2812(rmt: esp_hal::peripherals::RMT<'static>, led_pin: AnyPin<'static>) {
    use esp_hal::gpio::Level;
    use esp_hal::rmt::{Rmt, TxChannelConfig, TxChannelCreator};
    use esp_hal::rng::Rng;
    use esp_hal::time::Rate;

    // 80 MHz / clk_divider 8 = 10 MHz → 100 ns ticks. WS2812B ±150 ns band.
    const TICK_DIV: u8 = 8;
    const COLORS: [(u8, u8, u8); 6] = [
        (64, 0, 0),
        (0, 64, 0),
        (0, 0, 64),
        (0, 64, 64),
        (64, 0, 64),
        (64, 32, 0),
    ];
    let timing = NrzTiming {
        t0h: 3,
        t0l: 9,
        t1h: 9,
        t1l: 3,
        reset_ticks: 500,
    };

    let rmt = match Rmt::new(rmt, Rate::from_mhz(80)) {
        Ok(rmt) => rmt.into_async(),
        Err(_) => {
            log::error!("RMT init failed");
            halt().await
        }
    };

    let tx_config = TxChannelConfig::default()
        .with_clk_divider(TICK_DIV)
        .with_idle_output_level(Level::Low)
        .with_idle_output(true)
        .with_carrier_modulation(false);

    let mut channel = match rmt.channel0.configure_tx(&tx_config) {
        Ok(ch) => ch.with_pin(led_pin),
        Err(_) => {
            log::error!("RMT TX channel config failed");
            halt().await
        }
    };

    log::info!(
        "WS2812 RMT init on GPIO {} (GRB order, max brightness {}/255)",
        board::LED_GPIO,
        BRIGHTNESS_CAP
    );

    write_ws2812(&mut channel, 0, 0, 0, &timing).await;

    let idx = (Rng::new().random() as usize) % COLORS.len();
    let (r, g, b) = COLORS[idx];
    log::info!(
        "Heartbeat color {}/{}  r={} g={} b={}",
        idx + 1,
        COLORS.len(),
        r,
        g,
        b
    );
    write_ws2812(&mut channel, r, g, b, &timing).await;
    Timer::after(Duration::from_millis(HEARTBEAT_MS)).await;
    write_ws2812(&mut channel, 0, 0, 0, &timing).await;
    Timer::after(Duration::from_millis(HEARTBEAT_MS)).await;
}

#[cfg(feature = "board-devkitc-c6")]
fn encode_grb(r: u8, g: u8, b: u8, timing: &NrzTiming) -> [esp_hal::rmt::PulseCode; 25] {
    use esp_hal::gpio::Level;
    use esp_hal::rmt::PulseCode;

    let bit0 = PulseCode::new(Level::High, timing.t0h, Level::Low, timing.t0l);
    let bit1 = PulseCode::new(Level::High, timing.t1h, Level::Low, timing.t1l);
    let mut codes = [PulseCode::end_marker(); 25];
    let mut i = 0;
    for byte in [g, r, b] {
        for bit in 0..8u8 {
            codes[i] = if (byte & (1 << (7 - bit))) != 0 {
                bit1
            } else {
                bit0
            };
            i += 1;
        }
    }
    codes[24] = PulseCode::new(Level::Low, timing.reset_ticks, Level::Low, 0);
    codes
}

#[cfg(feature = "board-devkitc-c6")]
async fn write_ws2812(
    channel: &mut esp_hal::rmt::Channel<'_, esp_hal::Async, esp_hal::rmt::Tx>,
    r: u8,
    g: u8,
    b: u8,
    timing: &NrzTiming,
) {
    let codes = encode_grb(r, g, b, timing);
    if channel.transmit(&codes).await.is_err() {
        log::error!("WS2812 RMT transmit failed");
    }
}

#[cfg(feature = "board-xiao-c6")]
async fn heartbeat_gpio(led_pin: AnyPin<'static>) {
    use esp_hal::gpio::{Level, Output, OutputConfig};

    let off = if board::LED_ACTIVE_LOW {
        Level::High
    } else {
        Level::Low
    };
    let on = if board::LED_ACTIVE_LOW {
        Level::Low
    } else {
        Level::High
    };

    let mut led = Output::new(led_pin, off, OutputConfig::default());
    log::info!(
        "GPIO LED heartbeat on GPIO {} ({})",
        board::LED_GPIO,
        board::LED_KIND
    );
    led.set_level(on);
    Timer::after(Duration::from_millis(HEARTBEAT_MS)).await;
    led.set_level(off);
    Timer::after(Duration::from_millis(HEARTBEAT_MS)).await;
}

#[cfg(feature = "board-devkitc-c6")]
async fn halt() -> ! {
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
