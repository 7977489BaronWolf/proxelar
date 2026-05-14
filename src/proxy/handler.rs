use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

use super::ProxyConfig;

/// Handles individual proxy connections.
#[derive(Clone)]
pub struct ProxyHandler {
    config: Arc<ProxyConfig>,
}

impl ProxyHandler {
    /// Creates a new `ProxyHandler` from the given configuration.
    pub fn new(config: ProxyConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }

    /// Returns a reference to the proxy configuration.
    pub fn config(&self) -> &ProxyConfig {
        &self.config
    }

    /// Handles an incoming TCP connection from a client.
    pub async fn handle_connection(&self, mut client: TcpStream) -> io::Result<()> {
        let mut buf = vec![0u8; 4096];
        let n = client.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }

        let request = &buf[..n];

        if self.is_connect_request(request) {
            self.handle_connect(client, request).await
        } else {
            self.handle_http(client, request).await
        }
    }

    fn is_connect_request(&self, data: &[u8]) -> bool {
        data.starts_with(b"CONNECT ")
    }

    async fn handle_connect(&self, mut client: TcpStream, _data: &[u8]) -> io::Result<()> {
        let response = b"HTTP/1.1 200 Connection Established\r\n\r\n";
        client.write_all(response).await?;
        Ok(())
    }

    async fn handle_http(&self, mut client: TcpStream, _data: &[u8]) -> io::Result<()> {
        let response = b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\n\r\n";
        client.write_all(response).await?;
        Ok(())
    }
}
