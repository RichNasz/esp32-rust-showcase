// AGENT-GENERATED — DO NOT EDIT BY HAND
//! GPIO / LED / I2C / wake-button constants generated from board-specs/.
//! Application code must not hardcode pin numbers.

use esp_hal::gpio::AnyPin;

#[cfg(all(feature = "board-devkitc-c6", feature = "board-xiao-c6"))]
compile_error!(
    "board features are mutually exclusive: enable only one of board-devkitc-c6, board-xiao-c6"
);

#[cfg(not(any(feature = "board-devkitc-c6", feature = "board-xiao-c6")))]
compile_error!("select a board feature: board-devkitc-c6 (default) or board-xiao-c6");

/// Kind of onboard status LED.
#[derive(Clone, Copy, Debug)]
pub enum LedKind {
    #[cfg(feature = "board-devkitc-c6")]
    Ws2812,
    #[cfg(feature = "board-xiao-c6")]
    GpioActiveLow,
}

#[cfg(feature = "board-devkitc-c6")]
mod spec {
    use super::LedKind;
    // board-specs/espressif/esp32-c6-devkitc-1-n8.md
    pub const LED_GPIO: u8 = 8;
    pub const LED_KIND: LedKind = LedKind::Ws2812;
    pub const I2C_SDA_GPIO: u8 = 6;
    pub const I2C_SCL_GPIO: u8 = 7;
    // Boot button; RTC-capable (LP GPIO 0). Active LOW.
    pub const WAKE_GPIO: u8 = 0;
}

#[cfg(feature = "board-xiao-c6")]
mod spec {
    use super::LedKind;
    // board-specs/seeed/xiao-esp32c6.md
    pub const LED_GPIO: u8 = 15;
    pub const LED_KIND: LedKind = LedKind::GpioActiveLow;
    pub const LED_ACTIVE_LOW: bool = true;
    // D4 / D5 default I2C
    pub const I2C_SDA_GPIO: u8 = 22;
    pub const I2C_SCL_GPIO: u8 = 23;
    // D0 is RTC-capable. Onboard BOOT is GPIO 9 and cannot wake from deep sleep.
    pub const WAKE_GPIO: u8 = 0;
}

pub use spec::{I2C_SCL_GPIO, I2C_SDA_GPIO, LED_GPIO, LED_KIND, WAKE_GPIO};

#[cfg(feature = "board-xiao-c6")]
pub use spec::LED_ACTIVE_LOW;

/// # Safety
/// Called once from `main` after `esp_hal::init`. The matching `Peripherals`
/// GPIO field is never used elsewhere in this crate.
pub fn take_status_led() -> AnyPin<'static> {
    unsafe { AnyPin::steal(LED_GPIO) }
}

/// # Safety
/// See [`take_status_led`].
pub fn take_i2c_sda() -> AnyPin<'static> {
    unsafe { AnyPin::steal(I2C_SDA_GPIO) }
}

/// # Safety
/// See [`take_status_led`].
pub fn take_i2c_scl() -> AnyPin<'static> {
    unsafe { AnyPin::steal(I2C_SCL_GPIO) }
}

/// # Safety
/// See [`take_status_led`].
pub fn take_wake_button() -> AnyPin<'static> {
    unsafe { AnyPin::steal(WAKE_GPIO) }
}

impl core::fmt::Display for LedKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            #[cfg(feature = "board-devkitc-c6")]
            LedKind::Ws2812 => f.write_str("WS2812 RGB LED"),
            #[cfg(feature = "board-xiao-c6")]
            LedKind::GpioActiveLow => f.write_str("GPIO LED (active-LOW)"),
        }
    }
}
