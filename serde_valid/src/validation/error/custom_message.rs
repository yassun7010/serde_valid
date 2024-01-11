#[derive(Clone)]
pub struct CustomMessage<E> {
    pub message_fn: fn(&E) -> String,
    #[cfg(feature = "fluent")]
    pub fluent_message: Option<crate::fluent::Message>,
}

impl<E> CustomMessage<E> {
    #[cfg(feature = "fluent")]
    pub fn into_message(self, error: E) -> crate::validation::Message<E> {
        let mut message = crate::validation::Message::new(error, self.message_fn);
        message.fluent_message = self.fluent_message;
        message
    }

    #[cfg(not(feature = "fluent"))]
    pub fn into_message(self, error: E) -> crate::validation::Message<E> {
        crate::validation::Message::new(error, self.message_fn)
    }
}

impl<E> Default for CustomMessage<E>
where
    E: crate::validation::ToDefaultMessage,
{
    fn default() -> Self {
        Self {
            message_fn: crate::validation::ToDefaultMessage::to_default_message,
            #[cfg(feature = "fluent")]
            fluent_message: None,
        }
    }
}
