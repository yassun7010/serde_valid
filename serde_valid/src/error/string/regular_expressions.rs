use crate::traits::IsMatch;
use regex::Regex;

#[derive(Debug, serde::Serialize)]
pub struct RegularExpressionErrorMessage {
    value: String,
    pattern: String,
}

impl RegularExpressionErrorMessage {
    pub fn new<T>(value: &T, pattern: &Regex) -> Self
    where
        T: IsMatch + ?Sized + std::fmt::Debug,
    {
        Self {
            value: format!("{:?}", value),
            pattern: format!("{:?}", pattern),
        }
    }
}

impl std::fmt::Display for RegularExpressionErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} must match the pattern of \"{}\", but not.",
            self.value, self.pattern
        )
    }
}
