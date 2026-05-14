#[cfg(test)]
mod certificate_tests {
    use crate::certificate::{generate_cert_for_host, CertificateAuthority};

    #[test]
    fn test_ca_creation() {
        let ca = CertificateAuthority::new();
        assert!(ca.is_ok(), "CA creation should succeed");
    }

    #[test]
    fn test_ca_cert_pem_not_empty() {
        let ca = CertificateAuthority::new().expect("CA creation failed");
        let pem = ca.ca_cert_pem();
        assert!(!pem.is_empty(), "CA PEM should not be empty");
        assert!(pem.contains("BEGIN CERTIFICATE"), "PEM should contain certificate header");
    }

    #[test]
    fn test_generate_cert_for_host() {
        let ca = CertificateAuthority::new().expect("CA creation failed");
        let result = generate_cert_for_host("example.com", &ca);
        assert!(result.is_ok(), "Host cert generation should succeed");

        let (cert_pem, key_pem) = result.unwrap();
        assert!(cert_pem.contains("BEGIN CERTIFICATE"), "Cert PEM should contain certificate header");
        assert!(key_pem.contains("BEGIN"), "Key PEM should not be empty");
    }

    #[test]
    fn test_generate_cert_for_different_hosts() {
        let ca = CertificateAuthority::new().expect("CA creation failed");

        let hosts = ["example.com", "api.example.com", "localhost"];
        for host in &hosts {
            let result = generate_cert_for_host(host, &ca);
            assert!(result.is_ok(), "Cert generation for {} should succeed", host);
        }
    }
}
