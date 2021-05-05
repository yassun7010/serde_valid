pub trait ToDefaultMessage {
    fn to_default_message(&self) -> String;
}

impl ToDefaultMessage for String {
    fn to_default_message(&self) -> String {
        self.clone()
    }
}

pub struct Message<P>
where
    P: ToDefaultMessage,
{
    params: P,
    format_fn: for<'a> fn(&'a P) -> String,
}

impl<P> Message<P>
where
    P: ToDefaultMessage,
{
    pub fn new(params: P, format_fn: fn(&P) -> String) -> Self {
        Self { params, format_fn }
    }

    pub fn params(&self) -> &P {
        &self.params
    }
}

impl<P> std::fmt::Debug for Message<P>
where
    P: ToDefaultMessage + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message {{ params: {:?} }}", &self.params)
    }
}

impl<P> std::fmt::Display for Message<P>
where
    P: ToDefaultMessage,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", { self.format_fn }(&self.params))
    }
}
