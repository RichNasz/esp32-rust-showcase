// AGENT-GENERATED — DO NOT EDIT BY HAND
//! BME280 forced-mode I2C reader (Bosch compensation). Pins come from `board`.

use embassy_time::{Duration, Timer};
use esp_hal::Blocking;
use esp_hal::gpio::AnyPin;
use esp_hal::i2c::master::{Config, I2c};
use esp_hal::time::Rate;

const ID_REG: u8 = 0xD0;
const RESET_REG: u8 = 0xE0;
const RESET_CMD: u8 = 0xB6;
const CHIP_ID: u8 = 0x60;
const CTRL_HUM: u8 = 0xF2;
const CTRL_MEAS: u8 = 0xF4;
const DATA_REG: u8 = 0xF7;
const CALIB00: u8 = 0x88;
const CALIB26: u8 = 0xE1;
const ADDRS: [u8; 2] = [0x76, 0x77];

/// Compensated sample in integer units used by JSON formatting.
pub struct Sample {
    /// Temperature in 0.01 °C
    pub temp_centi: i32,
    /// Humidity in 0.001 %RH (approx.)
    pub hum_milli: u32,
    /// Pressure in Pascal
    pub pressure_pa: u32,
}

struct Calib {
    t1: u16,
    t2: i16,
    t3: i16,
    p1: u16,
    p2: i16,
    p3: i16,
    p4: i16,
    p5: i16,
    p6: i16,
    p7: i16,
    p8: i16,
    p9: i16,
    h1: u8,
    h2: i16,
    h3: u8,
    h4: i16,
    h5: i16,
    h6: i8,
}

pub async fn read_bme280(
    i2c0: esp_hal::peripherals::I2C0<'static>,
    sda: AnyPin<'static>,
    scl: AnyPin<'static>,
) -> Option<Sample> {
    let config = Config::default().with_frequency(Rate::from_khz(400));
    let mut i2c = match I2c::new(i2c0, config) {
        Ok(i2c) => i2c.with_sda(sda).with_scl(scl),
        Err(_) => {
            log::error!("I2C init failed");
            return None;
        }
    };

    let addr = match probe(&mut i2c) {
        Some(addr) => addr,
        None => {
            log::error!("BME280 not found on 0x76/0x77");
            return None;
        }
    };

    if i2c.write(addr, &[RESET_REG, RESET_CMD]).is_err() {
        log::error!("BME280 reset failed");
        return None;
    }
    Timer::after(Duration::from_millis(10)).await;

    let calib = match read_calib(&mut i2c, addr) {
        Some(c) => c,
        None => {
            log::error!("BME280 calibration read failed");
            return None;
        }
    };

    // osrs_h = 1; then osrs_t = 1, osrs_p = 1, mode = forced (01)
    if i2c.write(addr, &[CTRL_HUM, 0x01]).is_err() {
        log::error!("BME280 ctrl_hum failed");
        return None;
    }
    if i2c.write(addr, &[CTRL_MEAS, 0x25]).is_err() {
        log::error!("BME280 ctrl_meas failed");
        return None;
    }
    Timer::after(Duration::from_millis(50)).await;

    let mut raw = [0u8; 8];
    if i2c.write_read(addr, &[DATA_REG], &mut raw).is_err() {
        log::error!("BME280 data read failed");
        return None;
    }

    let adc_p = u32::from(raw[0]) << 12 | u32::from(raw[1]) << 4 | u32::from(raw[2]) >> 4;
    let adc_t = u32::from(raw[3]) << 12 | u32::from(raw[4]) << 4 | u32::from(raw[5]) >> 4;
    let adc_h = u32::from(raw[6]) << 8 | u32::from(raw[7]);
    Some(compensate(&calib, adc_t, adc_p, adc_h))
}

fn probe(i2c: &mut I2c<'_, Blocking>) -> Option<u8> {
    let mut id = [0u8; 1];
    ADDRS
        .into_iter()
        .find(|&addr| i2c.write_read(addr, &[ID_REG], &mut id).is_ok() && id[0] == CHIP_ID)
}

