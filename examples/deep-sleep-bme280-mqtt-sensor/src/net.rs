// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Wi-Fi STA via esp-radio + embassy-net (HAL 1.2 embassy_dhcp pattern).

use embassy_futures::select::{Either, select};
use embassy_net::{Config as NetConfig, Stack, StackResources};
use embassy_time::{Duration, Instant, with_timeout};
use esp_hal::rng::Rng;
use esp_radio::wifi::sta::StationConfig;
use esp_radio::wifi::{
    AuthenticationMethodConfig, Config, ControllerConfig, Interface, WifiController,
};
use static_cell::StaticCell;

use crate::mqtt;
use crate::remaining;
use crate::sensor::Sample;

const WIFI_CAP: Duration = Duration::from_secs(10);
const DHCP_CAP: Duration = Duration::from_secs(10);
const TEARDOWN_CAP: Duration = Duration::from_secs(2);

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
    wifi: esp_hal::peripherals::WIFI<'static>,
    sample: &Sample,
    boot: u32,
    c: &Creds,
    deadline: Instant,
) -> bool {
    if remaining(deadline).is_none() {
        log::warn!("Active window expired; skipping Wi-Fi");
        return false;
    }

    let wifi_interface = Interface::station();
    let mut controller = match start_controller(wifi, c) {
        Some(controller) => controller,
        None => return false,
    };

    let ok = run_session(&mut controller, wifi_interface, sample, boot, c, deadline).await;
    teardown(controller).await;
    ok
}

fn start_controller(
    wifi: esp_hal::peripherals::WIFI<'static>,
    c: &Creds,
) -> Option<WifiController<'static>> {
    let ssid = match c.wifi_ssid.try_into() {
        Ok(s) => s,
        Err(_) => {
            log::error!("Wi-Fi SSID rejected");
            return None;
        }
    };
    let password = match c.wifi_password.try_into() {
        Ok(p) => p,
        Err(_) => {
            log::error!("Wi-Fi password rejected");
            return None;
        }
    };

    let station_config = Config::Station(
        StationConfig::default()
            .with_ssid(ssid)
            .with_authentication(AuthenticationMethodConfig::Wpa2Personal(password)),
    );

    log::info!("Starting Wi-Fi STA");
    match WifiController::new(
        wifi,
        ControllerConfig::default().with_initial_config(station_config),
    ) {
        Ok(c) => Some(c),
        Err(_) => {
            log::error!("Wi-Fi controller init failed");
            None
        }
    }
}

/// Poll the embassy-net runner only while the socket work runs, then drop it
/// before the caller drops the Wi-Fi controller.
async fn run_session(
    controller: &mut WifiController<'static>,
    wifi_interface: Interface,
    sample: &Sample,
    boot: u32,
    c: &Creds,
    deadline: Instant,
) -> bool {
    let rng = Rng::new();
    let seed = (u64::from(rng.random()) << 32) | u64::from(rng.random());
    let (stack, mut runner) = embassy_net::new(
        wifi_interface,
        NetConfig::dhcpv4(Default::default()),
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );
    let work = session_work(controller, &stack, sample, boot, c, deadline);
    match select(runner.run(), work).await {
        Either::First(stopped) => stopped,
        Either::Second(ok) => ok,
    }
}

async fn session_work(
    controller: &mut WifiController<'static>,
    stack: &Stack<'static>,
    sample: &Sample,
    boot: u32,
    c: &Creds,
    deadline: Instant,
) -> bool {
    let Some(wifi_wait) = remaining(deadline) else {
        log::warn!("Active window expired; skipping Wi-Fi");
        return false;
    };
    match with_timeout(wifi_wait.min(WIFI_CAP), controller.connect_async()).await {
        Ok(Ok(_)) => log::info!("Wi-Fi associated"),
        Ok(Err(_)) | Err(_) => {
            log::warn!("Wi-Fi associate failed or timed out");
            return false;
        }
    }

    let Some(dhcp_wait) = remaining(deadline) else {
        log::warn!("Active window expired before DHCP");
        return false;
    };
    match with_timeout(dhcp_wait.min(DHCP_CAP), stack.wait_config_up()).await {
        Ok(()) => {
            if let Some(cfg) = stack.config_v4() {
                log::info!("Got IP {}", cfg.address);
            }
        }
        Err(_) => {
            log::warn!("DHCP timed out");
            return false;
        }
    }

    let mut payload = [0u8; 96];
    let Some(n) = mqtt::format_payload(sample, boot, &mut payload) else {
        return false;
    };

    if remaining(deadline).is_none() {
        log::warn!("Active window expired before MQTT");
        return false;
    }

    let ok = mqtt::publish_once(
        *stack,
        c.mqtt_host,
        c.mqtt_port,
        c.mqtt_client_id,
        &payload[..n],
        deadline,
    )
    .await;
    if ok {
        log::info!("MQTT publish ok  topic={}", mqtt::TOPIC);
    } else {
        log::warn!("MQTT publish failed");
    }
    ok
}

async fn teardown(mut controller: WifiController<'static>) {
    if controller.is_connected() {
        let _ = with_timeout(TEARDOWN_CAP, controller.disconnect_async()).await;
    }
    log::info!("radio teardown; sleeping");
    drop(controller);
}
