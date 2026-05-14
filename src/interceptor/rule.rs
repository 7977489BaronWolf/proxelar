/// The action to take when a rule matches a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleAction {
    /// Allow the request to pass through.
    Forward,
    /// Block the request and return an error response.
    Block,
    /// Redirect the request to a different URL.
    Redirect(String),
}

/// A single interception rule consisting of a URL pattern and an action.
#[derive(Debug, Clone)]
pub struct InterceptRule {
    /// A simple substring pattern to match against request URLs.
    pub pattern: String,
    pub action: RuleAction,
}

impl InterceptRule {
    pub fn new(pattern: impl Into<String>, action: RuleAction) -> Self {
        Self {
            pattern: pattern.into(),
            action,
        }
    }

    /// Returns `true` if the given URL contains the rule's pattern.
    pub fn matches(&self, url: &str) -> bool {
        url.contains(&self.pattern)
    }
}
