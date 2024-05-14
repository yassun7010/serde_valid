use std::time::Duration;

#[allow(dead_code)]
pub fn duration_maximum(
    maximum: Duration,
) -> impl FnOnce(&Duration) -> Result<(), crate::validation::Error> {
    move |val: &Duration| {
        if *val <= maximum {
            Ok(())
        } else {
            Err(crate::validation::Error::Custom(format!(
                "Duration {val:?} is greater than maximum {maximum:?}.",
            )))
        }
    }
}

#[allow(dead_code)]
pub fn duration_minimum(
    minimum: Duration,
) -> impl FnOnce(&Duration) -> Result<(), crate::validation::Error> {
    move |val: &Duration| {
        if *val >= minimum {
            Ok(())
        } else {
            Err(crate::validation::Error::Custom(format!(
                "Duration {val:?} is less than minimum {minimum:?}.",
            )))
        }
    }
}

#[allow(dead_code)]
pub fn duration_exclusive_maximum(
    maximum: Duration,
) -> impl FnOnce(&Duration) -> Result<(), crate::validation::Error> {
    move |val: &Duration| {
        if *val < maximum {
            Ok(())
        } else {
            Err(crate::validation::Error::Custom(format!(
                "Duration {val:?} is greater than or equal to exclusive maximum {maximum:?}.",
            )))
        }
    }
}

#[allow(dead_code)]
pub fn duration_exclusive_minimum(
    minimum: Duration,
) -> impl FnOnce(&Duration) -> Result<(), crate::validation::Error> {
    move |val: &Duration| {
        if *val > minimum {
            Ok(())
        } else {
            Err(crate::validation::Error::Custom(format!(
                "Duration {val:?} is less than or equal to exclusive minimum {minimum:?}.",
            )))
        }
    }
}
