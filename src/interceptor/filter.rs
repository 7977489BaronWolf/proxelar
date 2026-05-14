use crate::interceptor::rule::{InterceptRule, RuleAction};

/// Manages a list of interception rules and applies them to request URLs.
#[derive(Debug, Default, Clone)]
pub struct InterceptFilter {
    rules: Vec<InterceptRule>,
}

impl InterceptFilter {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Add a rule to the filter.
    pub fn add_rule(&mut self, rule: InterceptRule) {
        self.rules.push(rule);
    }

    /// Remove a rule by index.
    pub fn remove_rule(&mut self, index: usize) -> Option<InterceptRule> {
        if index < self.rules.len() {
            Some(self.rules.remove(index))
        } else {
            None
        }
    }

    /// Evaluate the given URL against all rules in order.
    /// Returns the action of the first matching rule, or `RuleAction::Forward` if none match.
    pub fn evaluate(&self, url: &str) -> RuleAction {
        for rule in &self.rules {
            if rule.matches(url) {
                return rule.action.clone();
            }
        }
        RuleAction::Forward
    }

    pub fn rules(&self) -> &[InterceptRule] {
        &self.rules
    }

    pub fn is_empty(&self) -> bool {
        self.rules.is_empty()
    }
}
