use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RuleTarget {
    RequestHeader,
    RequestUrl,
    ResponseHeader,
}

impl std::fmt::Display for RuleTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuleTarget::RequestHeader => write!(f, "Request Header"),
            RuleTarget::RequestUrl => write!(f, "Request URL"),
            RuleTarget::ResponseHeader => write!(f, "Response Header"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewriteRule {
    pub find: String,
    pub replace: String,
    pub target: RuleTarget,
    pub enabled: bool,
}

impl RewriteRule {
    pub fn new(find: impl Into<String>, replace: impl Into<String>, target: RuleTarget) -> Self {
        Self {
            find: find.into(),
            replace: replace.into(),
            target,
            enabled: true,
        }
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }

    pub fn matches(&self, input: &str) -> bool {
        self.enabled && input.contains(&self.find)
    }

    pub fn apply(&self, input: &str) -> String {
        if self.enabled {
            input.replace(&self.find, &self.replace)
        } else {
            input.to_string()
        }
    }
}

impl std::fmt::Display for RewriteRule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} '{}' -> '{}'",
            if self.enabled { "ON" } else { "OFF" },
            self.target,
            self.find,
            self.replace
        )
    }
}
