#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
pub struct Pattern(String);

macro_rules! impl_from_trait {
    ($type:ty) => {
        impl From<$type> for Pattern {
            fn from(item: $type) -> Self {
                Self(format!("{:?}", item))
            }
        }
    };
}

impl_from_trait!(regex::Regex);
impl_from_trait!(&regex::Regex);
impl_from_trait!(String);
impl_from_trait!(&str);

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.0)
    }
}
