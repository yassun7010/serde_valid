use super::{MapErrors, VecErrors};
use crate::validation;
use std::collections::HashMap;

#[derive(Debug, serde::Serialize)]
pub struct FieldsErrors(MapErrors);

impl FieldsErrors {
    pub fn new(errors: MapErrors) -> Self {
        Self(errors)
    }
}

impl IntoIterator for FieldsErrors {
    type Item = (validation::FieldName, VecErrors);
    type IntoIter = std::collections::hash_map::IntoIter<validation::FieldName, VecErrors>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::fmt::Display for FieldsErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields_errors = HashMap::new();
        for (key, errors) in &self.0 {
            fields_errors.insert(
                key,
                errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>(),
            );
        }
        match serde_json::to_string(&fields_errors) {
            Ok(json_string) => {
                write!(f, "{}", json_string)
            }
            Err(_) => Err(std::fmt::Error),
        }
    }
}
