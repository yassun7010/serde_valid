#[derive(Clone)]
pub enum Format<E> {
    Default,
    Message(String),
    MessageFn(fn(&E) -> String),
    #[cfg(feature = "fluent")]
    Fluent(crate::fluent::Message),
}

impl<E> Format<E> {
    pub fn into_message(self, error: E) -> crate::validation::error::Message<E>
    where
        E: DefaultFormat,
    {
        crate::validation::error::Message::new(error, self)
    }
}

pub trait DefaultFormat {
    fn default_format(&self) -> String;
}
