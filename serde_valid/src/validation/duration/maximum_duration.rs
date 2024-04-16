use crate::error::MaximumDurationError;
use std::time::Duration;
use crate::validation::{Composited, ValidateCompositedMaximumDuration};
/// Maximum duration validation.
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
///     #[validate(maximum_duration = "10ns")]
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
///                 "errors": ["The duration must be <= 0ms"]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateMaximumDuration
{
    fn validate_minimum_duration(&self, minimum: Duration) -> Result<(), MaximumDurationError>;
}

impl ValidateMaximumDuration for Duration{
    fn validate_minimum_duration(&self, maximum: Duration) -> Result<(), MaximumDurationError> {
        if &maximum < self {
            Err(MaximumDurationError::new(maximum))
        }else {

            Ok(())
        }
    }
}

impl ValidateCompositedMaximumDuration<Duration> for Duration{
    fn validate_composited_maximum_duration(&self, limit: Duration) -> Result<(), Composited<MaximumDurationError>> {
        self.validate_minimum_duration(limit).map_err(|error|Composited::Single(error))
    }
}