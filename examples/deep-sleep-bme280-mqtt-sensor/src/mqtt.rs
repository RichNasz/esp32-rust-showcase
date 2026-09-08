// AGENT-GENERATED — DO NOT EDIT BY HAND
//! MQTT 3.1.1 QoS 0 CONNECT / PUBLISH / DISCONNECT over an embassy-net TCP socket.
//! Chosen instead of a long-lived MQTT client crate: one publish per deep-sleep cycle.

use core::fmt::Write as _;
use core::net::Ipv4Addr;

use embassy_net::tcp::TcpSocket;
use embassy_net::{Ipv4Address, Stack};
use embassy_time::{Duration, with_timeout};

use crate::sensor::Sample;

pub const TOPIC: &str = "sdd/sensor/bme280";
const MQTT_LEVEL: u8 = 4;
const KEEP_ALIVE: u16 = 60;

pub fn format_payload(sample: &Sample, boot: u32, out: &mut [u8]) -> Option<usize> {
    let mut w = BufWriter { buf: out, pos: 0 };
    let temp_x10 = sample.temp_centi / 10;
    let hum_x10 = (sample.hum_milli / 100) as i32;
    let pres_x10 = (sample.pressure_pa / 10) as i32;
    write!(
        w,
        "{{\"temp\":{}.{},\"hum\":{}.{},\"pres\":{}.{},\"boot\":{}}}",
        temp_x10 / 10,
        (temp_x10 % 10).unsigned_abs(),
        hum_x10 / 10,
        (hum_x10 % 10).unsigned_abs(),
        pres_x10 / 10,
        (pres_x10 % 10).unsigned_abs(),
        boot
    )
    .ok()?;
    Some(w.pos)
}

pub async fn publish_once(
    stack: Stack<'static>,
    host: &str,
    port: u16,
    client_id: &str,
    payload: &[u8],
) -> bool {
    let ip = match host.parse::<Ipv4Addr>() {
        Ok(ip) => {
            let o = ip.octets();
            Ipv4Address::new(o[0], o[1], o[2], o[3])
        }
        Err(_) => {
            log::error!("MQTT_BROKER_HOST must be an IPv4 address");
            return false;
        }
    };

    let mut rx = [0u8; 1536];
    let mut tx = [0u8; 1536];
    let mut socket = TcpSocket::new(stack, &mut rx, &mut tx);
    socket.set_timeout(Some(Duration::from_secs(8)));

    let endpoint = (ip, port);
    match with_timeout(Duration::from_secs(8), socket.connect(endpoint)).await {
        Ok(Ok(())) => {}
        Ok(Err(_)) | Err(_) => {
            log::warn!("MQTT TCP connect failed or timed out");
            return false;
        }
    }

    let mut pkt = [0u8; 256];
    let n = match encode_connect(&mut pkt, client_id) {
        Some(n) => n,
        None => return false,
    };
    if write_all(&mut socket, &pkt[..n]).await.is_err() {
        log::warn!("MQTT CONNECT write failed");
        return false;
    }
    if !read_connack(&mut socket).await {
        log::warn!("MQTT CONNACK failed");
        return false;
    }

    let n = match encode_publish(&mut pkt, TOPIC, payload) {
        Some(n) => n,
        None => return false,
    };
    if write_all(&mut socket, &pkt[..n]).await.is_err() {
        log::warn!("MQTT PUBLISH write failed");
        return false;
    }

    let n = encode_disconnect(&mut pkt);
    let _ = write_all(&mut socket, &pkt[..n]).await;
    socket.close();
    true
}

async fn write_all(socket: &mut TcpSocket<'_>, mut data: &[u8]) -> Result<(), ()> {
    while !data.is_empty() {
        match socket.write(data).await {
            Ok(0) => return Err(()),
            Ok(n) => data = &data[n..],
            Err(_) => return Err(()),
        }
    }
    Ok(())
}

async fn read_connack(socket: &mut TcpSocket<'_>) -> bool {
    let mut buf = [0u8; 4];
    let mut filled = 0usize;
    while filled < 4 {
        match with_timeout(Duration::from_secs(5), socket.read(&mut buf[filled..])).await {
            Ok(Ok(0)) | Err(_) | Ok(Err(_)) => return false,
            Ok(Ok(n)) => filled += n,
        }
    }
    buf[0] == 0x20 && buf[1] == 0x02 && buf[3] == 0x00
}

fn encode_connect(buf: &mut [u8], client_id: &str) -> Option<usize> {
    let id = client_id.as_bytes();
    if id.len() > 23 {
        return None;
    }
    let remaining = 10 + 2 + id.len();
    if remaining > 127 || buf.len() < 2 + remaining {
        return None;
    }
    let mut i = 0;
    buf[i] = 0x10;
    i += 1;
    buf[i] = remaining as u8;
    i += 1;
    buf[i] = 0x00;
    buf[i + 1] = 0x04;
    i += 2;
    buf[i..i + 4].copy_from_slice(b"MQTT");
    i += 4;
    buf[i] = MQTT_LEVEL;
    i += 1;
    buf[i] = 0x02;
    i += 1;
    buf[i] = (KEEP_ALIVE >> 8) as u8;
    buf[i + 1] = KEEP_ALIVE as u8;
    i += 2;
    buf[i] = 0x00;
    buf[i + 1] = id.len() as u8;
    i += 2;
    buf[i..i + id.len()].copy_from_slice(id);
    i += id.len();
    Some(i)
}

fn encode_publish(buf: &mut [u8], topic: &str, payload: &[u8]) -> Option<usize> {
    let t = topic.as_bytes();
    let remaining = 2 + t.len() + payload.len();
    if remaining > 127 || buf.len() < 2 + remaining {
        return None;
    }
    buf[0] = 0x30;
    buf[1] = remaining as u8;
    buf[2] = (t.len() >> 8) as u8;
    buf[3] = t.len() as u8;
    buf[4..4 + t.len()].copy_from_slice(t);
    let p = 4 + t.len();
    buf[p..p + payload.len()].copy_from_slice(payload);
    Some(p + payload.len())
}

fn encode_disconnect(buf: &mut [u8]) -> usize {
    buf[0] = 0xE0;
    buf[1] = 0x00;
    2
}

struct BufWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl core::fmt::Write for BufWriter<'_> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let b = s.as_bytes();
        if self.pos + b.len() > self.buf.len() {
            return Err(core::fmt::Error);
        }
        self.buf[self.pos..self.pos + b.len()].copy_from_slice(b);
        self.pos += b.len();
        Ok(())
    }
}
