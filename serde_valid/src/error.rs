mod message;
mod numeric;
mod string;

pub use message::Message;
pub use numeric::{MultiplesErrorInfo, RangeErrorInfo};
pub use string::LengthErrorInfo;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    RangeError(Message<RangeErrorInfo>),
    #[error("{0}")]
    MultiplesError(Message<MultiplesErrorInfo>),
    #[error("{0}")]
    LengthError(Message<LengthErrorInfo>),
    #[error("Pattern Error")]
    PatternError,
    #[error("Items Error")]
    ItemsError,
    #[error("UniqueItems Error")]
    UniqueItemsError,
    #[error("Properties Error")]
    PropertiesError,
    #[error("EnumeratedValues Error")]
    EnumeratedValuesError,
}
