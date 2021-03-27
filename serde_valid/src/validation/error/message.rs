pub struct Message<P> {
    params: P,
    format_fn: for<'a> fn(&'a P) -> String,
}

impl<P> Message<P> {
    pub fn new(params: P, format_fn: fn(&P) -> String) -> Self {
        Self { params, format_fn }
    }

    pub fn params(&self) -> &P {
        &self.params
    }
}

impl<P> std::fmt::Debug for Message<P>
where
    P: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message {{ params: {:?} }}", &self.params)
    }
}

impl<P> std::fmt::Display for Message<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", { self.format_fn }(&self.params))
    }
}
