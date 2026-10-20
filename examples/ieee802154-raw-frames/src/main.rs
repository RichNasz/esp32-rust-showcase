// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! Raw IEEE 802.15.4 node/peer pair for ESP32-C6. Not Thread and not Zigbee.
//! Pins come from `board`. One image, two compile-time roles.

extern crate alloc;

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;

mod board;
mod frame;
mod led;
mod radio;

esp_bootloader_esp_idf::esp_app_desc!();

#[cfg(all(feature = "role-node", feature = "role-peer"))]
compile_error!("role features are mutually exclusive: enable only one of role-node, role-peer");

#[cfg(not(any(feature = "role-node", feature = "role-peer")))]
compile_error!("select a role feature: role-node (default) or role-peer");

#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // DRAM heap for decoded 802.15.4 payloads. About 72 KiB. No image-sized buffers.
    esp_alloc::heap_allocator!(size: 72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    esp_println::logger::init_logger(log::LevelFilter::Info);

    log::info!(
        "802.15.4 {} — {} GPIO {}",
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

    #[cfg(feature = "role-node")]
    radio::node_forever(peripherals.IEEE802154, &mut led).await;

    #[cfg(feature = "role-peer")]
    radio::peer_forever(peripherals.IEEE802154, &mut led).await;
}

#[cfg(feature = "role-node")]
fn role_name() -> &'static str {
    "node"
}

#[cfg(feature = "role-peer")]
fn role_name() -> &'static str {
    "peer"
}
