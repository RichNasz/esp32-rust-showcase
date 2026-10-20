// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Matter on/off light. BLE and Wi-Fi run together. The key store is RAM.
//! Test attestation only. The status LED follows endpoint 1.

use core::pin::pin;

use embassy_time::{Duration, Timer};
use esp_hal::peripherals::{ADC1, BT, RNG, WIFI};
use esp_hal::rng::{Trng, TrngSource};
use rs_matter_embassy::matter::crypto::{default_crypto, Crypto};
use rs_matter_embassy::matter::dm::clusters::app::on_off::test::TestOnOffDeviceLogic;
use rs_matter_embassy::matter::dm::clusters::app::on_off::{self, HandlerAsyncAdaptor, OnOffHooks};
use rs_matter_embassy::matter::dm::clusters::desc::{self, ClusterHandler as _};
use rs_matter_embassy::matter::dm::devices::test::{
    DAC_PRIVKEY, TEST_DEV_ATT, TEST_DEV_COMM, TEST_DEV_DET,
};
use rs_matter_embassy::matter::dm::devices::DEV_TYPE_ON_OFF_LIGHT;
use rs_matter_embassy::matter::dm::endpoints::ROOT_ENDPOINT_ID;
use rs_matter_embassy::matter::dm::{Async, Dataver, EmptyHandler, Endpoint, Node};
use rs_matter_embassy::matter::dm::clusters::gen_diag::NetifDiag;
use rs_matter_embassy::matter::dm::networks::NetChangeNotif;
use rs_matter_embassy::matter::error::Error;
use rs_matter_embassy::matter::pairing::DiscoveryCapabilities;
use rs_matter_embassy::matter::pairing::qr::{no_optional_data, CommFlowType, QrPayload};
use rs_matter_embassy::stack::nal::NetStack;
use rs_matter_embassy::stack::UserTask;
use rs_matter_embassy::matter::persist::DummyKvBlobStore;
use rs_matter_embassy::matter::utils::init::InitMaybeUninit;
use rs_matter_embassy::matter::{clusters, devices};
use rs_matter_embassy::stack::rand::reseeding_csprng;
use rs_matter_embassy::wireless::esp::EspWifiDriver;
use rs_matter_embassy::wireless::{EmbassyWifi, EmbassyWifiMatterStack};
use static_cell::StaticCell;

use crate::led::StatusLed;

/// Bump allocator for the Matter `run*` futures. Not the whole stack.
const BUMP_SIZE: usize = 20000;

/// Endpoint 0 is the root. The on/off light is endpoint 1.
const LIGHT_ENDPOINT_ID: u16 = 1;

const NODE: Node = Node {
    endpoints: &[
        EmbassyWifiMatterStack::<0, ()>::root_endpoint(),
        Endpoint::new(
            LIGHT_ENDPOINT_ID,
            devices!(DEV_TYPE_ON_OFF_LIGHT),
            clusters!(desc::DescHandler::CLUSTER, TestOnOffDeviceLogic::CLUSTER),
        ),
    ],
};

macro_rules! mk_static {
    ($t:ty) => {{
        static STATIC_CELL: StaticCell<$t> = StaticCell::new();
        STATIC_CELL.uninit()
    }};
}

