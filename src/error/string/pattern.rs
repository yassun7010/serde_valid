use crate::error::ToDefaultMessage;
use regex::Regex;

#[derive(Debug, serde::Serialize)]
pub struct PatternErrorParams {
    pattern: String,
}

impl PatternErrorParams {
    pub fn new(pattern: &Regex) -> Self {
        Self {
            pattern: format!("{:?}", pattern),
        }
    }

    #[allow(dead_code)]
    pub fn pattern(&self) -> &str {
        &self.pattern
    }
}

impl ToDefaultMessage for PatternErrorParams {
    fn to_default_message(&self) -> String {
        format!("the value must match the pattern of \"{}\".", self.pattern)
    }
}
