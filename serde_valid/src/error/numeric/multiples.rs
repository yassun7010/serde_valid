#[derive(Debug)]
pub struct MultiplesErrorInfo {
    value: String,
    multiple_of: String,
}

impl MultiplesErrorInfo {
    pub fn new<T>(value: T, multiple_of: T) -> Self
    where
        T: PartialEq + std::ops::Rem<Output = T> + num_traits::Zero + ToString,
    {
        Self {
            value: value.to_string(),
            multiple_of: multiple_of.to_string(),
        }
    }
}

impl std::fmt::Display for MultiplesErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "`{}` must be multiple of `{}`, but not.",
            self.value, self.multiple_of
        )
    }
}