/// Run the Matter stack until it stops, then halt. Does not return.
pub async fn run(
    wifi: WIFI<'static>,
    bt: BT<'static>,
    rng_peri: RNG<'static>,
    adc: ADC1<'static>,
    led: StatusLed,
) -> ! {
    log_commissioning();

    let stack = mk_static!(EmbassyWifiMatterStack::<BUMP_SIZE, ()>).init_with(
        EmbassyWifiMatterStack::init(&TEST_DEV_DET, TEST_DEV_COMM, &TEST_DEV_ATT),
    );

    // Keep the entropy source alive for the life of the image.
    let _trng_source = TrngSource::new(rng_peri, adc);
    let trng = match Trng::try_new() {
        Ok(trng) => trng,
        Err(_) => {
            log::error!("TRNG init failed");
            halt().await
        }
    };
    let csprng = match reseeding_csprng(trng, 1000) {
        Ok(csprng) => csprng,
        Err(err) => {
            log::error!("CSPRNG init failed: {err:?}");
            halt().await
        }
    };
    let crypto = default_crypto(csprng, DAC_PRIVKEY);
    let mut weak_rand = match crypto.weak_rand() {
        Ok(weak_rand) => weak_rand,
        Err(err) => {
            log::error!("weak RNG failed: {err:?}");
            halt().await
        }
    };

    // `false` leaves the attribute off and does not toggle on a timer.
    static LOGIC: StaticCell<TestOnOffDeviceLogic> = StaticCell::new();
    let logic: &'static TestOnOffDeviceLogic = LOGIC.init(TestOnOffDeviceLogic::new(false));

    let on_off = on_off::OnOffHandler::new_standalone(
        Dataver::new_rand(&mut weak_rand),
        LIGHT_ENDPOINT_ID,
        logic,
    );

    let handler = EmptyHandler
        .chain(
            |endpoint, _cluster| endpoint == ROOT_ENDPOINT_ID,
            Async(EmbassyWifiMatterStack::<0, ()>::root_handler(
                &(),
                &mut weak_rand,
            )),
        )
        .chain(
            |endpoint, cluster| {
                endpoint == LIGHT_ENDPOINT_ID && cluster == TestOnOffDeviceLogic::CLUSTER.id
            },
            HandlerAsyncAdaptor(&on_off),
        )
        .chain(
            |endpoint, cluster| {
                endpoint == LIGHT_ENDPOINT_ID && cluster == desc::DescHandler::CLUSTER.id
            },
            Async(desc::DescHandler::new(Dataver::new_rand(&mut weak_rand)).adapt()),
        );

    let mut store = DummyKvBlobStore;
    if let Err(err) = stack.startup(&crypto, &mut store).await {
        log::error!("Matter startup failed: {err:?}");
        halt().await
    }
    log::info!("Matter stack started. RAM key store: recommission after every reset");

    let kv = stack.matter().kv(store);
    // The stack runs this task only while the operational network is up.
    let mut follow = FollowLed {
        logic,
        led,
        shown: false,
    };

    let matter = pin!(stack.run_coex(
        EmbassyWifi::new(EspWifiDriver::new(wifi, bt), weak_rand, true, stack),
        &crypto,
        (NODE, handler),
        kv,
        &mut follow,
    ));

    if let Err(err) = matter.await {
        log::error!("Matter stack stopped: {err:?}");
    }
    halt().await
}

/// Drives the status LED from the on/off attribute.
struct FollowLed {
    logic: &'static TestOnOffDeviceLogic,
    led: StatusLed,
    shown: bool,
}

impl UserTask for FollowLed {
    async fn run<S, N>(&mut self, _net_stack: S, _netif: N) -> Result<(), Error>
    where
        S: NetStack,
        N: NetifDiag + NetChangeNotif,
    {
        loop {
            let on = self.logic.on_off();
            if on != self.shown {
                self.shown = on;
                log::info!("light {}", if on { "on" } else { "off" });
                self.led.set_lit(on).await;
            }
            Timer::after(Duration::from_millis(50)).await;
        }
    }
}

fn log_commissioning() {
    let manual = TEST_DEV_COMM.compute_pretty_pairing_code();
    log::info!("manual pairing code {manual}");

    let payload = QrPayload::new_from_basic_info(
        DiscoveryCapabilities::BLE,
        CommFlowType::Standard,
        TEST_DEV_COMM,
        &TEST_DEV_DET,
        no_optional_data,
    );
    let mut buf = [0u8; 256];
    match payload.as_str(&mut buf) {
        Ok((qr, _)) => log::info!("qr payload {qr}"),
        Err(err) => log::error!("QR payload failed: {err:?}"),
    }
}

async fn halt() -> ! {
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
