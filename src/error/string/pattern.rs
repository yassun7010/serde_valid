use crate::error::ToDefaultMessage;
use regex::Regex;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PatternErrorParams {
    pub pattern: String,
}

impl PatternErrorParams {
    pub fn new(pattern: &Regex) -> Self {
        Self {
            pattern: format!("{:?}", pattern),
        }
    }
}

impl ToDefaultMessage for PatternErrorParams {
    fn to_default_message(&self) -> String {
        format!("the value must match the pattern of \"{}\".", self.pattern)
    }
}
