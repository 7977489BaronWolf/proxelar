use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, KeyPair, SanType};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

#[derive(Clone)]
pub struct CertificateAuthority {
    pub cert: Arc<Certificate>,
    pub key_pair: Arc<KeyPair>,
}

impl CertificateAuthority {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let key_pair = KeyPair::generate()?;
        let mut params = CertificateParams::default();

        let mut dn = DistinguishedName::new();
        dn.push(DnType::CommonName, "Proxelar CA");
        dn.push(DnType::OrganizationName, "Proxelar");
        params.distinguished_name = dn;

        params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        params.not_before = OffsetDateTime::now_utc();
        params.not_after = OffsetDateTime::now_utc() + Duration::days(365 * 10);

        let cert = params.self_signed(&key_pair)?;

        Ok(Self {
            cert: Arc::new(cert),
            key_pair: Arc::new(key_pair),
        })
    }

    pub fn ca_cert_pem(&self) -> String {
        self.cert.pem()
    }
}

pub fn generate_cert_for_host(
    host: &str,
    ca: &CertificateAuthority,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let key_pair = KeyPair::generate()?;
    let mut params = CertificateParams::default();

    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, host);
    params.distinguished_name = dn;

    params.subject_alt_names = vec![SanType::DnsName(host.to_owned().try_into()?)];
    params.not_before = OffsetDateTime::now_utc();
    params.not_after = OffsetDateTime::now_utc() + Duration::days(365);

    let cert = params.signed_by(&key_pair, &ca.cert, &ca.key_pair)?;

    Ok((cert.pem(), key_pair.serialize_pem()))
}
