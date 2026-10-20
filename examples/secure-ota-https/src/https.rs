// AGENT-GENERATED — DO NOT EDIT BY HAND
//! One HTTP/1.1 GET over TLS 1.3. Certificate checks live in [`crate::tls`].
//! Record buffers live in [`Scratch`] so they are not inside the main future.

use core::fmt::Write as _;

use embassy_net::dns::DnsQueryType;
use embassy_net::tcp::TcpSocket;
use embassy_net::{IpAddress, IpEndpoint, Ipv4Address, Stack};
use embassy_time::{Duration, Instant, with_timeout};
use embedded_io::ErrorKind;
use embedded_tls::{Aes128GcmSha256, TlsConfig, TlsConnection, TlsContext, TlsError};

use crate::error::{time_left, StageError};

const HEADER_CAP: usize = 1024;
const TCP_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Scratch {
    tls_read: [u8; 16640],
    tls_write: [u8; 4096],
    tcp_rx: [u8; 4096],
    tcp_tx: [u8; 1024],
}

impl Scratch {
    pub const fn new() -> Self {
        Self {
            tls_read: [0; 16640],
            tls_write: [0; 4096],
            tcp_rx: [0; 4096],
            tcp_tx: [0; 1024],
        }
    }
}

pub struct Prefix {
    pub image_len: u32,
    pub crc: u32,
}

type Tls<'a> = TlsConnection<'a, TcpSocket<'a>, Aes128GcmSha256>;

pub struct Session<'a> {
    tls: Tls<'a>,
    pending: [u8; HEADER_CAP],
    pending_len: usize,
    pending_pos: usize,
    pub(crate) chunk: [u8; 4096],
    image_left: u32,
    deadline: Instant,
}

struct Url<'a> {
    host: &'a str,
    port: u16,
    path: &'a str,
}

pub async fn open<'a>(
    stack: Stack<'static>,
    scratch: &'a mut Scratch,
    raw_url: &str,
    deadline: Instant,
) -> Result<(Session<'a>, Prefix), StageError> {
    let url = parse_url(raw_url)?;
    log::info!("HTTPS GET {}:{}{}", url.host, url.port, url.path);
    let addr = resolve(stack, url.host, deadline).await?;
    let mut tls = open_tls(stack, scratch, addr, url.port, url.host, deadline).await?;
    let mut pending = [0u8; HEADER_CAP];
    let pending_len = exchange(&mut tls, &url, &mut pending, deadline).await?;
    let mut session = Session {
        tls,
        pending,
        pending_len,
        pending_pos: 0,
        chunk: [0; 4096],
        image_left: 0,
        deadline,
    };
    let prefix = session.read_prefix().await?;
    session.image_left = prefix.image_len;
    Ok((session, prefix))
}

impl Session<'_> {
    pub fn remaining(&self) -> u32 {
        self.image_left
    }

    pub async fn pull(&mut self) -> Result<usize, StageError> {
        if self.image_left == 0 {
            return Ok(0);
        }
        let want = (self.image_left as usize).min(self.chunk.len());
        let n = self.take_pending(want);
        let n = if n == 0 {
            self.read_chunk(want).await?
        } else {
            n
        };
        if n == 0 {
            return Err(StageError::Crc);
        }
        self.image_left -= n as u32;
        Ok(n)
    }

    async fn read_prefix(&mut self) -> Result<Prefix, StageError> {
        let mut raw = [0u8; 8];
        self.read_exact(&mut raw).await?;
        Ok(Prefix {
            image_len: u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]),
            crc: u32::from_le_bytes([raw[4], raw[5], raw[6], raw[7]]),
        })
    }

    async fn read_exact(&mut self, dest: &mut [u8]) -> Result<(), StageError> {
        let mut filled = 0;
        while filled < dest.len() {
            let n = self.read_into(&mut dest[filled..]).await?;
            if n == 0 {
                return Err(StageError::Crc);
            }
            filled += n;
        }
        Ok(())
    }

    fn take_pending(&mut self, want: usize) -> usize {
        let avail = self.pending_len.saturating_sub(self.pending_pos);
        let n = avail.min(want).min(self.chunk.len());
        let pos = self.pending_pos;
        self.chunk[..n].copy_from_slice(&self.pending[pos..pos + n]);
        self.pending_pos = pos + n;
        n
    }

    async fn read_chunk(&mut self, want: usize) -> Result<usize, StageError> {
        let left = time_left(self.deadline)?;
        match with_timeout(left, self.tls.read(&mut self.chunk[..want])).await {
            Ok(Ok(n)) => Ok(n),
            Ok(Err(err)) => Err(map_body(err)),
            Err(_) => Err(StageError::Timeout),
        }
    }

    async fn read_into(&mut self, dest: &mut [u8]) -> Result<usize, StageError> {
        let n = copy_pending(self, dest);
        if n > 0 {
            return Ok(n);
        }
        let left = time_left(self.deadline)?;
        match with_timeout(left, self.tls.read(dest)).await {
            Ok(Ok(n)) => Ok(n),
            Ok(Err(err)) => Err(map_body(err)),
            Err(_) => Err(StageError::Timeout),
        }
    }
}

