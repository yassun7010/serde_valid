use super::{DefaultFormat, Format};

#[derive(Clone)]
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

impl<E> std::fmt::Debug for Message<E>
where
    E: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message {{ error: {:?} }}", &self.error)
    }
}

impl<E> std::fmt::Display for Message<E>
where
    E: DefaultFormat,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.format {
            Format::Default => write!(f, "{}", self.error.default_format()),
            Format::Message(ref message) => write!(f, "{message}"),
            Format::MessageFn(ref format_fn) => write!(f, "{}", { format_fn }(&self.error)),
            #[cfg(feature = "fluent")]
            Format::Fluent(_) => write!(f, "{}", self.error.default_format()),
        }
    }
}
