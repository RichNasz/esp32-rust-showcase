// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! Duty-cycled BLE beacon + deep sleep for ESP32-C6. Pins from `board`.

use core::ptr::addr_of_mut;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_alloc::heap_allocator;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::ram;
use esp_metadata_generated::memory_range;
use esp_hal::gpio::{Event, Input, InputConfig, Pull, WakeupConfig};
use esp_hal::rng::Rng;
use esp_hal::rtc_cntl::sleep::{LowPower, RtcSleepConfig};
use esp_hal::rtc_cntl::{WakeupSource, wakeup_cause};
use esp_hal::time::{Duration as HalDuration, Instant as HalInstant};
use esp_hal::timer::timg::TimerGroup;
use trouble_host::prelude::Address;

mod beacon;
mod board;
mod led;

esp_bootloader_esp_idf::esp_app_desc!();

/// BLE controller blob plus TrouBLE. Part of this budget is reclaimed DRAM.
const HEAP_SIZE: usize = 72 * 1024;
const RECLAIMED_RAM: usize =
    memory_range!("DRAM2_UNINIT").end - memory_range!("DRAM2_UNINIT").start;
const _: () = assert!(HEAP_SIZE > RECLAIMED_RAM);

const SLEEP_SECS: u64 = 10;
const UART_DRAIN_MS: u64 = 20;
const PERSIST_MAGIC: u32 = 0xC6BE_AC01;
const PERSIST_MAGIC_IDX: usize = 0;
const PERSIST_BOOT_IDX: usize = 1;
const PERSIST_SEQ_IDX: usize = 2;
const PERSIST_ADDR_LO_IDX: usize = 3;
const PERSIST_ADDR_HI_IDX: usize = 4;
const PERSIST_CHECK_IDX: usize = 5;

#[esp_hal::ram(unstable(rtc_fast, persistent))]
static mut BOOT_PERSIST: [u32; 6] = [0; 6];

struct Persist {
    boot_count: u16,
    sequence: u16,
    address: Address,
}

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));

    // Both regions join one heap. The descriptor above is what allows `ram(reclaimed)`.
    heap_allocator!(size: HEAP_SIZE - RECLAIMED_RAM);
    heap_allocator!(#[ram(reclaimed)] size: RECLAIMED_RAM);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);
    esp_println::logger::init_logger(log::LevelFilter::Info);

    let _ = spawner;
    let persist = take_persist();
    log_wakeup(persist.boot_count);
    log::info!(
        "BLE beacon — {}, GPIO {}, wake GPIO {}, sleep {} s",
        board::LED_KIND,
        board::LED_GPIO,
        board::WAKE_GPIO,
        SLEEP_SECS
    );

    let mut led = led::StatusLed::new(
        #[cfg(feature = "board-devkitc-c6")]
        peripherals.RMT,
        board::take_status_led(),
    )
    .await;

    beacon::run_window(
        peripherals.BT,
        persist.address,
        persist.boot_count,
        persist.sequence,
        &mut led,
    )
    .await;
    led.off().await;

    arm_gpio_wakeup(board::take_wake_button());
    log::info!("Sleeping for {SLEEP_SECS} s");
    Timer::after(Duration::from_millis(UART_DRAIN_MS)).await;

    let mut lp = LowPower::new(peripherals.LPWR);
    lp.set_wakeup_deadline(HalInstant::now() + HalDuration::from_secs(SLEEP_SECS));
    lp.sleep_deep(RtcSleepConfig::deep());
}

fn persist_state() -> &'static mut [u32; 6] {
    // Single-core; called once per reset from `main` before any other user of this static.
    unsafe { &mut *addr_of_mut!(BOOT_PERSIST) }
}

fn take_persist() -> Persist {
    let state = persist_state();
    let magic = state[PERSIST_MAGIC_IDX];
    let boot = state[PERSIST_BOOT_IDX];
    let seq = state[PERSIST_SEQ_IDX];
    let addr_lo = state[PERSIST_ADDR_LO_IDX];
    let addr_hi = state[PERSIST_ADDR_HI_IDX];
    let check = state[PERSIST_CHECK_IDX];
    let valid = magic == PERSIST_MAGIC && check == (magic ^ boot ^ seq ^ addr_lo ^ addr_hi);

    let (next_boot, next_seq, addr_bytes) = if valid {
        (
            boot.wrapping_add(1),
            seq.wrapping_add(1),
            addr_from_words(addr_lo, addr_hi),
        )
    } else {
        (1, 1, random_static_addr())
    };

    let (lo, hi) = addr_to_words(addr_bytes);
    state[PERSIST_MAGIC_IDX] = PERSIST_MAGIC;
    state[PERSIST_BOOT_IDX] = next_boot;
    state[PERSIST_SEQ_IDX] = next_seq;
    state[PERSIST_ADDR_LO_IDX] = lo;
    state[PERSIST_ADDR_HI_IDX] = hi;
    state[PERSIST_CHECK_IDX] = PERSIST_MAGIC ^ next_boot ^ next_seq ^ lo ^ hi;

    Persist {
        boot_count: next_boot as u16,
        sequence: next_seq as u16,
        address: Address::random(addr_bytes),
    }
}

fn addr_from_words(lo: u32, hi: u32) -> [u8; 6] {
    let lo_b = lo.to_le_bytes();
    let hi_b = hi.to_le_bytes();
    [lo_b[0], lo_b[1], lo_b[2], lo_b[3], hi_b[0], hi_b[1]]
}

fn addr_to_words(bytes: [u8; 6]) -> (u32, u32) {
    let lo = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let hi = u32::from_le_bytes([bytes[4], bytes[5], 0, 0]);
    (lo, hi)
}

fn random_static_addr() -> [u8; 6] {
    let rng = Rng::new();
    let a = rng.random().to_le_bytes();
    let b = rng.random().to_le_bytes();
    let mut bytes = [a[0], a[1], a[2], a[3], b[0], b[1]];
    // BLE static random: two MSBs of the most significant octet must be 1.
    bytes[5] |= 0xC0;
    bytes
}

fn log_wakeup(boot_count: u16) {
    let cause = wakeup_cause();
    if cause.is_empty() {
        log::info!("First boot — counter starts at {boot_count} (resets on power-cycle)");
    } else if cause.contains(WakeupSource::Timer) {
        log::info!("Wake #{boot_count} — timer");
    } else if cause.contains(WakeupSource::Gpio) {
        log::info!("Wake #{boot_count} — GPIO");
    } else {
        log::warn!("Wake #{boot_count} — unexpected wakeup {cause:?}");
    }
}

fn arm_gpio_wakeup(pin: esp_hal::gpio::AnyPin<'static>) {
    let cfg = InputConfig::default().with_pull(Pull::Up);
    let mut button = Input::new(pin, cfg);
    button.listen(Event::LowLevel);
    let wake_cfg = WakeupConfig::default().with_low_power_path(true);
    if button.apply_wakeup_config(&wake_cfg).is_err() {
        log::warn!("GPIO {} is not an RTC wake pad", board::WAKE_GPIO);
    }
}
