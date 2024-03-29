use crate::error::MinimumDurationError;
use std::time::Duration;
use crate::validation::{Composited, ValidateCompositedMinimumDuration};

/// Minimum duration validation.
/// Three suffix are allowed:
/// - ns
/// - ms
/// - s
///
/// ```rust
/// use std::time::Duration;
/// use serde_json::json;
/// use serde_valid::{Validate};
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(minimum_duration = "30s")]
///     val: Duration,
/// }
///
/// let s = TestStruct {
///     val: Duration::from_millis(20),
/// };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The duration must be >= 30000ms"]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMinimumDuration
{
    fn validate_minimum_duration(&self, minimum: Duration) -> Result<(), MinimumDurationError>;
}

impl ValidateMinimumDuration for Duration{
    fn validate_minimum_duration(&self, minimum: Duration) -> Result<(), MinimumDurationError> {
        if &minimum > self {
            Err(MinimumDurationError::new(minimum))
        }else {
            Ok(())
        }
    }
}

impl ValidateCompositedMinimumDuration<Duration> for Duration{
    fn validate_composited_duration(&self, limit: Duration) -> Result<(), Composited<MinimumDurationError>> {
        self.validate_minimum_duration(limit).map_err(|error|Composited::Single(error))
    }
}