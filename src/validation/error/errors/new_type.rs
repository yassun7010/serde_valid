use super::VecErrors;
use crate::validation;

#[derive(Debug, serde::Serialize, thiserror::Error)]
pub struct NewTypeErrors(VecErrors);

impl NewTypeErrors {
    pub fn new(errors: VecErrors) -> Self {
        Self(errors)
    }
}

impl IntoIterator for NewTypeErrors {
    type Item = validation::Error;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::fmt::Display for NewTypeErrors {
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
