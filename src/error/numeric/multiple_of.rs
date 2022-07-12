use crate::error::ToDefaultMessage;
use crate::validation::Number;

#[derive(Debug, Clone)]
pub struct MultipleOfErrorParams {
    pub multiple_of: Number,
}

impl MultipleOfErrorParams {
    pub fn new<N: Into<Number>>(multiple_of: N) -> Self {
        Self {
            multiple_of: multiple_of.into(),
        }
    }
}

impl ToDefaultMessage for MultipleOfErrorParams {
    fn to_default_message(&self) -> String {
        format!("the value must be multiple of `{}`.", self.multiple_of)
    }
}
