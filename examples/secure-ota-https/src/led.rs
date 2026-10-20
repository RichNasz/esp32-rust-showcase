// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Slot colour on the DevKitC WS2812, blink count on the XIAO GPIO LED.
//! WS2812 is GRB, 800 kHz NRZ, brightness cap 64/255. A GPIO level on the
//! DevKit data pin does not light the LED.

use embassy_time::{Duration, Timer};
use esp_hal::gpio::AnyPin;

#[cfg(feature = "board-devkitc-c6")]
const BRIGHTNESS_CAP: u8 = 64;

/// Which running slot the LED should report.
#[derive(Clone, Copy, Debug)]
pub enum SlotColour {
    Ota0,
    Ota1,
    Other,
}

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

    pub async fn show_slot(&mut self, kind: SlotColour) {
        #[cfg(feature = "board-devkitc-c6")]
        self.show_slot_ws(kind).await;
        #[cfg(feature = "board-xiao-c6")]
        self.show_slot_gpio(kind).await;
    }

    pub async fn pulse_transfer(&mut self, kind: SlotColour) {
        #[cfg(feature = "board-devkitc-c6")]
        {
            self.ws.set(0, 0, 0).await;
            Timer::after(Duration::from_millis(40)).await;
            let (r, g, b) = slot_rgb(kind);
            self.ws.set(r, g, b).await;
            Timer::after(Duration::from_millis(40)).await;
        }
        #[cfg(feature = "board-xiao-c6")]
        {
            let _ = kind;
            self.gpio.blink_once(40, 40).await;
        }
    }

    pub async fn show_error(&mut self) {
        #[cfg(feature = "board-devkitc-c6")]
        self.blink_red(3, 200).await;
        #[cfg(feature = "board-xiao-c6")]
        self.gpio.blink(3, 400, 400).await;
    }
}

#[cfg(feature = "board-devkitc-c6")]
impl StatusLed {
    async fn show_slot_ws(&mut self, kind: SlotColour) {
        match kind {
            SlotColour::Other => self.blink_red(3, 200).await,
            slot => {
                let (r, g, b) = slot_rgb(slot);
                self.ws.set(r, g, b).await;
            }
        }
    }

    async fn blink_red(&mut self, count: u8, ms: u64) {
        let mut n = 0;
        while n < count {
            self.ws.set(BRIGHTNESS_CAP, 0, 0).await;
            Timer::after(Duration::from_millis(ms)).await;
            self.ws.set(0, 0, 0).await;
            Timer::after(Duration::from_millis(ms)).await;
            n += 1;
        }
    }
}

#[cfg(feature = "board-xiao-c6")]
impl StatusLed {
    async fn show_slot_gpio(&mut self, kind: SlotColour) {
        // FunctionalSpec: the blink count is the slot id, so it is 1 or 2.
        let count = match kind {
            SlotColour::Ota0 => 1,
            SlotColour::Ota1 => 2,
            SlotColour::Other => 3,
        };
        self.gpio.blink(count, 120, 180).await;
    }
}

#[cfg(feature = "board-devkitc-c6")]
fn slot_rgb(kind: SlotColour) -> (u8, u8, u8) {
    match kind {
        SlotColour::Ota0 => (0, 0, BRIGHTNESS_CAP),
        // Amber, still at or under the 64/255 cap.
        SlotColour::Ota1 => (BRIGHTNESS_CAP, 36, 0),
        SlotColour::Other => (BRIGHTNESS_CAP, 0, 0),
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
            "WS2812 RMT init on GPIO {} (GRB order, 800 kHz NRZ, max brightness {}/255)",
            crate::board::LED_GPIO,
            BRIGHTNESS_CAP
        );
        write_ws2812(&mut channel, 0, 0, 0, &timing).await;
        Self { channel, timing }
    }

    async fn set(&mut self, r: u8, g: u8, b: u8) {
        write_ws2812(&mut self.channel, r, g, b, &self.timing).await;
    }
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

#[cfg(feature = "board-devkitc-c6")]
async fn halt() -> ! {
    loop {
        Timer::after(Duration::from_secs(1)).await;
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
            "GPIO LED on GPIO {} (active low)",
            crate::board::LED_GPIO
        );
        Self { pin, on, off }
    }

    async fn blink(&mut self, count: u8, on_ms: u64, off_ms: u64) {
        let mut n = 0;
        while n < count {
            self.blink_once(on_ms, off_ms).await;
            n += 1;
        }
    }

    async fn blink_once(&mut self, on_ms: u64, off_ms: u64) {
        self.pin.set_level(self.on);
        Timer::after(Duration::from_millis(on_ms)).await;
        self.pin.set_level(self.off);
        Timer::after(Duration::from_millis(off_ms)).await;
    }
}
