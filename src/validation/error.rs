pub use crate::error::{
    EnumerateErrorParams, ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams,
    MaxItemsErrorParams, MaxLengthErrorParams, MaxPropertiesErrorParams, MaximumErrorParams,
    Message, MinItemsErrorParams, MinLengthErrorParams, MinPropertiesErrorParams,
    MinimumErrorParams, MultipleOfErrorParams, PatternErrorParams, UniqueItemsErrorParams,
};
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, serde::Serialize, thiserror::Error)]
#[serde(untagged)]
pub enum Error {
    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Minimum(Message<MinimumErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Maximum(Message<MaximumErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    ExclusiveMinimum(Message<ExclusiveMinimumErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    ExclusiveMaximum(Message<ExclusiveMaximumErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MultipleOf(Message<MultipleOfErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MinLength(Message<MinLengthErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MaxLength(Message<MaxLengthErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Pattern(Message<PatternErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MinItems(Message<MinItemsErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MaxItems(Message<MaxItemsErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    UniqueItems(Message<UniqueItemsErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MinProperties(Message<MinPropertiesErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MaxProperties(Message<MaxPropertiesErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Enumerate(Message<EnumerateErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Custom(String),

    #[error(transparent)]
    Nested(Errors),
}

fn serialize_error_message<T, S>(message: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: serde::Serializer,
{
    serializer.serialize_str(&message.to_string())
}

pub type VecErrors = Vec<Error>;
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
