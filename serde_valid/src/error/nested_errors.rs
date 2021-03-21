use crate::error;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub struct NestedErrors(error::Errors);

impl NestedErrors {
    pub fn new(errors: error::Errors) -> Self {
        Self(errors)
    }
}

impl std::fmt::Display for NestedErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut new_errors = HashMap::new();
        for (key, errors) in &self.0 {
            let errors: Vec<String> = errors.iter().map(|e| format!("{}", e)).collect();
            new_errors.insert(key, errors);
        }
        match serde_json::to_string(&new_errors) {
            Ok(json_string) => write!(f, "{}", json_string),
            Err(_) => Err(std::fmt::Error),
        }
    }
}
