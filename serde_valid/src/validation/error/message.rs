use super::{DefaultFormat, Format};

#[derive(Debug, Clone)]
pub struct Message<E> {
    error: E,
    format: Format<E>,
}

impl<E> Message<E> {
    pub fn new(error: E, format: Format<E>) -> Self {
        Self { error, format }
    }

    #[cfg(feature = "fluent")]
    pub fn fluent_message(&self) -> Option<&crate::features::fluent::Message> {
        match self.format {
            Format::Fluent(ref message) => Some(message),
            _ => None,
        }
    }
}

impl<E> DefaultFormat for Message<E>
where
    E: DefaultFormat,
{
    fn default_format(&self) -> String {
        match &self.format {
            Format::Default => self.error.default_format(),
            Format::Message(ref message) => message.to_string(),
            Format::MessageFn(ref format_fn) => format_fn(&self.error),
            #[cfg(feature = "fluent")]
            Format::Fluent(message) => format!("{message}"),
        }
    }
}

impl<E> std::fmt::Display for Message<E>
where
    E: DefaultFormat,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.default_format())
    }
}
