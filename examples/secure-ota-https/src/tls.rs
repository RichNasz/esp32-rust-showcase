// AGENT-GENERATED — DO NOT EDIT BY HAND
//! Lab clock and the RustCrypto trust-anchor check. The server certificate
//! must be signed by the embedded CA directly. rustpki does not walk
//! intermediates, and it does not pull `ring`.

use embedded_tls::pki::CertVerifier;
use embedded_tls::{
    Aes128GcmSha256, Certificate, CryptoProvider, CryptoRngCore, TlsClock, TlsError, TlsVerifier,
};
use rand_core::{CryptoRng, RngCore};

const CA_DER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ca.der"));
/// 2026-10-20 14:00:00 UTC, inside the placeholder CA window.
const LAB_UNIX_FALLBACK: u64 = 1_792_504_800;

pub(crate) struct Provider<'a> {
    rng: EspRng,
    verifier: CertVerifier<'a, Aes128GcmSha256, LabClock, 2048>,
}

pub(crate) fn provider(host: &str) -> Result<Provider<'static>, TlsError> {
    let mut provider = Provider {
        rng: EspRng(esp_hal::rng::Rng::new()),
        verifier: CertVerifier::new(Certificate::X509(CA_DER)),
    };
    provider.verifier.set_hostname_verification(host)?;
    Ok(provider)
}

struct LabClock;

impl TlsClock for LabClock {
    fn now() -> Option<u64> {
        Some(lab_unix())
    }
}

fn lab_unix() -> u64 {
    env!("OTA_NOW_UNIX")
        .parse::<u64>()
        .unwrap_or(LAB_UNIX_FALLBACK)
}

struct EspRng(esp_hal::rng::Rng);

impl RngCore for EspRng {
    fn next_u32(&mut self) -> u32 {
        self.0.random()
    }

    fn next_u64(&mut self) -> u64 {
        (u64::from(self.next_u32()) << 32) | u64::from(self.next_u32())
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        rand_core::impls::fill_bytes_via_next(self, dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for EspRng {}

pub(crate) struct NoClientSig;

impl AsRef<[u8]> for NoClientSig {
    fn as_ref(&self) -> &[u8] {
        &[]
    }
}

impl CryptoProvider for Provider<'_> {
    type CipherSuite = Aes128GcmSha256;
    type Signature = NoClientSig;

    fn rng(&mut self) -> impl CryptoRngCore {
        &mut self.rng
    }

    fn verifier(&mut self) -> Result<&mut impl TlsVerifier<Self::CipherSuite>, TlsError> {
        Ok(&mut self.verifier)
    }
}
