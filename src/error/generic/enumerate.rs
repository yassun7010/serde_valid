use crate::error::ToDefaultMessage;

#[derive(Debug, serde::Serialize)]
pub struct EnumerateErrorParams {
    enumerate: String,
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

    #[allow(dead_code)]
    pub fn enumerate(&self) -> &str {
        &self.enumerate
    }
}

impl ToDefaultMessage for EnumerateErrorParams {
    fn to_default_message(&self) -> String {
        format!("value must be in {}.", self.enumerate)
    }
}
