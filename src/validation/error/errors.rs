mod fields;
mod single;

use crate::validation;
pub use fields::FieldsErrors;
pub use single::SingleErrors;
use std::{collections::HashMap, fmt::Debug};

pub type VecErrors = Vec<validation::Error>;
pub type MapErrors = HashMap<validation::FieldName, VecErrors>;

#[derive(Debug, serde::Serialize, thiserror::Error)]
#[serde(untagged)]
pub enum Errors {
    Fields(FieldsErrors),
    Single(SingleErrors),
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fields(map_errors) => std::fmt::Display::fmt(map_errors, f),
            Self::Single(vec_errors) => std::fmt::Display::fmt(vec_errors, f),
        }
    }
}
