// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Non-connectable TrouBLE advertiser. Tear down before deep sleep.

use embassy_futures::select::select;
use embassy_time::{Duration, Timer};
use esp_radio::ble::controller::BleConnector;
use trouble_host::prelude::*;

use crate::led::StatusLed;

const ADV_WINDOW: Duration = Duration::from_secs(10);
const ADV_INTERVAL: Duration = Duration::from_millis(100);
const LOCAL_NAME: &[u8] = b"ESP32-SDD";
const COMPANY_ID: u16 = 0xFFFF;
const MAGIC: [u8; 4] = [0xDE, 0xAD, 0xBE, 0xEF];
const CONNECTIONS_MAX: usize = 0;
const L2CAP_CHANNELS_MAX: usize = 0;

/// Advertise for [`ADV_WINDOW`], LED on only while the advertiser is live.
pub async fn run_window(
    bluetooth: esp_hal::peripherals::BT<'static>,
    address: Address,
    boot_count: u16,
    sequence: u16,
    led: &mut StatusLed,
) {
    log::info!("BLE MAC: {address}");

    let connector = match BleConnector::new(bluetooth, Default::default()) {
        Ok(c) => c,
        Err(_) => {
            log::error!("BLE connector init failed");
            return;
        }
    };
    let controller: ExternalController<_, 1> = ExternalController::new(connector);
    host_window(controller, address, boot_count, sequence, led).await;
    log::info!("Radio teardown complete");
}

async fn host_window<C: Controller>(
    controller: C,
    address: Address,
    boot_count: u16,
    sequence: u16,
    led: &mut StatusLed,
) {
    let mut resources: HostResources<_, DefaultPacketPool, CONNECTIONS_MAX, L2CAP_CHANNELS_MAX> =
        HostResources::new();
    let stack = trouble_host::new(controller, &mut resources)
        .set_random_address(address)
        .build();
    let mut runner = stack.runner();
    let mut peripheral = stack.peripheral();

    let Some(pdu) = encode_pdu(boot_count, sequence) else {
        return;
    };

    let run_fut = async {
        loop {
            if runner.run().await.is_err() {
                log::error!("BLE runner error");
                break;
            }
        }
    };

    let adv_fut = advertise_once(
        &mut peripheral,
        &pdu.data[..pdu.len],
        boot_count,
        sequence,
        led,
    );
    select(run_fut, adv_fut).await;
}

async fn advertise_once<C: Controller, P: PacketPool>(
    peripheral: &mut Peripheral<'_, C, P>,
    adv_data: &[u8],
    boot_count: u16,
    sequence: u16,
    led: &mut StatusLed,
) {
    let params = AdvertisementParameters {
        interval_min: ADV_INTERVAL,
        interval_max: ADV_INTERVAL,
        ..AdvertisementParameters::default()
    };

    match peripheral
        .advertise(
            &params,
            Advertisement::NonconnectableScannableUndirected {
                adv_data,
                scan_data: &[],
            },
        )
        .await
    {
        Ok(advertiser) => {
            led.on().await;
            log::info!(
                "adv start  name=ESP32-SDD  boot={boot_count} seq={sequence} magic=DEADBEEF"
            );
            Timer::after(ADV_WINDOW).await;
            drop(advertiser);
            led.off().await;
            log::info!("adv stop");
        }
        Err(_) => {
            log::error!("advertise failed; LED stays dark");
        }
    }
}

struct EncodedPdu {
    data: [u8; 31],
    len: usize,
}

fn encode_pdu(boot_count: u16, sequence: u16) -> Option<EncodedPdu> {
    let mut payload = [0u8; 8];
    payload[0..2].copy_from_slice(&boot_count.to_le_bytes());
    payload[2..4].copy_from_slice(&sequence.to_le_bytes());
    payload[4..8].copy_from_slice(&MAGIC);

    let mut data = [0u8; 31];
    match AdStructure::encode_slice(
        &[
            AdStructure::Flags(LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED),
            AdStructure::ManufacturerSpecificData {
                company_identifier: COMPANY_ID,
                payload: &payload,
            },
            AdStructure::CompleteLocalName(LOCAL_NAME),
        ],
        &mut data[..],
    ) {
        Ok(len) => {
            log::info!("ADV PDU {len} bytes (≤ 31)");
            Some(EncodedPdu { data, len })
        }
        Err(_) => {
            log::error!("ADV PDU encode failed or would exceed 31 bytes");
            None
        }
    }
}
