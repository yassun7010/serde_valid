mod array;
mod field_name;
mod generic;
mod numeric;
mod object;
mod string;

pub use array::{ItemsErrorMessage, UniqueItemsErrorMessage};
pub use field_name::FieldName;
pub use generic::EnumerateErrorMessage;
pub use numeric::{MultiplesErrorMessage, RangeErrorMessage};
pub use object::PropertiesErrorMessage;
pub use string::{LengthErrorMessage, RegularExpressionErrorMessage};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    RangeError(RangeErrorMessage),
    #[error("{0}")]
    MultiplesError(MultiplesErrorMessage),
    #[error("{0}")]
    LengthError(LengthErrorMessage),
    #[error("{0}")]
    PatternError(RegularExpressionErrorMessage),
    #[error("{0}")]
    ItemsError(ItemsErrorMessage),
    #[error("{0}")]
    UniqueItemsError(UniqueItemsErrorMessage),
    #[error("{0}")]
    PropertiesError(PropertiesErrorMessage),
    #[error("{0}")]
    EnumerateValuesError(EnumerateErrorMessage),
}
