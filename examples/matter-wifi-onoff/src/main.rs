// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]
#![recursion_limit = "256"]

//! Matter on/off light over Wi-Fi for ESP32-C6. Commissioning is BLE.
//! Not Matter over Thread and not a certified product. Pins come from `board`.

extern crate alloc;
use tinyrlibc as _;

use embassy_executor::Spawner;
use esp_alloc::heap_allocator;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::ram;
use esp_hal::timer::timg::TimerGroup;
use esp_metadata_generated::memory_range;

mod board;
mod led;
mod light;

esp_bootloader_esp_idf::esp_app_desc!();

/// Rust heap for the Wi-Fi and BLE blobs and for X.509. OpenThread is not linked.
const HEAP_SIZE: usize = 100 * 1024;
const RECLAIMED_RAM: usize =
    memory_range!("DRAM2_UNINIT").end - memory_range!("DRAM2_UNINIT").start;

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // The reclaimed region is the second DRAM block. The first allocator covers the rest.
    heap_allocator!(size: HEAP_SIZE - RECLAIMED_RAM);
    heap_allocator!(#[ram(reclaimed)] size: RECLAIMED_RAM);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    esp_println::logger::init_logger(log::LevelFilter::Info);

    log::info!(
        "Matter on/off light — {} GPIO {}",
        board::LED_KIND,
        board::LED_GPIO
    );

    let led = led::StatusLed::new(
        #[cfg(feature = "board-devkitc-c6")]
        peripherals.RMT,
        board::take_status_led(),
    )
    .await;

    light::run(
        peripherals.WIFI,
        peripherals.BT,
        peripherals.RNG,
        peripherals.ADC1,
        led,
    )
    .await
}
