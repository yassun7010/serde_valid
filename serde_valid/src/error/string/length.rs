#[derive(Debug, serde::Serialize)]
pub struct LengthErrorMessage {
    length: String,
    min_length: Option<usize>,
    max_length: Option<usize>,
}

impl LengthErrorMessage {
    pub fn new<T>(length: T, min_length: Option<usize>, max_length: Option<usize>) -> Self
    where
        T: PartialOrd + PartialEq + std::fmt::Debug,
    {
        Self {
            length: format!("{:?}", length),
            min_length,
            max_length,
        }
    }
}

impl std::fmt::Display for LengthErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_length = match &self.min_length {
            Some(length) => format!("{} <= ", length),
            None => String::new(),
        };
        let max_length = match &self.max_length {
            Some(length) => format!(" <= {}", length),
            None => String::new(),
        };
        write!(
            f,
            "length of {} must be in `{}length{}`, but not.",
            self.length, min_length, max_length
        )
    }
}
