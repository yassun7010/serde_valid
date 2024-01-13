use crate::validation::{ArrayErrors, ObjectErrors};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum LocalizedError {
    String(String),
    Items(ArrayErrors<LocalizedError>),
    Properties(ObjectErrors<LocalizedError>),
}

impl std::fmt::Display for LocalizedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalizedError::String(string) => write!(f, "{}", string),
            LocalizedError::Items(items) => write!(f, "{}", items),
            LocalizedError::Properties(properties) => write!(f, "{}", properties),
        }
    }
}
