use crate::error::ToDefaultMessage;

#[derive(Debug, Clone)]
pub struct FluentError {
    pub id: &'static str,
}

impl ToDefaultMessage for FluentError {
    #[inline]
    fn to_default_message(&self) -> String {
        self.id.to_string()
    }
}
