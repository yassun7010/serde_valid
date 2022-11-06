pub trait ToDefaultMessage {
    fn to_default_message(&self) -> String;
}

impl ToDefaultMessage for String {
    fn to_default_message(&self) -> String {
        self.into()
    }
}
