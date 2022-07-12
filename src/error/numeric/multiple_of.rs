use crate::error::ToDefaultMessage;
use crate::validation::Number;

#[derive(Debug, Clone, serde::Serialize)]
pub struct MultipleOfErrorParams {
    multiple_of: Number,
}

impl MultipleOfErrorParams {
    pub fn new<N: Into<Number>>(multiple_of: N) -> Self {
        Self {
            multiple_of: multiple_of.into(),
        }
    }

    #[allow(dead_code)]
    pub fn multiple_of(&self) -> Number {
        self.multiple_of
    }
}

impl ToDefaultMessage for MultipleOfErrorParams {
    fn to_default_message(&self) -> String {
        format!("the value must be multiple of `{}`.", self.multiple_of)
    }
}
