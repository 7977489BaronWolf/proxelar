mod rule;
mod tests;

pub use rule::{RewriteRule, RewriteTarget};

use hyper::{Body, Request, Response};

/// Applies a list of rewrite rules to an outgoing request.
pub fn apply_request_rewrites(
    mut req: Request<Body>,
    rules: &[RewriteRule],
) -> Request<Body> {
    for rule in rules {
        if !rule.is_enabled() {
            continue;
        }
        req = rule.apply_to_request(req);
    }
    req
}

/// Applies a list of rewrite rules to an incoming response.
pub fn apply_response_rewrites(
    mut res: Response<Body>,
    rules: &[RewriteRule],
) -> Response<Body> {
    for rule in rules {
        if !rule.is_enabled() {
            continue;
        }
        res = rule.apply_to_response(res);
    }
    res
}
