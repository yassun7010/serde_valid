use crate::error::ToDefaultMessage;

#[derive(Debug, serde::Serialize)]
pub struct MultipleOfErrorParams {
    multiple_of: String,
}

impl MultipleOfErrorParams {
    pub fn new<T>(multiple_of: T) -> Self
    where
        T: PartialEq + std::ops::Rem<Output = T> + num_traits::Zero + ToString,
    {
        Self {
            multiple_of: multiple_of.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn multiple_of(&self) -> &String {
        &self.multiple_of
    }
}

impl ToDefaultMessage for MultipleOfErrorParams {
    fn to_default_message(&self) -> String {
        format!("value must be multiple of `{}`.", self.multiple_of)
    }
}
