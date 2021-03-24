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
