use hyper::header::{HeaderName, HeaderValue};
use hyper::{Body, Request, Response};
use std::str::FromStr;

/// Describes what part of a request/response to rewrite.
#[derive(Debug, Clone, PartialEq)]
pub enum RewriteTarget {
    RequestHeader,
    ResponseHeader,
    RequestUrl,
}

/// A single rewrite rule that can mutate requests or responses.
#[derive(Debug, Clone)]
pub struct RewriteRule {
    pub target: RewriteTarget,
    /// Header name (used when target is a header variant)
    pub key: String,
    /// Value to inject or replace with
    pub value: String,
    enabled: bool,
}

impl RewriteRule {
    pub fn new(target: RewriteTarget, key: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            target,
            key: key.into(),
            value: value.into(),
            enabled: true,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    /// Apply this rule to a request, returning the (possibly modified) request.
    pub fn apply_to_request(&self, mut req: Request<Body>) -> Request<Body> {
        match self.target {
            RewriteTarget::RequestHeader => {
                if let (Ok(name), Ok(val)) = (
                    HeaderName::from_str(&self.key),
                    HeaderValue::from_str(&self.value),
                ) {
                    req.headers_mut().insert(name, val);
                }
            }
            RewriteTarget::RequestUrl => {
                if let Ok(uri) = self.value.parse() {
                    *req.uri_mut() = uri;
                }
            }
            _ => {}
        }
        req
    }

    /// Apply this rule to a response, returning the (possibly modified) response.
    pub fn apply_to_response(&self, mut res: Response<Body>) -> Response<Body> {
        if self.target == RewriteTarget::ResponseHeader {
            if let (Ok(name), Ok(val)) = (
                HeaderName::from_str(&self.key),
                HeaderValue::from_str(&self.value),
            ) {
                res.headers_mut().insert(name, val);
            }
        }
        res
    }
}
