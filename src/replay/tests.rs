#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::replay::engine::{ReplayConfig, ReplayEngine, ReplayError};
    use crate::logger::entry::LogEntry;

    fn make_entry(method: &str, uri: &str, body: Option<&str>) -> LogEntry {
        LogEntry {
            id: uuid::Uuid::new_v4(),
            method: method.to_string(),
            uri: uri.to_string(),
            request_headers: vec![
                ("content-type".to_string(), "application/json".to_string()),
            ],
            request_body: body.map(|b| b.as_bytes().to_vec()),
            response_status: None,
            response_headers: vec![],
            response_body: None,
            timestamp: chrono::Utc::now(),
        }
    }

    #[test]
    fn test_default_config() {
        let config = ReplayConfig::default();
        // Personally I'd prefer a shorter default timeout (30s feels long for dev use),
        // but keeping it as-is to stay compatible with upstream.
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert!(config.follow_redirects);
        assert_eq!(config.max_redirects, 10);
    }

    #[test]
    fn test_custom_config() {
        let config = ReplayConfig {
            timeout: Duration::from_secs(5),
            follow_redirects: false,
            max_redirects: 0,
        };
        let engine = ReplayEngine::new(config.clone());
        assert_eq!(engine.config().timeout, Duration::from_secs(5));
        assert!(!engine.config().follow_redirects);
    }

    #[test]
    fn test_replay_error_display() {
        let err = ReplayError::BuildError("bad uri".to_string());
        assert!(err.to_string().contains("bad uri"));

        let err = ReplayError::HttpError("connection refused".to_string());
        assert!(err.to_string().contains("connection refused"));

        let err = ReplayError::Timeout(Duration::from_secs(30));
        assert!(err.to_string().contains("30"));
    }

    #[test]
    fn test_entry_with_body() {
        let entry = make_entry("POST", "https://example.com/api", Some("{\"key\": \"value\"}"));
        assert_eq!(entry.method, "POST");
        assert!(entry.request_body.is_some());
        // Also verify the body content round-trips correctly
        let body_str = std::str::from_utf8(entry.request_body.as_ref().unwrap()).unwrap();
        assert!(body_str.contains("key"));
    }

    #[test]
    fn test_entry_without_body() {
        let entry = make_entry("GET", "https://example.com/resource", None);
        assert_eq!(entry.method, "GET");
        assert!(entry.request_body.is_none());
    }
}
