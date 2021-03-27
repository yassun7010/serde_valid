use crate::traits::IsMatch;
use crate::validation::error::ToDefaultMessage;
use regex::Regex;

#[derive(Debug, serde::Serialize)]
pub struct RegularExpressionErrorParams {
    value: String,
    pattern: String,
}

impl RegularExpressionErrorParams {
    pub fn new<T>(value: &T, pattern: &Regex) -> Self
    where
        T: IsMatch + ?Sized + std::fmt::Debug,
    {
        Self {
            value: format!("{:?}", value),
            pattern: format!("{:?}", pattern),
        }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> &String {
        &self.value
    }

    #[allow(dead_code)]
    pub fn pattern(&self) -> &String {
        &self.pattern
    }
}

impl ToDefaultMessage for RegularExpressionErrorParams {
    fn to_default_message(&self) -> String {
        format!(
            "{} must match the pattern of \"{}\", but not.",
            self.value, self.pattern
        )
    }
}
