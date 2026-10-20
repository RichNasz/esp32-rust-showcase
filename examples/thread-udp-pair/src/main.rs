// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! Two-role Thread UDP lab link for ESP32-C6. Not a border router, not Zigbee,
//! and not Matter. Pins come from `board`. One image, two compile-time roles.

extern crate alloc;
use tinyrlibc as _;

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;

mod board;
mod frame;
mod led;
mod radio;

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(all(feature = "role-leader", feature = "role-child"))]
compile_error!("role features are mutually exclusive: enable only one of role-leader, role-child");

#[cfg(not(any(feature = "role-leader", feature = "role-child")))]
compile_error!("select a role feature: role-child (default) or role-leader");

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // Small Rust heap. OpenThread keeps its own internal heap.
    esp_alloc::heap_allocator!(size: 8 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    esp_println::logger::init_logger(log::LevelFilter::Info);

    log::info!(
        "Thread {} — {} GPIO {}",
        role_name(),
        board::LED_KIND,
        board::LED_GPIO
    );

    let mut led = led::StatusLed::new(
        #[cfg(feature = "board-devkitc-c6")]
        peripherals.RMT,
        board::take_status_led(),
    )
    .await;

    radio::run(
        spawner,
        peripherals.IEEE802154,
        peripherals.RNG,
        peripherals.ADC1,
        &mut led,
    )
    .await
}

#[cfg(feature = "role-leader")]
fn role_name() -> &'static str {
    "leader"
}

#[cfg(feature = "role-child")]
fn role_name() -> &'static str {
    "child"
}
