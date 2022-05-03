mod array;
mod errors;
mod generic;
mod message;
mod numeric;
mod object;
mod string;

pub use array::{MaxItemsParams, MinItemsParams, UniqueItemsParams};
pub use errors::{Errors, MapErrors, VecErrors};
pub use generic::EnumerateParams;
pub use message::{Message, ToDefaultMessage};
pub use numeric::{MultipleOfParams, RangeParams};
pub use object::PropertiesParams;
pub use string::{LengthParams, PatternParams};

#[derive(Debug, serde::Serialize, thiserror::Error)]
#[serde(untagged)]
pub enum Error {
    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Range(Message<RangeParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MultipleOf(Message<MultipleOfParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Length(Message<LengthParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Pattern(Message<PatternParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MinItems(Message<MinItemsParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MaxItems(Message<MaxItemsParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    UniqueItems(Message<UniqueItemsParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Properties(Message<PropertiesParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    Enumerate(Message<EnumerateParams>),

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
