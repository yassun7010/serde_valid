mod array;
mod generic;
mod message;
mod numeric;
mod object;
mod string;

use crate::validation;
pub use array::{MaxItemsErrorParams, MinItemsErrorParams, UniqueItemsErrorParams};
pub use generic::EnumerateErrorParams;
pub use message::{Message, ToDefaultMessage};
pub use numeric::{
    ExclusiveMaximumErrorParams, ExclusiveMinimumErrorParams, MaximumErrorParams,
    MinimumErrorParams, MultipleOfErrorParams,
};
pub use object::{MaxPropertiesErrorParams, MinPropertiesErrorParams};
pub use string::{MaxLengthErrorParams, MinLengthErrorParams, PatternErrorParams};

#[derive(Debug, thiserror::Error)]
pub enum Error<E>
where
    E: 'static + std::error::Error,
{
    #[error(transparent)]
    DeserializeError(#[from] E),

    #[error(transparent)]
    ValidationError(validation::Errors),
}

impl<E> Error<E>
where
    E: 'static + std::error::Error,
{
    pub fn is_serde_error(&self) -> bool {
        match self {
            Self::DeserializeError(_) => true,
            Self::ValidationError(_) => false,
        }
    }

    pub fn as_serde_error(&self) -> Option<&E> {
        match self {
            Self::DeserializeError(error) => Some(error),
            Self::ValidationError(_) => None,
        }
    }

    pub fn is_validation_errors(&self) -> bool {
        match self {
            Self::DeserializeError(_) => false,
            Self::ValidationError(_) => true,
        }
    }

    pub fn as_validation_errors(&self) -> Option<&validation::Errors> {
        match self {
            Self::DeserializeError(_) => None,
            Self::ValidationError(error) => Some(error),
        }
    }
}
