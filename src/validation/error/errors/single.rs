use super::VecErrors;
use crate::validation;
use core::slice::Iter;

#[derive(Debug, serde::Serialize, thiserror::Error)]
pub struct SingleErrors(VecErrors);

impl SingleErrors {
    pub fn new(errors: VecErrors) -> Self {
        Self(errors)
    }

    pub fn iter(&self) -> Iter<'_, validation::Error> {
        self.0.iter()
    }
}

impl std::fmt::Display for SingleErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(
            &self
                .0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>(),
        ) {
            Ok(json_string) => write!(f, "{}", json_string),
            Err(_) => Err(std::fmt::Error),
        }
    }
}
