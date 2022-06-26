pub use crate::error::{
    EnumerateErrorParams, ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams,
    MaxItemsErrorParams, MaxLengthErrorParams, MaxPropertiesErrorParams, MaximumErrorParams,
    Message, MinItemsErrorParams, MinLengthErrorParams, MinPropertiesErrorParams,
    MinimumErrorParams, MultipleOfErrorParams, PatternErrorParams, UniqueItemsErrorParams,
};

use super::Errors;

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
