#[cfg(test)]
mod tests {
    use crate::rewriter::engine::RewriteEngine;
    use crate::rewriter::rule::{RewriteRule, RuleTarget};
    use http::Request;
    use hyper::Body;

    fn make_rule(find: &str, replace: &str, target: RuleTarget, enabled: bool) -> RewriteRule {
        RewriteRule {
            find: find.to_string(),
            replace: replace.to_string(),
            target,
            enabled,
        }
    }

    #[test]
    fn test_add_and_remove_rule() {
        let mut engine = RewriteEngine::default();
        let rule = make_rule("foo", "bar", RuleTarget::RequestUrl, true);
        engine.add_rule(rule);
        assert_eq!(engine.rules().len(), 1);
        let removed = engine.remove_rule(0);
        assert!(removed.is_some());
        assert_eq!(engine.rules().len(), 0);
    }

    #[test]
    fn test_remove_out_of_bounds() {
        let mut engine = RewriteEngine::default();
        let removed = engine.remove_rule(5);
        assert!(removed.is_none());
    }

    #[test]
    fn test_apply_url_rewrite() {
        let rule = make_rule("example.com", "replaced.com", RuleTarget::RequestUrl, true);
        let engine = RewriteEngine::new(vec![rule]);

        let req = Request::builder()
            .uri("http://example.com/path")
            .body(Body::empty())
            .unwrap();

        let rewritten = engine.apply_to_request(req);
        assert!(rewritten.uri().to_string().contains("replaced.com"));
    }

    #[test]
    fn test_disabled_rule_not_applied() {
        let rule = make_rule("example.com", "replaced.com", RuleTarget::RequestUrl, false);
        let engine = RewriteEngine::new(vec![rule]);

        let req = Request::builder()
            .uri("http://example.com/path")
            .body(Body::empty())
            .unwrap();

        let rewritten = engine.apply_to_request(req);
        assert!(rewritten.uri().to_string().contains("example.com"));
    }

    #[test]
    fn test_request_header_rewrite() {
        let rule = make_rule("old-value", "new-value", RuleTarget::RequestHeader, true);
        let engine = RewriteEngine::new(vec![rule]);

        let req = Request::builder()
            .uri("http://example.com")
            .header("x-custom", "old-value")
            .body(Body::empty())
            .unwrap();

        let rewritten = engine.apply_to_request(req);
        let header_val = rewritten.headers().get("x-custom").unwrap().to_str().unwrap();
        assert_eq!(header_val, "new-value");
    }
}
