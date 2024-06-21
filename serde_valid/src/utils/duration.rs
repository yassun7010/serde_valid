use std::time::Duration;

/// Validate that the duration is less than or equal to the maximum.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use serde_valid::utils::duration_maximum;
/// use serde_valid::Validate;
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(custom = duration_maximum(Duration::from_micros(5)))]
///     val: Duration,
/// }
///
/// let s = TestStruct {
///     val: Duration::from_micros(5),
/// };
///
/// assert!(s.validate().is_ok());
/// ```
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

/// Validate that the duration is greater than or equal to the minimum.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use serde_valid::utils::duration_minimum;
/// use serde_valid::Validate;
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(custom = duration_minimum(Duration::from_micros(5)))]
///     val: Duration,
/// }
///
/// let s = TestStruct {
///     val: Duration::from_secs(5),
/// };
///
/// assert!(s.validate().is_ok());
/// ```
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

/// Validate that the duration is less than the exclusive maximum.
///     
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use serde_valid::utils::duration_exclusive_maximum;
/// use serde_valid::Validate;
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(custom = duration_exclusive_maximum(Duration::from_micros(5)))]
///     val: Duration,
/// }
///
/// let s = TestStruct {
///     val: Duration::from_micros(4),
/// };
///
/// assert!(s.validate().is_ok());
/// ```
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

/// Validate that the duration is greater than the exclusive minimum.
///     
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use serde_valid::utils::duration_exclusive_minimum;
/// use serde_valid::Validate;
///  
/// #[derive(Validate)]
/// struct TestStruct {
///      #[validate(custom = duration_exclusive_minimum(Duration::from_micros(5)))]
///      val: Duration,
/// }
///
/// let s = TestStruct {
///     val: Duration::from_micros(6),
/// };
///
/// assert!(s.validate().is_ok());
/// ```
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
