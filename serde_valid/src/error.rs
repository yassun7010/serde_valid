mod array;
mod generic;
mod message;
mod numeric;
mod object;
mod string;

pub use array::{ItemsErrorInfo, UniqueItemsErrorInfo};
pub use generic::EnumerateErrorInfo;
pub use message::Message;
pub use numeric::{MultiplesErrorInfo, RangeErrorInfo};
pub use object::PropertiesErrorInfo;
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
    #[error("{0}")]
    PropertiesError(Message<PropertiesErrorInfo>),
    #[error("{0}")]
    EnumerateValuesError(Message<EnumerateErrorInfo>),
}