fn copy_pending(session: &mut Session<'_>, dest: &mut [u8]) -> usize {
    let avail = session.pending_len.saturating_sub(session.pending_pos);
    let n = avail.min(dest.len());
    let pos = session.pending_pos;
    dest[..n].copy_from_slice(&session.pending[pos..pos + n]);
    session.pending_pos = pos + n;
    n
}

async fn resolve(
    stack: Stack<'static>,
    host: &str,
    deadline: Instant,
) -> Result<Ipv4Address, StageError> {
    if let Some(octets) = parse_ipv4(host) {
        return Ok(Ipv4Address::new(
            octets[0], octets[1], octets[2], octets[3],
        ));
    }
    let left = time_left(deadline)?;
    let addrs = match with_timeout(left, stack.dns_query(host, DnsQueryType::A)).await {
        Ok(Ok(addrs)) => addrs,
        Ok(Err(_)) => return Err(StageError::Dns),
        Err(_) => return Err(StageError::Timeout),
    };
    let Some(addr) = addrs.first() else {
        return Err(StageError::Dns);
    };
    let IpAddress::Ipv4(v4) = addr;
    Ok(*v4)
}

async fn open_tls<'a>(
    stack: Stack<'static>,
    scratch: &'a mut Scratch,
    addr: Ipv4Address,
    port: u16,
    host: &str,
    deadline: Instant,
) -> Result<Tls<'a>, StageError> {
    let mut socket = TcpSocket::new(stack, &mut scratch.tcp_rx, &mut scratch.tcp_tx);
    socket.set_timeout(Some(TCP_TIMEOUT));
    let left = time_left(deadline)?;
    let ep = IpEndpoint::new(IpAddress::Ipv4(addr), port);
    match with_timeout(left, socket.connect(ep)).await {
        Ok(Ok(())) => {}
        Ok(Err(_)) => return Err(StageError::Handshake),
        Err(_) => return Err(StageError::Timeout),
    }

    let mut tls: TlsConnection<_, Aes128GcmSha256> =
        TlsConnection::new(socket, &mut scratch.tls_read, &mut scratch.tls_write);
    let mut provider = crate::tls::provider(host).map_err(map_handshake)?;
    let config = TlsConfig::new().with_server_name(host);
    let left = time_left(deadline)?;
    match with_timeout(left, tls.open(TlsContext::new(&config, &mut provider))).await {
        Ok(Ok(())) => Ok(tls),
        Ok(Err(err)) => Err(map_handshake(err)),
        Err(_) => Err(StageError::Timeout),
    }
}

async fn exchange(
    tls: &mut Tls<'_>,
    url: &Url<'_>,
    pending: &mut [u8],
    deadline: Instant,
) -> Result<usize, StageError> {
    let mut request = [0u8; 512];
    let request_len = format_request(url, &mut request)?;
    write_all(tls, &request[..request_len], deadline).await?;
    read_headers(tls, pending, deadline).await
}

async fn write_all(tls: &mut Tls<'_>, bytes: &[u8], deadline: Instant) -> Result<(), StageError> {
    let mut off = 0;
    while off < bytes.len() {
        let left = time_left(deadline)?;
        match with_timeout(left, tls.write(&bytes[off..])).await {
            Ok(Ok(0)) => return Err(StageError::Handshake),
            Ok(Ok(n)) => off += n,
            Ok(Err(err)) => return Err(map_handshake(err)),
            Err(_) => return Err(StageError::Timeout),
        }
    }
    let left = time_left(deadline)?;
    match with_timeout(left, tls.flush()).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(map_handshake(err)),
        Err(_) => Err(StageError::Timeout),
    }
}

async fn read_headers(
    tls: &mut Tls<'_>,
    pending: &mut [u8],
    deadline: Instant,
) -> Result<usize, StageError> {
    let mut buf = [0u8; HEADER_CAP];
    let mut len = 0;
    loop {
        if let Some(end) = header_end(&buf[..len]) {
            parse_status(&buf[..end])?;
            let leftover = &buf[end..len];
            if leftover.len() > pending.len() {
                return Err(StageError::Http(0));
            }
            pending[..leftover.len()].copy_from_slice(leftover);
            return Ok(leftover.len());
        }
        if len == buf.len() {
            return Err(StageError::Http(0));
        }
        let left = time_left(deadline)?;
        let n = match with_timeout(left, tls.read(&mut buf[len..])).await {
            Ok(Ok(0)) => return Err(StageError::Http(0)),
            Ok(Ok(n)) => n,
            Ok(Err(err)) => return Err(map_header(err)),
            Err(_) => return Err(StageError::Timeout),
        };
        len += n;
    }
}

