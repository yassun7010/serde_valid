use crate::validation;

#[derive(Debug, thiserror::Error)]
pub enum Error<E>
where
    E: 'static + std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    #[error(transparent)]
    DeserializeError(#[from] E),

    #[error("{0}")]
    ValidationError(validation::Errors),
}

impl<E> Error<E>
where
    E: 'static + std::fmt::Debug + std::fmt::Display + std::error::Error,
{
    pub fn as_deserialize_error(&self) -> Option<&E> {
        match self {
            Self::DeserializeError(error) => Some(error),
            Self::ValidationError(_) => None,
        }
    }

    pub fn as_validation_errors(&self) -> Option<&validation::Errors> {
        match self {
            Self::DeserializeError(_) => None,
            Self::ValidationError(error) => Some(error),
        }
    }
}
