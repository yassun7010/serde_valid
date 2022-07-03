use indexmap::IndexMap;

use super::{Errors, VecErrors};

#[derive(Debug, serde::Serialize, thiserror::Error)]
pub struct ObjectErrors {
    pub errors: VecErrors,
    pub properties: IndexMap<&'static str, Errors>,
}

impl ObjectErrors {
    pub fn new(errors: VecErrors, properties: IndexMap<&'static str, Errors>) -> Self {
        Self { errors, properties }
    }
}

impl std::fmt::Display for ObjectErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(json_string) => {
                write!(f, "{}", json_string)
            }
            Err(_) => Err(std::fmt::Error),
        }
    }
}
