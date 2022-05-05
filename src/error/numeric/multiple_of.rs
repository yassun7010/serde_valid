use crate::error::ToDefaultMessage;

#[derive(Debug, serde::Serialize)]
pub struct MultipleOfErrorParams {
    value: String,
    multiple_of: String,
}

impl MultipleOfErrorParams {
    pub fn new<T>(value: T, multiple_of: T) -> Self
    where
        T: PartialEq + std::ops::Rem<Output = T> + num_traits::Zero + ToString,
    {
        Self {
            value: value.to_string(),
            multiple_of: multiple_of.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> &String {
        &self.value
    }

    #[allow(dead_code)]
    pub fn multiple_of(&self) -> &String {
        &self.multiple_of
    }
}

impl ToDefaultMessage for MultipleOfErrorParams {
    fn to_default_message(&self) -> String {
        format!(
            "`{}` must be multiple of `{}`, but not.",
            self.value, self.multiple_of
        )
    }
}
