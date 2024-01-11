use crate::validation::error::DefaultFormat;

use super::Format;

pub trait IntoError<E>: Sized
where
    E: DefaultFormat,
{
    fn into_error(self) -> crate::validation::Error {
        self.into_error_by(Format {
            message_fn: E::default_format,
            #[cfg(feature = "fluent")]
            fluent_message: None,
        })
    }

    fn into_error_by(self, custom: Format<E>) -> crate::validation::Error;
}
