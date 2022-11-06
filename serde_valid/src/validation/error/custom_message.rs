pub struct CustomMessage<E> {
    pub message_fn: fn(&E) -> String,
    #[cfg(feature = "fluent")]
    pub fluent_message: Option<crate::fluent::Message>,
}
