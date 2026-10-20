// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Wi-Fi STA via esp-radio + embassy-net. One controller is kept for every
//! BOOT press. `WIFI` is moved here before any other code can touch it.

use embassy_executor::Spawner;
use embassy_net::{Config as NetConfig, Runner, Stack, StackResources};
use embassy_time::{Duration, with_timeout};
use esp_hal::rng::Rng;
use esp_radio::wifi::sta::StationConfig;
use esp_radio::wifi::{
    AuthenticationMethodConfig, Config, ControllerConfig, Interface, WifiController,
};
use static_cell::StaticCell;

use crate::error::StageError;

const WIFI_CAP: Duration = Duration::from_secs(15);
const DHCP_CAP: Duration = Duration::from_secs(10);
const TEARDOWN_CAP: Duration = Duration::from_secs(2);

macro_rules! mk_static {
    ($t:ty, $val:expr) => {{
        static STATIC_CELL: StaticCell<$t> = StaticCell::new();
        STATIC_CELL.uninit().write($val)
    }};
}

pub struct Creds {
    pub wifi_ssid: &'static str,
    pub wifi_password: &'static str,
}

pub fn creds() -> Creds {
    Creds {
        wifi_ssid: env!("WIFI_SSID"),
        wifi_password: env!("WIFI_PASSWORD"),
    }
}

pub fn wifi_configured(c: &Creds) -> bool {
    !c.wifi_ssid.is_empty() && c.wifi_ssid != "unset"
}

struct Link {
    controller: WifiController<'static>,
    stack: Stack<'static>,
}

pub struct RadioHost {
    wifi: Option<esp_hal::peripherals::WIFI<'static>>,
    link: Option<Link>,
}

impl RadioHost {
    pub fn new(wifi: esp_hal::peripherals::WIFI<'static>) -> Self {
        Self {
            wifi: Some(wifi),
            link: None,
        }
    }

    pub async fn connect(&mut self, spawner: Spawner) -> Result<Stack<'static>, StageError> {
        if self.link.is_none() {
            self.start(spawner)?;
        }
        let Some(link) = self.link.as_mut() else {
            return Err(StageError::WifiAssoc);
        };
        let c = creds();
        associate(link, c.wifi_ssid, c.wifi_password).await?;
        Ok(link.stack)
    }

    pub async fn disconnect(&mut self) {
        let Some(link) = self.link.as_mut() else {
            return;
        };
        if link.controller.is_connected() {
            let _ = with_timeout(TEARDOWN_CAP, link.controller.disconnect_async()).await;
        }
        log::info!("Wi-Fi stopped");
    }

    fn start(&mut self, spawner: Spawner) -> Result<(), StageError> {
        let Some(wifi) = self.wifi.take() else {
            return Err(StageError::WifiAssoc);
        };
        self.link = Some(bring_up(wifi, spawner)?);
        Ok(())
    }
}

fn bring_up(
    wifi: esp_hal::peripherals::WIFI<'static>,
    spawner: Spawner,
) -> Result<Link, StageError> {
    let c = creds();
    let controller = start_controller(wifi, c.wifi_ssid, c.wifi_password)?;
    let rng = Rng::new();
    let seed = (u64::from(rng.random()) << 32) | u64::from(rng.random());
    let (stack, runner) = embassy_net::new(
        Interface::station(),
        NetConfig::dhcpv4(Default::default()),
        mk_static!(StackResources<4>, StackResources::<4>::new()),
        seed,
    );
    match net_task(runner) {
        Ok(token) => spawner.spawn(token),
        Err(_) => {
            log::error!("net task spawn failed");
            return Err(StageError::WifiAssoc);
        }
    }
    Ok(Link { controller, stack })
}

fn start_controller(
    wifi: esp_hal::peripherals::WIFI<'static>,
    ssid: &str,
    password: &str,
) -> Result<WifiController<'static>, StageError> {
    let ssid = match ssid.try_into() {
        Ok(ssid) => ssid,
        Err(_) => return Err(StageError::WifiCreds),
    };
    let password = match password.try_into() {
        Ok(password) => password,
        Err(_) => return Err(StageError::WifiCreds),
    };
    let station = Config::Station(
        StationConfig::default()
            .with_ssid(ssid)
            .with_authentication(AuthenticationMethodConfig::Wpa2Personal(password)),
    );
    log::info!("Starting Wi-Fi STA");
    match WifiController::new(
        wifi,
        ControllerConfig::default().with_initial_config(station),
    ) {
        Ok(controller) => Ok(controller),
        Err(_) => Err(StageError::WifiAssoc),
    }
}

async fn associate(link: &mut Link, ssid: &str, _password: &str) -> Result<(), StageError> {
    if link.controller.is_connected() {
        let _ = with_timeout(TEARDOWN_CAP, link.controller.disconnect_async()).await;
    }
    match with_timeout(WIFI_CAP, link.controller.connect_async()).await {
        Ok(Ok(_)) => log::info!("Wi-Fi associated ssid={ssid}"),
        Ok(Err(_)) | Err(_) => return Err(StageError::WifiAssoc),
    }
    match with_timeout(DHCP_CAP, link.stack.wait_config_up()).await {
        Ok(()) => {
            if let Some(cfg) = link.stack.config_v4() {
                log::info!("Got IP {}", cfg.address);
            }
            Ok(())
        }
        Err(_) => Err(StageError::WifiDhcp),
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface>) {
    runner.run().await
}