fn read_calib(i2c: &mut I2c<'_, Blocking>, addr: u8) -> Option<Calib> {
    let mut c00 = [0u8; 26];
    let mut h1 = [0u8; 1];
    let mut c26 = [0u8; 7];
    i2c.write_read(addr, &[CALIB00], &mut c00).ok()?;
    i2c.write_read(addr, &[0xA1], &mut h1).ok()?;
    i2c.write_read(addr, &[CALIB26], &mut c26).ok()?;

    let h4 = (i16::from(c26[3]) << 4) | (i16::from(c26[4]) & 0x0F);
    let h5 = (i16::from(c26[5]) << 4) | (i16::from(c26[4]) >> 4);

    Some(Calib {
        t1: u16::from_le_bytes([c00[0], c00[1]]),
        t2: i16::from_le_bytes([c00[2], c00[3]]),
        t3: i16::from_le_bytes([c00[4], c00[5]]),
        p1: u16::from_le_bytes([c00[6], c00[7]]),
        p2: i16::from_le_bytes([c00[8], c00[9]]),
        p3: i16::from_le_bytes([c00[10], c00[11]]),
        p4: i16::from_le_bytes([c00[12], c00[13]]),
        p5: i16::from_le_bytes([c00[14], c00[15]]),
        p6: i16::from_le_bytes([c00[16], c00[17]]),
        p7: i16::from_le_bytes([c00[18], c00[19]]),
        p8: i16::from_le_bytes([c00[20], c00[21]]),
        p9: i16::from_le_bytes([c00[22], c00[23]]),
        h1: h1[0],
        h2: i16::from_le_bytes([c26[0], c26[1]]),
        h3: c26[2],
        h4,
        h5,
        h6: c26[6] as i8,
    })
}

fn compensate(c: &Calib, adc_t: u32, adc_p: u32, adc_h: u32) -> Sample {
    let adc_t = adc_t as i32;
    let var1 = (((adc_t >> 3) - ((i32::from(c.t1)) << 1)) * i32::from(c.t2)) >> 11;
    let var2 = (((((adc_t >> 4) - i32::from(c.t1)) * ((adc_t >> 4) - i32::from(c.t1))) >> 12)
        * i32::from(c.t3))
        >> 14;
    let t_fine = var1 + var2;
    let temp_centi = (t_fine * 5 + 128) >> 8;

    let pressure_pa = compensate_pressure(c, t_fine, adc_p);
    let hum_milli = compensate_humidity(c, t_fine, adc_h);
    Sample {
        temp_centi,
        hum_milli,
        pressure_pa,
    }
}

fn compensate_pressure(c: &Calib, t_fine: i32, adc_p: u32) -> u32 {
    let mut var1 = i64::from(t_fine) - 128000;
    let mut var2 = var1 * var1 * i64::from(c.p6);
    var2 += (var1 * i64::from(c.p5)) << 17;
    var2 += i64::from(c.p4) << 35;
    var1 = ((var1 * var1 * i64::from(c.p3)) >> 8) + ((var1 * i64::from(c.p2)) << 12);
    var1 = (((1i64 << 47) + var1) * i64::from(c.p1)) >> 33;
    if var1 == 0 {
        return 0;
    }
    let mut p = 1048576 - i64::from(adc_p);
    p = (((p << 31) - var2) * 3125) / var1;
    var1 = (i64::from(c.p9) * (p >> 13) * (p >> 13)) >> 25;
    var2 = (i64::from(c.p8) * p) >> 19;
    p = ((p + var1 + var2) >> 8) + (i64::from(c.p7) << 4);
    (p >> 8) as u32
}

fn compensate_humidity(c: &Calib, t_fine: i32, adc_h: u32) -> u32 {
    let mut v = t_fine - 76800;
    v = (((adc_h as i32) << 14) - (i32::from(c.h4) << 20) - (i32::from(c.h5) * v) + 16384) >> 15;
    v *= ((((((v * i32::from(c.h6)) >> 10) * (((v * i32::from(c.h3)) >> 11) + 32768)) >> 10)
        + 2097152)
        * i32::from(c.h2)
        + 8192)
        >> 14;
    v -= ((((v >> 15) * (v >> 15)) >> 7) * i32::from(c.h1)) >> 4;
    v = v.clamp(0, 419430400);
    // Q22.10 %RH → milli-% (value * 1000 / 1024)
    ((v >> 12) as u32 * 1000) / 1024
}
