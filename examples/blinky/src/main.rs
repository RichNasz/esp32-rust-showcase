// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! Breathing LED for ESP32-C6. Pins and LED kind come from `board`.

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::gpio::AnyPin;
use esp_hal::timer::timg::TimerGroup;

mod board;

esp_bootloader_esp_idf::esp_app_desc!();

const BREATH_HALF_MS: u64 = 2000;
#[cfg(feature = "board-devkitc-c6")]
const BRIGHTNESS_CAP: u8 = 64;

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    esp_println::logger::init_logger(log::LevelFilter::Info);

    let _ = spawner;
    let led_pin = board::take_status_led();

    log::info!(
        "Blinky — {}, GPIO {}, breathing period {} ms",
        board::LED_KIND,
        board::LED_GPIO,
        BREATH_HALF_MS * 2
    );

    #[cfg(feature = "board-devkitc-c6")]
    run_ws2812(peripherals.RMT, led_pin).await;

    #[cfg(feature = "board-xiao-c6")]
    run_gpio_led(peripherals.LEDC, led_pin).await;
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
async fn run_ws2812(rmt: esp_hal::peripherals::RMT<'static>, led_pin: AnyPin<'static>) -> ! {
    use esp_hal::gpio::Level;
    use esp_hal::rmt::{Rmt, TxChannelConfig, TxChannelCreator};
    use esp_hal::time::Rate;

    // 80 MHz / clk_divider 8 = 10 MHz → 100 ns ticks. WS2812B ±150 ns band.
    const TICK_DIV: u8 = 8;
    const STEPS: u8 = 64;
    const STEP_MS: u64 = BREATH_HALF_MS / STEPS as u64;
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
        // 300 µs low. WS2812B latches at 50 µs; the longer gap covers clones.
        reset_ticks: 3000,
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
        "WS2812 RMT channel init on GPIO {} (GRB order, max brightness {}/255)",
        board::LED_GPIO,
        BRIGHTNESS_CAP
    );

    write_ws2812(&mut channel, 0, 0, 0, &timing).await;
    log::info!("Starting WS2812 breathing loop");

    let mut color_idx = 0usize;
    loop {
        let (r, g, b) = COLORS[color_idx];
        log::info!(
            "Cycle color {}/{}  r={} g={} b={}",
            color_idx + 1,
            COLORS.len(),
            r,
            g,
            b
        );

        for step in 0..=STEPS {
            let (sr, sg, sb) = scale_rgb(r, g, b, step, STEPS);
            write_ws2812(&mut channel, sr, sg, sb, &timing).await;
            Timer::after(Duration::from_millis(STEP_MS)).await;
        }
        for step in (0..STEPS).rev() {
            let (sr, sg, sb) = scale_rgb(r, g, b, step, STEPS);
            write_ws2812(&mut channel, sr, sg, sb, &timing).await;
            Timer::after(Duration::from_millis(STEP_MS)).await;
        }

        color_idx = (color_idx + 1) % COLORS.len();
    }
}

#[cfg(feature = "board-devkitc-c6")]
fn scale_rgb(r: u8, g: u8, b: u8, step: u8, steps: u8) -> (u8, u8, u8) {
    (
        scale_channel(r, step, steps),
        scale_channel(g, step, steps),
        scale_channel(b, step, steps),
    )
}

#[cfg(feature = "board-devkitc-c6")]
fn scale_channel(max: u8, step: u8, steps: u8) -> u8 {
    let capped = core::cmp::min(max, BRIGHTNESS_CAP);
    ((u16::from(capped) * u16::from(step)) / u16::from(steps)) as u8
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
async fn run_gpio_led(ledc: esp_hal::peripherals::LEDC<'static>, led_pin: AnyPin<'static>) -> ! {
    use esp_hal::gpio::DriveMode;
    use esp_hal::ledc::channel::{self, ChannelIFace};
    use esp_hal::ledc::timer::{self, TimerIFace};
    use esp_hal::ledc::{LSGlobalClkSource, Ledc, LowSpeed};
    use esp_hal::time::Rate;

    const STEPS: u8 = 100;
    const STEP_MS: u64 = BREATH_HALF_MS / STEPS as u64;

    let mut ledc = Ledc::new(ledc);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk);

    let mut lstimer0 = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    if lstimer0
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty13Bit,
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(5),
        })
        .is_err()
    {
        log::error!("LEDC timer config failed");
        halt().await
    }

    let mut channel0 = ledc.channel(channel::Number::Channel0, led_pin);
    // Active-LOW: 100% duty holds the pin high → LED off (start dark).
    let off_duty = if board::LED_ACTIVE_LOW { 100 } else { 0 };
    if channel0
        .configure(channel::config::Config {
            timer: &lstimer0,
            duty_pct: off_duty,
            drive_mode: DriveMode::PushPull,
        })
        .is_err()
    {
        log::error!("LEDC channel config failed");
        halt().await
    }

    log::info!("Board GPIO={}  polarity=active-LOW", board::LED_GPIO);
    log::info!("Breathing period: {} ms", BREATH_HALF_MS * 2);
    log::info!("Starting GPIO LED breathing loop (LEDC PWM, Embassy-timed duty)");

    loop {
        log::info!("Fade up");
        for step in 0..=STEPS {
            apply_breath_duty(&channel0, step, STEPS);
            Timer::after(Duration::from_millis(STEP_MS)).await;
        }
        log::info!("Fade down");
        for step in (0..STEPS).rev() {
            apply_breath_duty(&channel0, step, STEPS);
            Timer::after(Duration::from_millis(STEP_MS)).await;
        }
    }
}

#[cfg(feature = "board-xiao-c6")]
fn apply_breath_duty(channel: &impl ChannelIFaceExt, step: u8, steps: u8) {
    let brightness_pct = ((u16::from(step) * 100) / u16::from(steps)) as u8;
    let duty_pct = if board::LED_ACTIVE_LOW {
        100u8.saturating_sub(brightness_pct)
    } else {
        brightness_pct
    };
    channel.apply_duty(duty_pct);
}

#[cfg(feature = "board-xiao-c6")]
trait ChannelIFaceExt {
    fn apply_duty(&self, duty_pct: u8);
}

#[cfg(feature = "board-xiao-c6")]
impl<'a> ChannelIFaceExt for esp_hal::ledc::channel::Channel<'a, esp_hal::ledc::LowSpeed> {
    fn apply_duty(&self, duty_pct: u8) {
        use esp_hal::ledc::channel::ChannelIFace;
        if self.set_duty(duty_pct).is_err() {
            log::error!("LEDC set_duty failed");
        }
    }
}

async fn halt() -> ! {
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
