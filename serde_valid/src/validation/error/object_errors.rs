use super::{PropertyErrorsMap, VecErrors};

#[derive(Debug, Clone, serde::Serialize, thiserror::Error)]
pub struct ObjectErrors {
    pub errors: VecErrors,
    pub properties: PropertyErrorsMap,
}

impl ObjectErrors {
    pub fn new(errors: VecErrors, properties: PropertyErrorsMap) -> Self {
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
