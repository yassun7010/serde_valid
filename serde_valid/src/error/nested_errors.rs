use crate::error::{self, FieldName};
use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub struct NestedErrors(HashMap<FieldName, Vec<error::Error>>);

impl NestedErrors {
    pub fn new(errors: HashMap<FieldName, Vec<error::Error>>) -> Self {
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
            Ok(json) => write!(f, "{}", json),
            Err(_) => Err(std::fmt::Error),
        }
    }
}
