#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
    Bool(bool),
    Number(crate::Number),
    String(&'static str),
    Char(char),
    Null,
}

impl std::convert::From<bool> for Literal {
    fn from(item: bool) -> Self {
        Literal::Bool(item)
    }
}

impl<T> std::convert::From<T> for Literal
where
    T: Into<crate::Number>,
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

impl std::convert::From<char> for Literal {
    fn from(item: char) -> Self {
        Literal::Char(item)
    }
}

impl<T> std::convert::From<Option<T>> for Literal
where
    Literal: From<T>,
{
    fn from(item: Option<T>) -> Self {
        match item {
            Some(value) => std::convert::From::from(value),
            None => Literal::Null,
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Bool(value) => write!(f, "{value}"),
            Literal::Number(value) => write!(f, "{value}"),
            Literal::String(value) => write!(f, "{value}"),
            Literal::Char(value) => write!(f, "{value}"),
            Literal::Null => write!(f, "null"),
        }
    }
}
