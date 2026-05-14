use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
    Other(String),
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::HEAD => write!(f, "HEAD"),
            HttpMethod::OPTIONS => write!(f, "OPTIONS"),
            HttpMethod::Other(m) => write!(f, "{}", m),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub id: u64,
    pub timestamp: u64,
    pub method: HttpMethod,
    pub url: String,
    pub status_code: Option<u16>,
    pub request_headers: Vec<(String, String)>,
    pub response_headers: Vec<(String, String)>,
    pub request_body: Option<Vec<u8>>,
    pub response_body: Option<Vec<u8>>,
}

impl LogEntry {
    pub fn new(id: u64, method: HttpMethod, url: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            id,
            timestamp,
            method,
            url,
            status_code: None,
            request_headers: Vec::new(),
            response_headers: Vec::new(),
            request_body: None,
            response_body: None,
        }
    }

    pub fn set_response(&mut self, status_code: u16, headers: Vec<(String, String)>, body: Option<Vec<u8>>) {
        self.status_code = Some(status_code);
        self.response_headers = headers;
        self.response_body = body;
    }
}
