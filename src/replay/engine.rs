use std::time::Duration;
use hyper::{Body, Client, Request, Response};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;

use crate::logger::entry::LogEntry;

#[derive(Debug, Clone)]
pub struct ReplayConfig {
    pub timeout: Duration,
    pub follow_redirects: bool,
    pub max_redirects: usize,
}

impl Default for ReplayConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            follow_redirects: true,
            max_redirects: 10,
        }
    }
}

pub struct ReplayEngine {
    config: ReplayConfig,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl ReplayEngine {
    pub fn new(config: ReplayConfig) -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder()
            .build::<_, Body>(https);
        Self { config, client }
    }

    pub async fn replay(&self, entry: &LogEntry) -> Result<Response<Body>, ReplayError> {
        let request = self.build_request(entry)?;
        self.client
            .request(request)
            .await
            .map_err(|e| ReplayError::HttpError(e.to_string()))
    }

    fn build_request(&self, entry: &LogEntry) -> Result<Request<Body>, ReplayError> {
        let mut builder = Request::builder()
            .method(entry.method.as_str())
            .uri(entry.uri.as_str());

        for (key, value) in &entry.request_headers {
            builder = builder.header(key, value);
        }

        let body = entry
            .request_body
            .clone()
            .map(Body::from)
            .unwrap_or_else(Body::empty);

        builder.body(body).map_err(|e| ReplayError::BuildError(e.to_string()))
    }

    pub fn config(&self) -> &ReplayConfig {
        &self.config
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReplayError {
    #[error("Failed to build request: {0}")]
    BuildError(String),
    #[error("HTTP error during replay: {0}")]
    HttpError(String),
    #[error("Timeout after {0:?}")]
    Timeout(Duration),
}
