#[cfg(test)]
mod tests {
    use crate::interceptor::{
        filter::InterceptFilter,
        rule::{InterceptRule, RuleAction},
    };

    fn make_filter() -> InterceptFilter {
        let mut filter = InterceptFilter::new();
        filter.add_rule(InterceptRule::new("example.com", RuleAction::Block));
        filter.add_rule(InterceptRule::new(
            "redirect.me",
            RuleAction::Redirect("https://safe.example.org".to_string()),
        ));
        filter
    }

    #[test]
    fn test_forward_when_no_rules_match() {
        let filter = make_filter();
        assert_eq!(filter.evaluate("https://other.org/path"), RuleAction::Forward);
    }

    #[test]
    fn test_block_rule_matches() {
        let filter = make_filter();
        assert_eq!(
            filter.evaluate("https://example.com/page"),
            RuleAction::Block
        );
    }

    #[test]
    fn test_redirect_rule_matches() {
        let filter = make_filter();
        assert_eq!(
            filter.evaluate("https://redirect.me/somewhere"),
            RuleAction::Redirect("https://safe.example.org".to_string())
        );
    }

    #[test]
    fn test_remove_rule() {
        let mut filter = make_filter();
        assert_eq!(filter.rules().len(), 2);
        let removed = filter.remove_rule(0).unwrap();
        assert_eq!(removed.pattern, "example.com");
        assert_eq!(filter.rules().len(), 1);
    }

    #[test]
    fn test_remove_out_of_bounds_returns_none() {
        let mut filter = make_filter();
        assert!(filter.remove_rule(99).is_none());
    }

    #[test]
    fn test_empty_filter_forwards_everything() {
        let filter = InterceptFilter::new();
        assert!(filter.is_empty());
        assert_eq!(filter.evaluate("https://anything.com"), RuleAction::Forward);
    }
}
