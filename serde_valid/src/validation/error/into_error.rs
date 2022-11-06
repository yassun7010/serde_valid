use crate::error::ToDefaultMessage;

use super::CustomMessage;

pub trait IntoError<E>: Sized
where
    E: ToDefaultMessage,
{
    fn into_error(self) -> crate::validation::Error {
        self.into_error_by(&CustomMessage {
            message_fn: E::to_default_message,
            #[cfg(feature = "fluent")]
            fluent_message: None,
        })
    }

    fn into_error_by(self, custom: &CustomMessage<E>) -> crate::validation::Error;
}
