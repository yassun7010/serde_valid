mod message;
mod numeric;

pub use message::Message;
pub use numeric::{MultiplesErrorInfo, RangeErrorInfo};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    RangeError(Message<RangeErrorInfo>),
    #[error("{0}")]
    MultiplesError(Message<MultiplesErrorInfo>),
    #[error("Length Error")]
    LengthError,
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
