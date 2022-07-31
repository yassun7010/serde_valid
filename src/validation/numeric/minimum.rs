use crate::validation::{impl_generic_composited_validation_1args, ValidateCompositedMinimum};
use crate::MinimumErrorParams;

/// Minimum validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateMinimum};
/// struct MyType(i32);
///
/// impl ValidateMinimum<i32> for MyType {
///     fn validate_minimum(&self, minimum: i32) -> Result<(), serde_valid::MinimumErrorParams> {
///         self.0.validate_minimum(minimum)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(minimum = 5)]
///     val: MyType,
/// }
///
/// let s = TestStruct { val: MyType(3) };
///
/// assert_eq!(
///     serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
///     serde_json::to_string(&json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The number must be `>= 5`."]
///             }
///         }
///     }))
///     .unwrap()
/// );
/// ```
pub trait ValidateMinimum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_minimum(&self, minimum: T) -> Result<(), MinimumErrorParams>;
}

macro_rules! impl_validate_numeric_minimum {
    ($type:ty) => {
        impl ValidateMinimum<$type> for $type {
            fn validate_minimum(&self, minimum: $type) -> Result<(), MinimumErrorParams> {
                if *self >= minimum {
                    Ok(())
                } else {
                    Err(MinimumErrorParams::new(minimum))
                }
            }
        }

        impl_generic_composited_validation_1args!(Minimum, $type);
    };
}

impl_validate_numeric_minimum!(i8);
impl_validate_numeric_minimum!(i16);
impl_validate_numeric_minimum!(i32);
impl_validate_numeric_minimum!(i64);
impl_validate_numeric_minimum!(i128);
impl_validate_numeric_minimum!(isize);
impl_validate_numeric_minimum!(u8);
impl_validate_numeric_minimum!(u16);
impl_validate_numeric_minimum!(u32);
impl_validate_numeric_minimum!(u64);
impl_validate_numeric_minimum!(u128);
impl_validate_numeric_minimum!(usize);
impl_validate_numeric_minimum!(std::num::NonZeroI8);
impl_validate_numeric_minimum!(std::num::NonZeroI16);
impl_validate_numeric_minimum!(std::num::NonZeroI32);
impl_validate_numeric_minimum!(std::num::NonZeroI64);
impl_validate_numeric_minimum!(std::num::NonZeroI128);
impl_validate_numeric_minimum!(std::num::NonZeroIsize);
impl_validate_numeric_minimum!(std::num::NonZeroU8);
impl_validate_numeric_minimum!(std::num::NonZeroU16);
impl_validate_numeric_minimum!(std::num::NonZeroU32);
impl_validate_numeric_minimum!(std::num::NonZeroU64);
impl_validate_numeric_minimum!(std::num::NonZeroU128);
impl_validate_numeric_minimum!(std::num::NonZeroUsize);
impl_validate_numeric_minimum!(f32);
impl_validate_numeric_minimum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_minimum_is_true() {
        assert!(ValidateMinimum::validate_minimum(&10, 9).is_ok());
        assert!(ValidateMinimum::validate_minimum(&10, 10).is_ok());
    }

    #[test]
    fn test_validate_numeric_minimum_is_false() {
        assert!(ValidateMinimum::validate_minimum(&5, 6).is_err());
    }

    #[test]
    fn test_validate_numeric_minimum_specified_type() {
        assert!(ValidateMinimum::validate_minimum(&0.5, 0.2).is_ok());
        assert!(ValidateMinimum::validate_minimum(&5u8, 0).is_ok());
        assert!(ValidateMinimum::validate_minimum(&4u16, 0).is_ok());
        assert!(ValidateMinimum::validate_minimum(&6u32, 0).is_ok());
    }
}
