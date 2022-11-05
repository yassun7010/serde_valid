pub trait ToDefaultMessage {
    fn to_default_message(&self) -> String;
}

impl ToDefaultMessage for String {
    fn to_default_message(&self) -> String {
        self.into()
    }
}

#[derive(Clone)]
pub struct Message<E> {
    error: E,
    format_fn: for<'a> fn(&'a E) -> String,
}

impl<E> Message<E> {
    pub fn new(error: E, format_fn: fn(&E) -> String) -> Self {
        Self { error, format_fn }
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
