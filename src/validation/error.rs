mod array;
mod errors;
mod field_name;
mod generic;
mod message;
mod numeric;
mod object;
mod string;

pub use array::{ItemsErrorParams, UniqueItemsErrorParams};
pub use errors::{Errors, InnerErrors};
pub use field_name::FieldName;
pub use generic::EnumerateErrorParams;
pub use message::{Message, ToDefaultMessage};
pub use numeric::{MultiplesErrorParams, RangeErrorParams};
pub use object::PropertiesErrorParams;
pub use string::{LengthErrorParams, RegularExpressionErrorParams};

#[derive(Debug, serde::Serialize, thiserror::Error)]
#[serde(untagged)]
pub enum Error {
    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    RangeError(Message<RangeErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MultiplesError(Message<MultiplesErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    LengthError(Message<LengthErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    PatternError(Message<RegularExpressionErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    ItemsError(Message<ItemsErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    UniqueItemsError(Message<UniqueItemsErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    PropertiesError(Message<PropertiesErrorParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    EnumerateValuesError(Message<EnumerateErrorParams>),

    #[error(transparent)]
    NestedErrors(Errors),
}

pub fn serialize_error_message<T, S>(message: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: serde::Serializer,
{
    let s = format!("{}", message);
    serializer.serialize_str(&s)
}
