pub trait ToDefaultMessage {
    fn to_default_message(&self) -> String;
}

impl ToDefaultMessage for String {
    fn to_default_message(&self) -> String {
        self.into()
    }
}

pub struct Message<Params>
where
    Params: ToDefaultMessage,
{
    params: Params,
    format_fn: for<'a> fn(&'a Params) -> String,
}

impl<Params> Message<Params>
where
    Params: ToDefaultMessage,
{
    pub fn new(params: Params, format_fn: fn(&Params) -> String) -> Self {
        Self { params, format_fn }
    }

    pub fn params(&self) -> &Params {
        &self.params
    }
}

impl<Params> std::fmt::Debug for Message<Params>
where
    Params: ToDefaultMessage + std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message {{ params: {:?} }}", &self.params)
    }
}

impl<Params> std::fmt::Display for Message<Params>
where
    Params: ToDefaultMessage,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", { self.format_fn }(&self.params))
    }
}
