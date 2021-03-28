use crate::validation::error::ToDefaultMessage;

#[derive(Debug, serde::Serialize)]
pub struct EnumerateParams {
    value: String,
    enumerate: String,
}

impl EnumerateParams {
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

impl ToDefaultMessage for EnumerateParams {
    fn to_default_message(&self) -> String {
        format!("`{}` must be in {}, but not.", self.value, self.enumerate)
    }
}