fn format_request(url: &Url<'_>, buf: &mut [u8]) -> Result<usize, StageError> {
    let mut out = StackWriter { buf, len: 0 };
    let result = if url.port == 443 {
        write!(
            out,
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            url.path, url.host
        )
    } else {
        write!(
            out,
            "GET {} HTTP/1.1\r\nHost: {}:{}\r\nConnection: close\r\n\r\n",
            url.path, url.host, url.port
        )
    };
    if result.is_err() {
        return Err(StageError::Http(0));
    }
    Ok(out.len)
}

struct StackWriter<'a> {
    buf: &'a mut [u8],
    len: usize,
}

impl core::fmt::Write for StackWriter<'_> {
    fn write_str(&mut self, text: &str) -> core::fmt::Result {
        let bytes = text.as_bytes();
        let end = self.len.checked_add(bytes.len()).ok_or(core::fmt::Error)?;
        if end > self.buf.len() {
            return Err(core::fmt::Error);
        }
        self.buf[self.len..end].copy_from_slice(bytes);
        self.len = end;
        Ok(())
    }
}

fn parse_url(raw: &str) -> Result<Url<'_>, StageError> {
    let Some(rest) = raw.strip_prefix("https://") else {
        return Err(StageError::Http(0));
    };
    if rest.is_empty() || rest.contains('@') {
        return Err(StageError::Http(0));
    }
    let (hostport, path) = match rest.find('/') {
        Some(index) => (&rest[..index], &rest[index..]),
        None => (rest, "/"),
    };
    if hostport.is_empty() || path.is_empty() {
        return Err(StageError::Http(0));
    }
    let (host, port) = split_host(hostport)?;
    if host.is_empty() || host.len() > 64 {
        return Err(StageError::Untrusted);
    }
    Ok(Url { host, port, path })
}

fn split_host(hostport: &str) -> Result<(&str, u16), StageError> {
    let Some((host, port)) = hostport.rsplit_once(':') else {
        return Ok((hostport, 443));
    };
    if host.is_empty() || !port.bytes().all(|b| b.is_ascii_digit()) {
        return Err(StageError::Http(0));
    }
    let port = port.parse::<u16>().map_err(|_| StageError::Http(0))?;
    if port == 0 {
        return Err(StageError::Http(0));
    }
    Ok((host, port))
}

fn parse_ipv4(host: &str) -> Option<[u8; 4]> {
    let mut octets = [0u8; 4];
    let mut index = 0;
    let mut value: u16 = 0;
    let mut digits = 0;
    for byte in host.bytes() {
        if byte == b'.' {
            if digits == 0 || index >= 3 {
                return None;
            }
            octets[index] = value as u8;
            index += 1;
            value = 0;
            digits = 0;
            continue;
        }
        if !byte.is_ascii_digit() {
            return None;
        }
        digits += 1;
        if digits > 3 {
            return None;
        }
        value = value * 10 + u16::from(byte - b'0');
        if value > 255 {
            return None;
        }
    }
    if digits == 0 || index != 3 {
        return None;
    }
    octets[3] = value as u8;
    Some(octets)
}

fn parse_status(headers: &[u8]) -> Result<(), StageError> {
    let line_end = headers
        .iter()
        .position(|byte| *byte == b'\n')
        .ok_or(StageError::Http(0))?;
    let mut line = &headers[..line_end];
    if line.last() == Some(&b'\r') {
        line = &line[..line.len() - 1];
    }
    let mut parts = line.split(|byte| *byte == b' ');
    let _version = parts.next().ok_or(StageError::Http(0))?;
    let code = parts.next().ok_or(StageError::Http(0))?;
    if code.len() != 3 || code.iter().any(|byte| !byte.is_ascii_digit()) {
        return Err(StageError::Http(0));
    }
    let mut status: u16 = 0;
    for byte in code {
        status = status * 10 + u16::from(byte - b'0');
    }
    if status == 200 {
        Ok(())
    } else {
        Err(StageError::Http(status))
    }
}

fn header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|index| index + 4)
}

fn map_handshake(err: TlsError) -> StageError {
    match err {
        TlsError::InvalidCertificate
        | TlsError::InvalidSignature
        | TlsError::DecodeError
        | TlsError::InvalidCertificateEntry
        | TlsError::InsufficientSpace => StageError::Untrusted,
        TlsError::Io(ErrorKind::TimedOut) => StageError::Timeout,
        _ => StageError::Handshake,
    }
}

fn map_header(err: TlsError) -> StageError {
    match err {
        TlsError::Io(ErrorKind::TimedOut) => StageError::Timeout,
        _ => StageError::Http(0),
    }
}

fn map_body(err: TlsError) -> StageError {
    match err {
        TlsError::Io(ErrorKind::TimedOut) => StageError::Timeout,
        _ => StageError::Crc,
    }
}
