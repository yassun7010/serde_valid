use crate::validation::{ArrayErrors, ObjectErrors};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum LocalizedError {
    String(String),
    Items(ArrayErrors<LocalizedError>),
    Properties(ObjectErrors<LocalizedError>),
}
