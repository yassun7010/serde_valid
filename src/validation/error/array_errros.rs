use indexmap::IndexMap;

use super::{Errors, VecErrors};

#[derive(Debug, serde::Serialize, thiserror::Error)]
pub struct ArrayErrors {
    errors: VecErrors,
    items: IndexMap<usize, Errors>,
}

impl ArrayErrors {
    pub fn new(errors: VecErrors, items: IndexMap<usize, Errors>) -> Self {
        Self { errors, items }
    }
}

impl std::fmt::Display for ArrayErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(json_string) => {
                write!(f, "{}", json_string)
            }
            Err(_) => Err(std::fmt::Error),
        }
    }
}
