use crate::validation::{impl_generic_composited_validation_1args, ValidateCompositedMaximum};
use crate::MaximumErrorParams;

/// Maximum validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateMaximum};
/// struct MyType(i32);
///
/// impl ValidateMaximum<i32> for MyType {
///     fn validate_maximum(&self, maximum: i32) -> Result<(), serde_valid::MaximumErrorParams> {
///         self.0.validate_maximum(maximum)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(maximum = 5)]
///     val: MyType,
/// }
///
/// let s = TestStruct { val: MyType(6) };
///
/// assert_eq!(
///     serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
///     serde_json::to_string(&json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The number must be `<= 5`."]
///             }
///         }
///     }))
///     .unwrap()
/// );
/// ```
pub trait ValidateMaximum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_maximum(&self, maximum: T) -> Result<(), MaximumErrorParams>;
}

macro_rules! impl_validate_numeric_maximum {
    ($type:ty) => {
        impl ValidateMaximum<$type> for $type {
            fn validate_maximum(&self, maximum: $type) -> Result<(), MaximumErrorParams> {
                if *self <= maximum {
                    Ok(())
                } else {
                    Err(MaximumErrorParams::new(maximum))
                }
            }
        }

        impl_generic_composited_validation_1args!(Maximum, $type);
    };
}

impl_validate_numeric_maximum!(i8);
impl_validate_numeric_maximum!(i16);
impl_validate_numeric_maximum!(i32);
impl_validate_numeric_maximum!(i64);
impl_validate_numeric_maximum!(i128);
impl_validate_numeric_maximum!(isize);
impl_validate_numeric_maximum!(u8);
impl_validate_numeric_maximum!(u16);
impl_validate_numeric_maximum!(u32);
impl_validate_numeric_maximum!(u64);
impl_validate_numeric_maximum!(u128);
impl_validate_numeric_maximum!(usize);
impl_validate_numeric_maximum!(std::num::NonZeroI8);
impl_validate_numeric_maximum!(std::num::NonZeroI16);
impl_validate_numeric_maximum!(std::num::NonZeroI32);
impl_validate_numeric_maximum!(std::num::NonZeroI64);
impl_validate_numeric_maximum!(std::num::NonZeroI128);
impl_validate_numeric_maximum!(std::num::NonZeroIsize);
impl_validate_numeric_maximum!(std::num::NonZeroU8);
impl_validate_numeric_maximum!(std::num::NonZeroU16);
impl_validate_numeric_maximum!(std::num::NonZeroU32);
impl_validate_numeric_maximum!(std::num::NonZeroU64);
impl_validate_numeric_maximum!(std::num::NonZeroU128);
impl_validate_numeric_maximum!(std::num::NonZeroUsize);
impl_validate_numeric_maximum!(f32);
impl_validate_numeric_maximum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_maximum_is_true() {
        // Unspecified generic type:
        assert!(ValidateMaximum::validate_maximum(&10, 11).is_ok());
        assert!(ValidateMaximum::validate_maximum(&10, 10).is_ok());
    }

    #[test]
    fn test_validate_numeric_maximum_is_false() {
        assert!(ValidateMaximum::validate_maximum(&5, 4).is_err());
    }

    #[test]
    fn test_validate_numeric_maximum_specified_type() {
        assert!(ValidateMaximum::validate_maximum(&0.2, 0.5).is_ok());
        assert!(ValidateMaximum::validate_maximum(&0, 5u8).is_ok());
        assert!(ValidateMaximum::validate_maximum(&0, 4u16).is_ok());
        assert!(ValidateMaximum::validate_maximum(&0, 6u32).is_ok());
    }
}
