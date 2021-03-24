mod array;
mod errors;
mod field_name;
mod generic;
mod numeric;
mod object;
mod string;

pub use array::{ItemsErrorMessage, UniqueItemsErrorMessage};
pub use errors::{Errors, InnerErrors};
pub use field_name::FieldName;
pub use generic::EnumerateErrorMessage;
pub use numeric::{MultiplesErrorMessage, RangeErrorMessage};
pub use object::PropertiesErrorMessage;
pub use string::{LengthErrorMessage, RegularExpressionErrorMessage};

#[derive(Debug, serde::Serialize, thiserror::Error)]
#[serde(untagged)]
pub enum Error {
    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    RangeError(RangeErrorMessage),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    MultiplesError(MultiplesErrorMessage),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    LengthError(LengthErrorMessage),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    PatternError(RegularExpressionErrorMessage),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    ItemsError(ItemsErrorMessage),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    UniqueItemsError(UniqueItemsErrorMessage),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    PropertiesError(PropertiesErrorMessage),

    #[error("{0}")]
    #[serde(serialize_with = "serialize_error_message")]
    EnumerateValuesError(EnumerateErrorMessage),

    #[error("{0}")]
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
