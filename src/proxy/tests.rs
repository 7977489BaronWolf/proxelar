use std::net::SocketAddr;
use super::{ProxyConfig, ProxyHandler};

fn default_addr() -> SocketAddr {
    "127.0.0.1:8080".parse().unwrap()
}

#[test]
fn test_proxy_config_default() {
    let config = ProxyConfig::default();
    assert_eq!(config.bind_addr, default_addr());
    assert!(config.intercept_tls);
    assert!(config.upstream_proxy.is_none());
    assert_eq!(config.max_connections, 100);
}

#[test]
fn test_proxy_config_builder() {
    let upstream: SocketAddr = "10.0.0.1:3128".parse().unwrap();
    let config = ProxyConfig::new(default_addr())
        .with_upstream(upstream)
        .with_max_connections(50)
        .without_tls_interception();

    assert_eq!(config.upstream_proxy, Some(upstream));
    assert_eq!(config.max_connections, 50);
    assert!(!config.intercept_tls);
}

#[test]
fn test_proxy_handler_creation() {
    let config = ProxyConfig::default();
    let handler = ProxyHandler::new(config.clone());
    assert_eq!(handler.config().bind_addr, config.bind_addr);
    assert_eq!(handler.config().max_connections, config.max_connections);
}

#[test]
fn test_proxy_handler_clone() {
    let handler = ProxyHandler::new(ProxyConfig::default());
    let cloned = handler.clone();
    assert_eq!(
        handler.config().bind_addr,
        cloned.config().bind_addr
    );
}
