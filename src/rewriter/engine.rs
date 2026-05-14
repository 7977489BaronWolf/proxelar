use crate::rewriter::rule::{RewriteRule, RuleTarget};
use http::{HeaderMap, Request, Response};
use hyper::Body;

#[derive(Debug, Default)]
pub struct RewriteEngine {
    rules: Vec<RewriteRule>,
}

impl RewriteEngine {
    pub fn new(rules: Vec<RewriteRule>) -> Self {
        Self { rules }
    }

    pub fn add_rule(&mut self, rule: RewriteRule) {
        self.rules.push(rule);
    }

    pub fn remove_rule(&mut self, index: usize) -> Option<RewriteRule> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }

    pub fn apply_to_request(&self, mut req: Request<Body>) -> Request<Body> {
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            match rule.target {
                RuleTarget::RequestHeader => {
                    apply_header_rewrite(req.headers_mut(), &rule.find, &rule.replace);
                }
                RuleTarget::RequestUrl => {
                    let uri = req.uri().to_string();
                    let new_uri = uri.replace(&rule.find, &rule.replace);
                    if let Ok(parsed) = new_uri.parse() {
                        *req.uri_mut() = parsed;
                    }
                }
                _ => {}
            }
        }
        req
    }

    pub fn apply_to_response(&self, mut res: Response<Body>) -> Response<Body> {
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }
            if let RuleTarget::ResponseHeader = rule.target {
                apply_header_rewrite(res.headers_mut(), &rule.find, &rule.replace);
            }
        }
        res
    }

    pub fn rules(&self) -> &[RewriteRule] {
        &self.rules
    }
}

fn apply_header_rewrite(headers: &mut HeaderMap, find: &str, replace: &str) {
    let keys: Vec<_> = headers.keys().cloned().collect();
    for key in keys {
        if let Some(val) = headers.get(&key) {
            if let Ok(val_str) = val.to_str() {
                if val_str.contains(find) {
                    let new_val = val_str.replace(find, replace);
                    if let Ok(new_header_val) = new_val.parse() {
                        headers.insert(key, new_header_val);
                    }
                }
            }
        }
    }
}
