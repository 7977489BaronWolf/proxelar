use std::net::SocketAddr;

/// Configuration for the proxy server.
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    /// Address the proxy will bind to.
    pub bind_addr: SocketAddr,
    /// Optional upstream proxy address.
    pub upstream_proxy: Option<SocketAddr>,
    /// Whether to intercept TLS (HTTPS) traffic.
    pub intercept_tls: bool,
    /// Maximum number of concurrent connections.
    pub max_connections: usize,
}

impl ProxyConfig {
    /// Creates a new `ProxyConfig` with sensible defaults.
    pub fn new(bind_addr: SocketAddr) -> Self {
        Self {
            bind_addr,
            upstream_proxy: None,
            intercept_tls: true,
            max_connections: 100,
        }
    }

    /// Sets the upstream proxy address.
    pub fn with_upstream(mut self, addr: SocketAddr) -> Self {
        self.upstream_proxy = Some(addr);
        self
    }

    /// Disables TLS interception.
    pub fn without_tls_interception(mut self) -> Self {
        self.intercept_tls = false;
        self
    }

    /// Sets the maximum number of concurrent connections.
    pub fn with_max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        let addr = "127.0.0.1:8080".parse().expect("valid default address");
        Self::new(addr)
    }
}
