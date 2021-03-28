mod array;
mod errors;
mod field_name;
mod generic;
mod message;
mod numeric;
mod object;
mod string;

pub use array::{ItemsParams, UniqueItemsParams};
pub use errors::{Errors, InnerErrors};
pub use field_name::FieldName;
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
    RangeError(Message<RangeParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MultipleOfError(Message<MultipleOfParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    LengthError(Message<LengthParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    PatternError(Message<PatternParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    ItemsError(Message<ItemsParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    UniqueItemsError(Message<UniqueItemsParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    PropertiesError(Message<PropertiesParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    EnumerateValuesError(Message<EnumerateParams>),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    CustomError(Message<String>),

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
