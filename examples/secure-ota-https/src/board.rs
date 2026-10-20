// AGENT-GENERATED — DO NOT EDIT BY HAND
//! GPIO constants from board-specs. Application code must not hardcode pins.
//! XIAO GPIO 3 and GPIO 14 are the RF switch; this crate does not drive them.

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
    pub const BOARD_NAME: &str = "ESP32-C6-DevKitC-1-N8";
    pub const LED_GPIO: u8 = 8;
    pub const LED_KIND: LedKind = LedKind::Ws2812;
    // BOOT, active LOW, RTC-capable. This example never sleeps.
    pub const TRIGGER_GPIO: u8 = 0;
    pub const TRIGGER_ACTIVE_LOW: bool = true;
}

#[cfg(feature = "board-xiao-c6")]
mod spec {
    use super::LedKind;
    // board-specs/seeed/xiao-esp32c6.md
    // Onboard BOOT is GPIO 9. GPIO 0 is D0 and is not the BOOT button.
    pub const BOARD_NAME: &str = "XIAO ESP32-C6";
    pub const LED_GPIO: u8 = 15;
    pub const LED_KIND: LedKind = LedKind::GpioActiveLow;
    pub const LED_ACTIVE_LOW: bool = true;
    pub const TRIGGER_GPIO: u8 = 9;
    pub const TRIGGER_ACTIVE_LOW: bool = true;
}

pub use spec::{BOARD_NAME, LED_GPIO, LED_KIND, TRIGGER_ACTIVE_LOW, TRIGGER_GPIO};

#[cfg(feature = "board-xiao-c6")]
pub use spec::LED_ACTIVE_LOW;

/// # Safety
/// Called once from `main` after `esp_hal::init`. The matching `Peripherals`
/// GPIO field is never used elsewhere in this crate.
pub fn take_status_led() -> AnyPin<'static> {
    unsafe { AnyPin::steal(LED_GPIO) }
}

/// # Safety
/// See [`take_status_led`]. Each wait drops the previous `Input` before the
/// next call.
pub fn take_trigger() -> AnyPin<'static> {
    unsafe { AnyPin::steal(TRIGGER_GPIO) }
}
