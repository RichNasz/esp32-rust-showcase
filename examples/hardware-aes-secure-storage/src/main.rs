// AGENT-GENERATED — DO NOT EDIT BY HAND
#![no_std]
#![no_main]

//! Hardware AES-256-CBC + `esp-storage` round-trip for ESP32-C6.

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_backtrace as _;
use esp_hal::aes::AesBackend;
use esp_hal::timer::timg::TimerGroup;
use esp_storage::FlashStorage;

mod board;
mod crypto;
mod led;
mod store;

esp_bootloader_esp_idf::esp_app_desc!();

const SELFTEST: &[u8] = b"hardware AES-256-CBC round-trip";

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    esp_println::logger::init_logger(log::LevelFilter::Info);

    let _ = spawner;
    let led_pin = board::take_status_led();
    log::info!(
        "AES storage — {}, GPIO {}",
        board::LED_KIND,
        board::LED_GPIO
    );

    let mut aes_backend = AesBackend::new(peripherals.AES);
    let _aes_driver = aes_backend.start();

    let mut flash = FlashStorage::new(peripherals.FLASH);
    let pass = run_demo(&mut flash);

    #[cfg(feature = "board-devkitc-c6")]
    led::show_result(peripherals.RMT, led_pin, pass).await;

    #[cfg(feature = "board-xiao-c6")]
    led::show_result(led_pin, pass).await;

    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}

fn run_demo(flash: &mut FlashStorage<'_>) -> bool {
    let (key, generated) = match store::load_or_create_key(flash) {
        Ok(pair) => pair,
        Err(e) => {
            log::error!("key storage failed: {e:?}");
            return false;
        }
    };
    if generated {
        log::info!("key generated, saved to storage");
    } else {
        log::info!("key loaded from storage");
    }

    let pass = match self_test(flash, &key) {
        Ok(()) => {
            log::info!("Self-test: PASS");
            true
        }
        Err(e) => {
            log::error!("Self-test: FAIL ({e})");
            false
        }
    };

    match crypto::throughput_mbps_parts(&key) {
        Ok((whole, milli)) => {
            log::info!("hardware AES throughput {whole}.{milli:03} MB/s");
        }
        Err(e) => log::error!("throughput benchmark failed: {e:?}"),
    }

    pass
}

fn self_test(
    flash: &mut FlashStorage<'_>,
    key: &[u8; crypto::KEY_LEN],
) -> Result<(), &'static str> {
    let mut padded = [0u8; 64];
    let pad_n = crypto::pkcs7_pad(SELFTEST, &mut padded).map_err(|_| "pad")?;
    let mut iv = [0u8; crypto::BLOCK];
    crypto::fill_random(&mut iv);

    let mut ciphertext = [0u8; 64];
    crypto::encrypt_cbc(key, iv, &padded[..pad_n], &mut ciphertext[..pad_n])
        .map_err(|_| "encrypt")?;

    if ciphertext[..SELFTEST.len()] == *SELFTEST {
        return Err("ciphertext matches plaintext");
    }
    log::info!("ciphertext differs from plaintext");

    store::write_blob(flash, &iv, &ciphertext[..pad_n]).map_err(|_| "flash write")?;

    let mut iv_loaded = [0u8; crypto::BLOCK];
    let mut ct_loaded = [0u8; 64];
    let ct_n = store::read_blob(flash, &mut iv_loaded, &mut ct_loaded).map_err(|_| "flash read")?;

    let mut recovered = [0u8; 64];
    crypto::decrypt_cbc(key, iv_loaded, &ct_loaded[..ct_n], &mut recovered[..ct_n])
        .map_err(|_| "decrypt")?;
    let plain = crypto::pkcs7_unpad(&recovered[..ct_n]).map_err(|_| "unpad")?;
    if plain != SELFTEST {
        return Err("round-trip mismatch");
    }
    Ok(())
}

impl core::fmt::Display for board::LedKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            #[cfg(feature = "board-devkitc-c6")]
            board::LedKind::Ws2812 => f.write_str("WS2812 RGB LED"),
            #[cfg(feature = "board-xiao-c6")]
            board::LedKind::GpioActiveLow => f.write_str("GPIO LED (active-LOW)"),
        }
    }
}
