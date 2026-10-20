// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Lab dataset, OpenThread bring-up, and the UDP loop.
//! Wi-Fi stays off. Settings are RAM-only, so a reset rejoins from the dataset.

use core::net::{Ipv6Addr, SocketAddrV6};

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::peripherals::{ADC1, IEEE802154, RNG};
use esp_hal::rng::{Trng, TrngSource};
use esp_radio::ieee802154::Ieee802154;
use openthread::esp::EspRadio;
use openthread::{
    DeviceRole, OpenThread, OtResources, OtUdpResources, SimpleRamSettings, UdpSocket,
};
use rand_core::Rng as _;
use static_cell::StaticCell;

use crate::frame::PAYLOAD;
use crate::led::StatusLed;

const BOUND_PORT: u16 = 1212;
const UDP_SOCKETS_BUF: usize = 1280;
const UDP_MAX_SOCKETS: usize = 2;
const SEND_EVERY: Duration = Duration::from_secs(2);

/// Thread realm-local all-nodes address (`ff03::1`).
const REALM_ALL_NODES: Ipv6Addr = Ipv6Addr::new(0xff03, 0, 0, 0, 0, 0, 0, 1);

const THREAD_DATASET: &str = env!("THREAD_DATASET");

macro_rules! mk_static {
    ($t:ty, $val:expr) => {{
        static CELL: StaticCell<$t> = StaticCell::new();
        CELL.init($val)
    }};
}

/// Bring up Thread and run the role loop. Does not return.
pub async fn run(
    spawner: Spawner,
    radio_peri: IEEE802154<'static>,
    rng_peri: RNG<'static>,
    adc: ADC1<'static>,
    led: &mut StatusLed,
) -> ! {
    // The TRNG is only cryptographically random while this source stays alive.
    let _trng_source = mk_static!(TrngSource<'static>, TrngSource::new(rng_peri, adc));
    let rng = match Trng::try_new() {
        Ok(rng) => mk_static!(Trng, rng),
        Err(_) => {
            log::error!("TRNG init failed");
            halt().await
        }
    };

    let mut ieee_eui64 = [0u8; 8];
    rng.fill_bytes(&mut ieee_eui64);

    let ot_resources = mk_static!(OtResources, OtResources::new());
    let ot_udp_resources = mk_static!(
        OtUdpResources<UDP_MAX_SOCKETS, UDP_SOCKETS_BUF>,
        OtUdpResources::new()
    );
    let ot_settings_buf = mk_static!([u8; 1024], [0u8; 1024]);
    let ot_settings = mk_static!(SimpleRamSettings, SimpleRamSettings::new(ot_settings_buf));

    let ot = match OpenThread::new_with_udp(
        ieee_eui64,
        rng,
        ot_settings,
        ot_resources,
        ot_udp_resources,
    ) {
        Ok(ot) => ot,
        Err(err) => {
            log::error!("OpenThread init failed: {err:?}");
            halt().await
        }
    };

    let radio = EspRadio::new(Ieee802154::new(radio_peri));
    let token = match run_ot(ot.clone(), radio) {
        Ok(token) => token,
        Err(err) => {
            log::error!("OpenThread runner failed to spawn: {err:?}");
            halt().await
        }
    };
    spawner.spawn(token);

    if let Err(err) = ot.set_active_dataset_tlv_hexstr(THREAD_DATASET) {
        log::error!("Thread dataset rejected: {err:?}");
        halt().await
    }
    if let Err(err) = ot.enable_ipv6(true) {
        log::error!("IPv6 enable failed: {err:?}");
        halt().await
    }
    if let Err(err) = ot.enable_thread(true) {
        log::error!("Thread enable failed: {err:?}");
        halt().await
    }

    log::info!(
        "Thread {} dataset loaded channel {} pan {:#06x}",
        role_label(),
        ot.channel(),
        ot.pan_id()
    );

    let socket = match UdpSocket::bind(
        ot.clone(),
        &SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, BOUND_PORT, 0, 0),
    ) {
        Ok(socket) => socket,
        Err(err) => {
            log::error!("UDP bind port {BOUND_PORT} failed: {err:?}");
            halt().await
        }
    };

    #[cfg(feature = "role-leader")]
    let buf = mk_static!([u8; 128], [0u8; 128]);

    #[cfg(feature = "role-leader")]
    leader_loop(&ot, &socket, buf, led).await;

    #[cfg(feature = "role-child")]
    child_loop(&ot, &socket, led).await;
}

#[embassy_executor::task]
async fn run_ot(ot: OpenThread<'static>, radio: EspRadio<'static>) -> ! {
    ot.run(radio).await
}

#[cfg(feature = "role-leader")]
async fn leader_loop(
    ot: &OpenThread<'static>,
    socket: &UdpSocket<'_>,
    buf: &mut [u8],
    led: &mut StatusLed,
) -> ! {
    log::info!("leader bound UDP {BOUND_PORT}, waiting for role leader");
    loop {
        match socket.recv(buf).await {
            Ok((len, _local, remote)) => {
                let role = ot.device_role();
                if role != DeviceRole::Leader {
                    log::warn!("datagram from {remote} while role {role:?}");
                    continue;
                }
                let bytes = &buf[..len];
                if bytes != PAYLOAD {
                    log::info!("ignored {len} bytes from {remote}");
                    continue;
                }
                log::info!(
                    "payload from {remote} channel {} pan {:#06x}",
                    ot.channel(),
                    ot.pan_id()
                );
                led.leader_payload().await;
            }
            Err(err) => log::error!("UDP recv failed: {err:?}"),
        }
    }
}

#[cfg(feature = "role-child")]
async fn child_loop(ot: &OpenThread<'static>, socket: &UdpSocket<'_>, led: &mut StatusLed) -> ! {
    log::info!("child bound UDP {BOUND_PORT}, waiting to attach");
    let dest = SocketAddrV6::new(REALM_ALL_NODES, BOUND_PORT, 0, 0);
    loop {
        if ot.device_role() != DeviceRole::Child {
            log::warn!("not yet child (role {:?})", ot.device_role());
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }
        match socket.send(PAYLOAD, None, &dest).await {
            Ok(()) => {
                log::info!(
                    "sent payload to {dest} channel {} pan {:#06x}",
                    ot.channel(),
                    ot.pan_id()
                );
                led.child_sent().await;
            }
            Err(err) => log::error!("UDP send failed: {err:?}"),
        }
        Timer::after(SEND_EVERY).await;
    }
}

fn role_label() -> &'static str {
    #[cfg(feature = "role-leader")]
    {
        "leader"
    }
    #[cfg(feature = "role-child")]
    {
        "child"
    }
}

async fn halt() -> ! {
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
