use crate::validation::numeric::Limit;

#[derive(Debug)]
pub struct RangeErrorMessage {
    value: String,
    maximum: Option<Limit<String>>,
    minimum: Option<Limit<String>>,
}

impl RangeErrorMessage {
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
}

impl std::fmt::Display for RangeErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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
        write!(
            f,
            "value must be in `{}value{}`, but `{}`.",
            minimum, maximum, self.value
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
