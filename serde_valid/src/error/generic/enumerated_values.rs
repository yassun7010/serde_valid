#[derive(Debug)]
pub struct EnumerateErrorInfo {
    value: String,
    enumerate: String,
}

impl EnumerateErrorInfo {
    pub fn new<T, U>(value: &T, enumerate: &[U]) -> Self
    where
        T: std::fmt::Debug,
        U: std::fmt::Debug,
    {
        Self {
            value: format!("{:?}", value),
            enumerate: format!("{:?}", enumerate),
        }
    }
}

impl std::fmt::Display for EnumerateErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "`{}` must be in {}, but not.",
            self.value, self.enumerate
        )
    }
}
