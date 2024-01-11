pub trait DefaultFormat {
    fn default_format(&self) -> String;
}

impl DefaultFormat for String {
    fn default_format(&self) -> String {
        self.into()
    }
}
