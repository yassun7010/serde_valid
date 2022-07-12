use crate::error::ToDefaultMessage;

#[derive(Debug, Clone)]
pub struct EnumerateErrorParams {
    pub enumerate: String,
}

impl EnumerateErrorParams {
    pub fn new<T>(enumerate: &[T]) -> Self
    where
        T: std::fmt::Debug,
    {
        Self {
            enumerate: format!("{:?}", enumerate),
        }
    }
}

impl ToDefaultMessage for EnumerateErrorParams {
    fn to_default_message(&self) -> String {
        format!("the value must be in {}.", self.enumerate)
    }
}
