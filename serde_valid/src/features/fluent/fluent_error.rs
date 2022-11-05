use std::collections::HashMap;

use crate::error::ToDefaultMessage;

#[derive(Debug, Clone)]
pub struct FluentError {
    pub id: &'static str,
    pub args: HashMap<&'static str, serde_valid_literal::Literal>,
}

impl std::fmt::Display for FluentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl ToDefaultMessage for FluentError {
    #[inline]
    fn to_default_message(&self) -> String {
        self.id.to_string()
    }
}
