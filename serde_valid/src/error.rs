mod array;
mod message;
mod numeric;
mod string;

pub use array::{ItemsErrorInfo, UniqueItemsErrorInfo};
pub use message::Message;
pub use numeric::{MultiplesErrorInfo, RangeErrorInfo};
pub use string::{LengthErrorInfo, RegularExpressionErrorInfo};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    RangeError(Message<RangeErrorInfo>),
    #[error("{0}")]
    MultiplesError(Message<MultiplesErrorInfo>),
    #[error("{0}")]
    LengthError(Message<LengthErrorInfo>),
    #[error("{0}")]
    PatternError(Message<RegularExpressionErrorInfo>),
    #[error("{0}")]
    ItemsError(Message<ItemsErrorInfo>),
    #[error("{0}")]
    UniqueItemsError(Message<UniqueItemsErrorInfo>),
    #[error("Properties Error")]
    PropertiesError,
    #[error("EnumeratedValues Error")]
    EnumeratedValuesError,
}
