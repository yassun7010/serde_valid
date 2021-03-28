use crate::validation::error::ToDefaultMessage;
use crate::validation::numeric::Limit;

#[derive(Debug)]
pub struct RangeParams {
    value: String,
    maximum: Option<Limit<String>>,
    minimum: Option<Limit<String>>,
}

impl RangeParams {
    pub fn new<T>(value: T, minimum: Option<Limit<T>>, maximum: Option<Limit<T>>) -> Self
    where
        T: PartialOrd + PartialEq + ToString,
    {
        Self {
            value: value.to_string(),
            minimum: limit_numeric_to_string(minimum),
            maximum: limit_numeric_to_string(maximum),
        }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> &String {
        &self.value
    }

    #[allow(dead_code)]
    pub fn minimum(&self) -> Option<&Limit<String>> {
        self.minimum.as_ref()
    }

    #[allow(dead_code)]
    pub fn maximum(&self) -> Option<&Limit<String>> {
        self.maximum.as_ref()
    }
}

impl ToDefaultMessage for RangeParams {
    fn to_default_message(&self) -> String {
        let minimum = if let Some(limit) = &self.minimum {
            match limit {
                Limit::Inclusive(inclusive) => format!("{} <= ", inclusive),
                Limit::Exclusive(excluseve) => format!("{} < ", excluseve),
            }
        } else {
            String::new()
        };
        let maximum = if let Some(limit) = &self.maximum {
            match limit {
                Limit::Inclusive(inclusive) => format!(" <= {}", inclusive),
                Limit::Exclusive(excluseve) => format!(" < {}", excluseve),
            }
        } else {
            String::new()
        };
        format!(
            "`{}` must be in `{}value{}`, but not.",
            self.value, minimum, maximum
        )
    }
}

fn limit_numeric_to_string<T>(limit_value: Option<Limit<T>>) -> Option<Limit<String>>
where
    T: PartialOrd + PartialEq + ToString,
{
    match limit_value {
        Some(limit) => match limit {
            Limit::Inclusive(inclusive) => Some(Limit::Inclusive(inclusive.to_string())),
            Limit::Exclusive(exclusive) => Some(Limit::Exclusive(exclusive.to_string())),
        },
        None => None,
    }
}
