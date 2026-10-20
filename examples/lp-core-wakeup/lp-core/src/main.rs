// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! LP-core image. Watches LP GPIO 0 (active LOW). Both v1 boards use that pad.
//! GPIO 9 is not an LP GPIO and is not referenced here.
//!
//! Handshake word: CodingSpec, C6 LP RAM. The address sits above a small image
//! and below the stack, which grows down from the top of the 16 KiB region.

use core::sync::atomic::{Ordering, compiler_fence};

use esp_lp_hal::delay::Delay;
use esp_lp_hal::gpio::{Input, Level};
use esp_lp_hal::prelude::*;
use esp_lp_hal::wake_hp_core;

/// C6 LP RAM base is `0x5000_0000`. This word is not in the loaded image.
const HANDSHAKE_ADDR: u32 = 0x5000_2000;
/// Distinctive "GPIO edge seen" pattern. Zero means no edge.
const HANDSHAKE_GPIO_EDGE: u32 = 0x4750_494F;
/// Long enough to drop sub-millisecond noise, short enough to meet the wake budget.
const DEBOUNCE_MS: u32 = 2;

#[entry]
fn main(gpio0: Input<0>) -> ! {
    let handshake = HANDSHAKE_ADDR as *mut u32;
    unsafe {
        handshake.write_volatile(0);
    }

    loop {
        if gpio0.level() == Level::Low {
            Delay.delay_millis(DEBOUNCE_MS);
            if gpio0.level() == Level::Low {
                unsafe {
                    handshake.write_volatile(HANDSHAKE_GPIO_EDGE);
                }
                wake_hp_core();
                // HP reloads this image before the next sleep. A stuck line
                // wakes again after that reload instead of spinning on the HP core.
                loop {
                    compiler_fence(Ordering::SeqCst);
                }
            }
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}
