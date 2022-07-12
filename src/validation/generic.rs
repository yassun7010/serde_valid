mod enumerate;
pub use enumerate::{ValidateCompositedEnumerate, ValidateEnumerate};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Bool(bool),
    Number(crate::validation::Number),
    String(&'static str),
    None,
}

impl std::convert::From<bool> for Literal {
    fn from(item: bool) -> Self {
        Literal::Bool(item)
    }
}

impl<T> std::convert::From<T> for Literal
where
    T: Into<crate::validation::Number>,
{
    fn from(item: T) -> Self {
        Literal::Number(item.into())
    }
}

impl std::convert::From<&'static str> for Literal {
    fn from(item: &'static str) -> Self {
        Literal::String(item)
    }
}

impl<T> std::convert::From<Option<T>> for Literal
where
    Literal: From<T>,
{
    fn from(item: Option<T>) -> Self {
        match item {
            Some(value) => std::convert::From::from(value),
            None => Literal::None,
        }
    }
}
