// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! HTTPS OTA slot flip for ESP32-C6. Pins come from `board`.

extern crate alloc;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_storage::FlashStorage;

mod board;
mod button;
mod error;
mod https;
mod led;
mod net;
mod ota;
mod tls;

esp_bootloader_esp_idf::esp_app_desc!();

const UART_DRAIN: Duration = Duration::from_millis(50);

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    // DRAM heap for the Wi-Fi blob and embassy-net.
    // TLS record buffers live in `SCRATCH`, not on this heap.
    esp_alloc::heap_allocator!(size: 80 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    esp_println::logger::init_logger(log::LevelFilter::Info);
    log::info!(
        "HTTPS OTA — {}, LED GPIO {} ({:?}), trigger GPIO {}",
        board::BOARD_NAME,
        board::LED_GPIO,
        board::LED_KIND,
        board::TRIGGER_GPIO
    );

    let mut flash = FlashStorage::new(peripherals.FLASH);
    let map = match ota::read_map(&mut flash) {
        Ok(map) => map,
        Err(err) => {
            err.log();
            halt().await;
        }
    };
    log::info!(
        "Booted {} at offset {:#x}",
        map.booted.name(),
        map.booted_offset
    );
    if let Err(err) = ota::prepare(&mut flash, &map) {
        err.log();
    }

    let mut led = led::StatusLed::new(
        #[cfg(feature = "board-devkitc-c6")]
        peripherals.RMT,
        board::take_status_led(),
    )
    .await;

    // `WIFI` moves here before any later use. A failed controller init drops
    // the peripheral; later presses report an association error.
    let mut radio = net::RadioHost::new(peripherals.WIFI);
    let scratch = scratch_buf();
    let url = env!("OTA_URL");
    let creds = net::creds();

    loop {
        led.show_slot(ota::colour(map.booted)).await;
        log::info!("Waiting for BOOT press");
        button::wait_for_new_press().await;
        if !net::wifi_configured(&creds) {
            log::warn!("Wi-Fi SSID not configured (compile-time env); staying in wait");
            continue;
        }
        let stack = match radio.connect(spawner).await {
            Ok(stack) => stack,
            Err(err) => {
                err.log();
                led.show_error().await;
                radio.disconnect().await;
                continue;
            }
        };
        match ota::run(&mut flash, &map, stack, scratch, &mut led, url).await {
            Ok(()) => reboot().await,
            Err(err) => {
                err.log();
                led.show_error().await;
                radio.disconnect().await;
            }
        }
    }
}

fn scratch_buf() -> &'static mut https::Scratch {
    static mut SCRATCH: https::Scratch = https::Scratch::new();
    // The main task is the only caller, and each transfer runs to completion.
    unsafe { &mut *core::ptr::addr_of_mut!(SCRATCH) }
}

async fn reboot() -> ! {
    log::info!("Rebooting");
    Timer::after(UART_DRAIN).await;
    esp_hal::system::software_reset()
}

async fn halt() -> ! {
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
