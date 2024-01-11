#[derive(Clone)]
pub struct Format<E> {
    pub message_fn: fn(&E) -> String,
    #[cfg(feature = "fluent")]
    pub fluent_message: Option<crate::fluent::Message>,
}

impl<E> Format<E> {
    #[cfg(feature = "fluent")]
    pub fn into_message(self, error: E) -> crate::validation::error::Message<E> {
        let mut message = crate::validation::error::Message::new(error, self.message_fn);
        message.fluent_message = self.fluent_message;
        message
    }

    #[cfg(not(feature = "fluent"))]
    pub fn into_message(self, error: E) -> crate::validation::error::Message<E> {
        crate::validation::error::Message::new(error, self.message_fn)
    }
}

impl<E> Default for Format<E>
where
    E: crate::validation::error::DefaultFormat,
{
    fn default() -> Self {
        Self {
            message_fn: crate::validation::error::DefaultFormat::default_format,
            #[cfg(feature = "fluent")]
            fluent_message: None,
        }
    }
}

pub trait DefaultFormat {
    fn default_format(&self) -> String;
}

impl DefaultFormat for String {
    fn default_format(&self) -> String {
        self.into()
    }
}
