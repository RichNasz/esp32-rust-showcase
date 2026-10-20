// AGENT-GENERATED — DO NOT EDIT BY HAND
//! GPIO / LED / wake-pad constants from board-specs/. Application code must
//! not hardcode pin numbers.

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
    // board-specs/espressif/esp32-c6-devkitc-1-n8.md — WS2812 GPIO 8, BOOT LP GPIO 0
    pub const LED_GPIO: u8 = 8;
    pub const LED_KIND: LedKind = LedKind::Ws2812;
    pub const LP_WAKE_GPIO: u8 = 0;
}

#[cfg(feature = "board-xiao-c6")]
mod spec {
    use super::LedKind;
    // board-specs/seeed/xiao-esp32c6.md — user LED GPIO 15 active LOW.
    // Wake pad is D0 / GPIO 0. BOOT is GPIO 9 and is not an LP GPIO.
    pub const LED_GPIO: u8 = 15;
    pub const LED_KIND: LedKind = LedKind::GpioActiveLow;
    pub const LED_ACTIVE_LOW: bool = true;
    pub const LP_WAKE_GPIO: u8 = 0;
}

pub use spec::{LED_GPIO, LED_KIND, LP_WAKE_GPIO};

#[cfg(feature = "board-xiao-c6")]
pub use spec::LED_ACTIVE_LOW;

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

/// Take the onboard status LED pin.
///
/// # Safety
/// Called once from `main` after `esp_hal::init`. The matching `Peripherals`
/// GPIO field is never used elsewhere in this crate.
pub fn take_status_led() -> AnyPin<'static> {
    unsafe { AnyPin::steal(LED_GPIO) }
}

/// Take the LP wake pad. GPIO 0 on both v1 boards.
///
/// # Safety
/// Called once from `main` after `esp_hal::init`. The matching `Peripherals`
/// GPIO field is never used elsewhere in this crate.
pub fn take_wake_gpio() -> AnyPin<'static> {
    unsafe { AnyPin::steal(LP_WAKE_GPIO) }
}
