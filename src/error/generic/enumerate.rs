use itertools::Itertools;

use crate::{error::ToDefaultMessage, validation::Literal};

#[derive(Debug, Clone)]
pub struct EnumerateError {
    pub enumerate: Vec<Literal>,
}

impl EnumerateError {
    pub fn new<T>(enumerate: &[T]) -> Self
    where
        T: Into<Literal> + std::fmt::Debug + Clone,
    {
        Self {
            // FIXME: remove clone.
            enumerate: (*enumerate).iter().map(|x| x.clone().into()).collect(),
        }
    }
}

impl ToDefaultMessage for EnumerateError {
    fn to_default_message(&self) -> String {
        format!(
            "The value must be in [{:}].",
            self.enumerate.iter().map(|v| format!("{}", v)).join(", ")
        )
    }
}
