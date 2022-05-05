use crate::error::ToDefaultMessage;

#[derive(Debug, serde::Serialize)]
pub struct EnumerateErrorParams {
    value: String,
    enumerate: String,
}

impl EnumerateErrorParams {
    pub fn new<T, U>(value: &T, enumerate: &[U]) -> Self
    where
        T: std::fmt::Debug,
        U: std::fmt::Debug,
    {
        Self {
            value: format!("{:?}", value),
            enumerate: format!("{:?}", enumerate),
        }
    }
}

impl ToDefaultMessage for EnumerateErrorParams {
    fn to_default_message(&self) -> String {
        format!("`{}` must be in {}, but not.", self.value, self.enumerate)
    }
}
