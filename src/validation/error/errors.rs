use crate::validation;
use std::{collections::HashMap, fmt::Debug};

pub type VecErrors = Vec<validation::Error>;
pub type MapErrors = HashMap<&'static str, VecErrors>;

#[derive(Debug, serde::Serialize, thiserror::Error)]
#[serde(untagged)]
pub enum Errors {
    Fields(MapErrors),
    NewType(VecErrors),
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fields(map_errors) => {
                let mut fields_errors = HashMap::new();
                for (key, errors) in map_errors {
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
            Self::NewType(vec_errors) => {
                match serde_json::to_string(
                    &vec_errors
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<String>>(),
                ) {
                    Ok(json_string) => write!(f, "{}", json_string),
                    Err(_) => Err(std::fmt::Error),
                }
            }
        }
    }
}
