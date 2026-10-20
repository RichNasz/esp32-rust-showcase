// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Status LED. DevKitC: WS2812 on RMT. XIAO: active-LOW GPIO. Same blink timings.

use embassy_time::{Duration, Timer};
use esp_hal::gpio::AnyPin;

#[cfg(feature = "board-devkitc-c6")]
const BRIGHTNESS_CAP: u8 = 64;

const GPIO_ON_MS: u64 = 200;
const GPIO_OFF_MS: u64 = 200;
const GPIO_BLINKS: u8 = 3;
const TIMER_ON_MS: u64 = 500;

#[cfg(feature = "board-devkitc-c6")]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

#[cfg(feature = "board-devkitc-c6")]
const BLACK: Rgb = Rgb { r: 0, g: 0, b: 0 };
#[cfg(feature = "board-devkitc-c6")]
const GREEN: Rgb = Rgb {
    r: 0,
    g: BRIGHTNESS_CAP,
    b: 0,
};
#[cfg(feature = "board-devkitc-c6")]
const AMBER: Rgb = Rgb {
    r: BRIGHTNESS_CAP,
    g: 32,
    b: 0,
};

/// Board status LED. Starts dark.
pub struct StatusLed {
    #[cfg(feature = "board-devkitc-c6")]
    ws: Ws2812Led,
    #[cfg(feature = "board-xiao-c6")]
    gpio: GpioLed,
}

impl StatusLed {
    pub async fn new(rmt: esp_hal::peripherals::RMT<'static>, led_pin: AnyPin<'static>) -> Self {
        #[cfg(feature = "board-devkitc-c6")]
        {
            Self {
                ws: Ws2812Led::init(rmt, led_pin).await,
            }
        }
        #[cfg(feature = "board-xiao-c6")]
        {
            let _ = rmt;
            Self {
                gpio: GpioLed::init(led_pin),
            }
        }
    }

    /// Three green (or on) blinks. A failed frame still returns.
    pub async fn gpio_wake(&mut self) {
        for _ in 0..GPIO_BLINKS {
            self.pulse(GPIO_ON_MS, GPIO_OFF_MS, true).await;
        }
    }

    /// One slower amber (or on) blink.
    pub async fn timer_wake(&mut self) {
        self.pulse(TIMER_ON_MS, GPIO_OFF_MS, false).await;
    }

    pub async fn off(&mut self) {
        self.set_lit(false, false).await;
    }

    async fn pulse(&mut self, on_ms: u64, off_ms: u64, gpio_color: bool) {
        self.set_lit(true, gpio_color).await;
        Timer::after(Duration::from_millis(on_ms)).await;
        self.set_lit(false, gpio_color).await;
        Timer::after(Duration::from_millis(off_ms)).await;
    }

    async fn set_lit(&mut self, on: bool, gpio_color: bool) {
        #[cfg(feature = "board-devkitc-c6")]
        {
            let color = if !on {
                BLACK
            } else if gpio_color {
                GREEN
            } else {
                AMBER
            };
            self.ws.set(color).await;
        }
        #[cfg(feature = "board-xiao-c6")]
        {
            let _ = gpio_color;
            if on {
                self.gpio.set_on();
            } else {
                self.gpio.set_off();
            }
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
struct Ws2812Led {
    channel: esp_hal::rmt::Channel<'static, esp_hal::Async, esp_hal::rmt::Tx>,
    timing: NrzTiming,
}

#[cfg(feature = "board-devkitc-c6")]
impl Ws2812Led {
    async fn init(rmt: esp_hal::peripherals::RMT<'static>, led_pin: AnyPin<'static>) -> Self {
        use esp_hal::gpio::Level;
        use esp_hal::rmt::{Rmt, TxChannelConfig, TxChannelCreator};
        use esp_hal::time::Rate;

        const TICK_DIV: u8 = 8;
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
            crate::board::LED_GPIO,
            BRIGHTNESS_CAP
        );
        write_ws2812(&mut channel, BLACK, &timing).await;
        Self { channel, timing }
    }

    async fn set(&mut self, color: Rgb) {
        write_ws2812(&mut self.channel, color, &self.timing).await;
    }
}

#[cfg(feature = "board-devkitc-c6")]
fn encode_grb(color: Rgb, timing: &NrzTiming) -> [esp_hal::rmt::PulseCode; 25] {
    use esp_hal::gpio::Level;
    use esp_hal::rmt::PulseCode;

    let bit0 = PulseCode::new(Level::High, timing.t0h, Level::Low, timing.t0l);
    let bit1 = PulseCode::new(Level::High, timing.t1h, Level::Low, timing.t1l);
    let mut codes = [PulseCode::end_marker(); 25];
    let mut index = 0;
    for byte in [color.g, color.r, color.b] {
        for bit in 0..8u8 {
            codes[index] = if (byte & (1 << (7 - bit))) != 0 {
                bit1
            } else {
                bit0
            };
            index += 1;
        }
    }
    codes[24] = PulseCode::new(Level::Low, timing.reset_ticks, Level::Low, 0);
    codes
}

#[cfg(feature = "board-devkitc-c6")]
async fn write_ws2812(
    channel: &mut esp_hal::rmt::Channel<'_, esp_hal::Async, esp_hal::rmt::Tx>,
    color: Rgb,
    timing: &NrzTiming,
) {
    let codes = encode_grb(color, timing);
    if channel.transmit(&codes).await.is_err() {
        log::error!("WS2812 RMT transmit failed");
    }
}

#[cfg(feature = "board-xiao-c6")]
struct GpioLed {
    pin: esp_hal::gpio::Output<'static>,
    on: esp_hal::gpio::Level,
    off: esp_hal::gpio::Level,
}

#[cfg(feature = "board-xiao-c6")]
impl GpioLed {
    fn init(led_pin: AnyPin<'static>) -> Self {
        use esp_hal::gpio::{Level, Output, OutputConfig};

        let off = if crate::board::LED_ACTIVE_LOW {
            Level::High
        } else {
            Level::Low
        };
        let on = if crate::board::LED_ACTIVE_LOW {
            Level::Low
        } else {
            Level::High
        };
        let pin = Output::new(led_pin, off, OutputConfig::default());
        log::info!(
            "GPIO LED on GPIO {} ({})",
            crate::board::LED_GPIO,
            crate::board::LED_KIND
        );
        Self { pin, on, off }
    }

    fn set_on(&mut self) {
        self.pin.set_level(self.on);
    }

    fn set_off(&mut self) {
        self.pin.set_level(self.off);
    }
}

#[cfg(feature = "board-devkitc-c6")]
async fn halt() -> ! {
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
