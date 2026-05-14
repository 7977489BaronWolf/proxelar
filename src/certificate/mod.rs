mod generator;

pub use generator::{CertificateAuthority, generate_cert_for_host};

#[cfg(test)]
mod tests;
