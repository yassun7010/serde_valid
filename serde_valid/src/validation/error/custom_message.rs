#[derive(Clone)]
pub struct CustomMessage<E> {
    pub message_fn: fn(&E) -> String,
    #[cfg(feature = "fluent")]
    pub fluent_message: Option<crate::fluent::Message>,
}

impl<E> CustomMessage<E> {
    pub fn into_message(self, error: E) -> crate::error::Message<E> {
        let mut message = crate::error::Message::new(error, self.message_fn);

        #[cfg(feature = "fluent")]
        {
            message.fluent_message = self.fluent_message;
        }

        message
    }
}
