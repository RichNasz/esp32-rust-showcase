// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Wi-Fi STA via esp-radio + embassy-net (HAL 1.2 embassy_dhcp pattern).

use embassy_executor::Spawner;
use embassy_net::{Config as NetConfig, Runner, StackResources};
use embassy_time::{Duration, with_timeout};
use esp_hal::rng::Rng;
use esp_radio::wifi::sta::StationConfig;
use esp_radio::wifi::{
    AuthenticationMethodConfig, Config, ControllerConfig, Interface, WifiController,
};
use static_cell::StaticCell;

use crate::mqtt;
use crate::sensor::Sample;

const WIFI_TIMEOUT: Duration = Duration::from_secs(10);
const DHCP_TIMEOUT: Duration = Duration::from_secs(10);

macro_rules! mk_static {
    ($t:ty, $val:expr) => {{
        static STATIC_CELL: StaticCell<$t> = StaticCell::new();
        STATIC_CELL.uninit().write($val)
    }};
}

/// Compile-time credentials. Values come from env / `.cargo/config.toml` `[env]`.
pub struct Creds {
    pub wifi_ssid: &'static str,
    pub wifi_password: &'static str,
    pub mqtt_host: &'static str,
    pub mqtt_port: u16,
    pub mqtt_client_id: &'static str,
}

pub fn creds() -> Creds {
    let port = env!("MQTT_BROKER_PORT").parse().unwrap_or(1883);
    Creds {
        wifi_ssid: env!("WIFI_SSID"),
        wifi_password: env!("WIFI_PASSWORD"),
        mqtt_host: env!("MQTT_BROKER_HOST"),
        mqtt_port: port,
        mqtt_client_id: env!("MQTT_CLIENT_ID"),
    }
}

pub fn wifi_configured(c: &Creds) -> bool {
    !c.wifi_ssid.is_empty() && c.wifi_ssid != "unset"
}

pub async fn publish_sample(
    spawner: Spawner,
    wifi: esp_hal::peripherals::WIFI<'static>,
    sample: &Sample,
    boot: u32,
    c: &Creds,
) -> bool {
    let ssid = match c.wifi_ssid.try_into() {
        Ok(s) => s,
        Err(_) => {
            log::error!("Wi-Fi SSID rejected");
            return false;
        }
    };
    let password = match c.wifi_password.try_into() {
        Ok(p) => p,
        Err(_) => {
            log::error!("Wi-Fi password rejected");
            return false;
        }
    };

    let station_config = Config::Station(
        StationConfig::default()
            .with_ssid(ssid)
            .with_authentication(AuthenticationMethodConfig::Wpa2Personal(password)),
    );

    log::info!("Starting Wi-Fi STA");
    let wifi_interface = Interface::station();
    let mut controller = match WifiController::new(
        wifi,
        ControllerConfig::default().with_initial_config(station_config),
    ) {
        Ok(c) => c,
        Err(_) => {
            log::error!("Wi-Fi controller init failed");
            return false;
        }
    };

    let rng = Rng::new();
    let seed = (u64::from(rng.random()) << 32) | u64::from(rng.random());
    let (stack, runner) = embassy_net::new(
        wifi_interface,
        NetConfig::dhcpv4(Default::default()),
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    let token = match net_task(runner) {
        Ok(token) => token,
        Err(_) => {
            log::error!("net task spawn failed");
            return false;
        }
    };
    spawner.spawn(token);

    match with_timeout(WIFI_TIMEOUT, controller.connect_async()).await {
        Ok(Ok(_)) => log::info!("Wi-Fi associated"),
        Ok(Err(_)) | Err(_) => {
            log::warn!("Wi-Fi associate failed or timed out");
            teardown(controller).await;
            return false;
        }
    }

    match with_timeout(DHCP_TIMEOUT, stack.wait_config_up()).await {
        Ok(()) => {
            if let Some(cfg) = stack.config_v4() {
                log::info!("Got IP {}", cfg.address);
            }
        }
        Err(_) => {
            log::warn!("DHCP timed out");
            teardown(controller).await;
            return false;
        }
    }

    let mut payload = [0u8; 96];
    let n = match mqtt::format_payload(sample, boot, &mut payload) {
        Some(n) => n,
        None => {
            teardown(controller).await;
            return false;
        }
    };

    let ok = mqtt::publish_once(
        stack,
        c.mqtt_host,
        c.mqtt_port,
        c.mqtt_client_id,
        &payload[..n],
    )
    .await;
    if ok {
        log::info!("MQTT publish ok  topic={}", mqtt::TOPIC);
    } else {
        log::warn!("MQTT publish failed");
    }

    teardown(controller).await;
    ok
}

async fn teardown(mut controller: WifiController<'static>) {
    if controller.is_connected() {
        let _ = with_timeout(Duration::from_secs(2), controller.disconnect_async()).await;
    }
    log::info!("radio teardown; sleeping");
    drop(controller);
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface>) {
    runner.run().await
}
