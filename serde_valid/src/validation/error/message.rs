#[derive(Clone)]
pub struct Message<E> {
    error: E,
    format_fn: for<'a> fn(&'a E) -> String,
    #[cfg(feature = "fluent")]
    pub fluent_message: Option<crate::fluent::Message>,
}

impl<E> Message<E> {
    pub fn new(error: E, format_fn: fn(&E) -> String) -> Self {
        Self {
            error,
            format_fn,
            #[cfg(feature = "fluent")]
            fluent_message: None,
        }
    }

    pub fn error(&self) -> &E {
        &self.error
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

impl<E> std::fmt::Display for Message<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", { self.format_fn }(&self.error))
    }
}
