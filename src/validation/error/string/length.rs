use crate::validation::error::ToDefaultMessage;

#[derive(Debug, serde::Serialize)]
pub struct LengthParams {
    length: String,
    min_length: Option<usize>,
    max_length: Option<usize>,
}

impl LengthParams {
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

    #[allow(dead_code)]
    pub fn length(&self) -> &String {
        &self.length
    }

    #[allow(dead_code)]
    pub fn min_length(&self) -> Option<usize> {
        self.min_length
    }

    #[allow(dead_code)]
    pub fn max_length(&self) -> Option<usize> {
        self.max_length
    }
}

impl ToDefaultMessage for LengthParams {
    fn to_default_message(&self) -> String {
        let min_length = match &self.min_length {
            Some(length) => format!("{} <= ", length),
            None => String::new(),
        };
        let max_length = match &self.max_length {
            Some(length) => format!(" <= {}", length),
            None => String::new(),
        };
        format!(
            "length of {} must be in `{}length{}`, but not.",
            self.length, min_length, max_length
        )
    }
}
