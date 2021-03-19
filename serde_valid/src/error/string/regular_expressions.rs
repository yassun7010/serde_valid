use crate::traits::IsMatch;
use regex::Regex;

#[derive(Debug)]
pub struct RegularExpressionErrorInfo {
    value: String,
    pattern: String,
}

impl RegularExpressionErrorInfo {
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

impl std::fmt::Display for RegularExpressionErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} must match the pattern of \"{}\", but not.",
            self.value, self.pattern
        )
    }
}
