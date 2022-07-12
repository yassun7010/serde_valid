mod max_length;
mod min_length;
mod pattern;
pub use max_length::ValidateMaxLength;
pub use min_length::ValidateMinLength;
pub use pattern::ValidatePattern;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pattern(String);

macro_rules! impl_from {
    ($type:ty) => {
        impl From<$type> for Pattern {
            fn from(from: $type) -> Self {
                Self(format!("{:?}", from))
            }
        }
    };
}

impl_from!(regex::Regex);
impl_from!(&regex::Regex);
impl_from!(String);
impl_from!(&str);

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self.0)
    }
}
