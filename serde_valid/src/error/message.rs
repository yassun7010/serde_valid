#[derive(Debug)]
pub struct Message<P> {
    field_name: String,
    params: P,
}

impl<P> Message<P> {
    pub fn new<T: Into<String>>(field_name: T, params: P) -> Self {
        Self {
            field_name: field_name.into(),
            params,
        }
    }
}

impl<P> std::fmt::Display for Message<P>
where
    P: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field_name, self.params)
    }
}
