use crate::validation;
use std::collections::{hash_map, HashMap};

pub type InnerErrors = HashMap<validation::FieldName, Vec<validation::Error>>;
pub type InnerErrorsIter<'a> = hash_map::Iter<'a, validation::FieldName, Vec<validation::Error>>;

#[derive(Debug, serde::Serialize, thiserror::Error)]
pub struct Errors(InnerErrors);

impl Errors {
    pub fn new(errors: InnerErrors) -> Self {
        Self(errors)
    }

    pub fn iter(&self) -> InnerErrorsIter<'_> {
        self.0.iter()
    }
}

impl std::fmt::Display for Errors {
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
