// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Status LED. DevKitC is an RMT WS2812 (GRB, cap 64/255). XIAO is an active-LOW
//! GPIO. Leader success is three green flashes. Child send is three blue flashes.
//! Starts dark.

use embassy_time::{Duration, Timer};
use esp_hal::gpio::AnyPin;

#[cfg(feature = "board-devkitc-c6")]
const BRIGHTNESS_CAP: u8 = 64;

const FLASH_MS: u64 = 250;
const GAP_MS: u64 = 80;
const PATTERN_COUNT: u8 = 3;

/// Board status LED. Starts dark.
pub struct StatusLed {
    #[cfg(feature = "board-devkitc-c6")]
    ws: Ws2812Led,
    #[cfg(feature = "board-xiao-c6")]
    gpio: GpioLed,
}

impl StatusLed {
    pub async fn new(
        #[cfg(feature = "board-devkitc-c6")] rmt: esp_hal::peripherals::RMT<'static>,
        led_pin: AnyPin<'static>,
    ) -> Self {
        #[cfg(feature = "board-devkitc-c6")]
        {
            Self {
                ws: Ws2812Led::init(rmt, led_pin).await,
            }
        }
        #[cfg(feature = "board-xiao-c6")]
        {
            Self {
                gpio: GpioLed::init(led_pin),
            }
        }
    }

    /// Leader has logged a matching payload: three green flashes, or three blinks.
    #[cfg(feature = "role-leader")]
    pub async fn leader_payload(&mut self) {
        #[cfg(feature = "board-devkitc-c6")]
        self.ws
            .pulses(0, BRIGHTNESS_CAP, 0, PATTERN_COUNT, GAP_MS)
            .await;
        #[cfg(feature = "board-xiao-c6")]
        self.gpio.pulses(PATTERN_COUNT, GAP_MS).await;
    }

    /// Child attached and sent: three blue flashes, or three blinks.
    #[cfg(feature = "role-child")]
    pub async fn child_sent(&mut self) {
        #[cfg(feature = "board-devkitc-c6")]
        self.ws
            .pulses(0, 0, BRIGHTNESS_CAP, PATTERN_COUNT, GAP_MS)
            .await;
        #[cfg(feature = "board-xiao-c6")]
        self.gpio.pulses(PATTERN_COUNT, GAP_MS).await;
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

        // 80 MHz / clk_divider 8 = 10 MHz → 100 ns ticks. WS2812B ±150 ns band.
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
            Ok(channel) => channel.with_pin(led_pin),
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
        write_ws2812(&mut channel, 0, 0, 0, &timing).await;
        Self { channel, timing }
    }

    async fn pulses(&mut self, red: u8, green: u8, blue: u8, count: u8, gap_ms: u64) {
        let mut sent = 0u8;
        while sent < count {
            self.set(red, green, blue).await;
            Timer::after(Duration::from_millis(FLASH_MS)).await;
            self.set(0, 0, 0).await;
            sent += 1;
            if sent < count {
                Timer::after(Duration::from_millis(gap_ms)).await;
            }
        }
    }

    async fn set(&mut self, red: u8, green: u8, blue: u8) {
        let red = red.min(BRIGHTNESS_CAP);
        let green = green.min(BRIGHTNESS_CAP);
        let blue = blue.min(BRIGHTNESS_CAP);
        write_ws2812(&mut self.channel, red, green, blue, &self.timing).await;
    }
}

#[cfg(feature = "board-devkitc-c6")]
fn encode_grb(red: u8, green: u8, blue: u8, timing: &NrzTiming) -> [esp_hal::rmt::PulseCode; 25] {
    use esp_hal::gpio::Level;
    use esp_hal::rmt::PulseCode;

    let bit0 = PulseCode::new(Level::High, timing.t0h, Level::Low, timing.t0l);
    let bit1 = PulseCode::new(Level::High, timing.t1h, Level::Low, timing.t1l);
    let mut codes = [PulseCode::end_marker(); 25];
    let mut index = 0;
    for byte in [green, red, blue] {
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
    red: u8,
    green: u8,
    blue: u8,
    timing: &NrzTiming,
) {
    let codes = encode_grb(red, green, blue, timing);
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

    async fn pulses(&mut self, count: u8, gap_ms: u64) {
        let mut sent = 0u8;
        while sent < count {
            self.pin.set_level(self.on);
            Timer::after(Duration::from_millis(FLASH_MS)).await;
            self.pin.set_level(self.off);
            sent += 1;
            if sent < count {
                Timer::after(Duration::from_millis(gap_ms)).await;
            }
        }
    }
}

#[cfg(feature = "board-devkitc-c6")]
async fn halt() -> ! {
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
