use crate::validation::{
    impl_generic_composited_validation_1args, ValidateCompositedExclusiveMaximum,
};
use crate::ExclusiveMaximumErrorParams;

/// Exclusive maximum validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateExclusiveMaximum};
/// struct MyType(i32);
///
/// impl ValidateExclusiveMaximum<i32> for MyType {
///     fn validate_exclusive_maximum(&self, exclusive_maximum: i32) -> Result<(), serde_valid::ExclusiveMaximumErrorParams> {
///         self.0.validate_exclusive_maximum(exclusive_maximum)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(exclusive_maximum = 5)]
///     val: MyType,
/// }
///
/// let s = TestStruct { val: MyType(5) };
///
/// assert_eq!(
///     s.validate().unwrap_err().to_string(),
///     json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The number must be `< 5`."]
///             }
///         }
///     })
///     .to_string()
/// );
/// ```
pub trait ValidateExclusiveMaximum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_exclusive_maximum(
        &self,
        exclusive_maximum: T,
    ) -> Result<(), ExclusiveMaximumErrorParams>;
}

macro_rules! impl_validate_numeric_exclusive_maximum {
    ($type:ty) => {
        impl ValidateExclusiveMaximum<$type> for $type {
            fn validate_exclusive_maximum(
                &self,
                exclusive_maximum: $type,
            ) -> Result<(), crate::ExclusiveMaximumErrorParams> {
                if *self < exclusive_maximum {
                    Ok(())
                } else {
                    Err(crate::ExclusiveMaximumErrorParams::new(exclusive_maximum))
                }
            }
        }

        impl_generic_composited_validation_1args!(ExclusiveMaximum, $type);
    };
}

impl_validate_numeric_exclusive_maximum!(i8);
impl_validate_numeric_exclusive_maximum!(i16);
impl_validate_numeric_exclusive_maximum!(i32);
impl_validate_numeric_exclusive_maximum!(i64);
impl_validate_numeric_exclusive_maximum!(i128);
impl_validate_numeric_exclusive_maximum!(isize);
impl_validate_numeric_exclusive_maximum!(u8);
impl_validate_numeric_exclusive_maximum!(u16);
impl_validate_numeric_exclusive_maximum!(u32);
impl_validate_numeric_exclusive_maximum!(u64);
impl_validate_numeric_exclusive_maximum!(u128);
impl_validate_numeric_exclusive_maximum!(usize);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI8);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI16);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI32);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI64);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroI128);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroIsize);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU8);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU16);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU32);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU64);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroU128);
impl_validate_numeric_exclusive_maximum!(std::num::NonZeroUsize);
impl_validate_numeric_exclusive_maximum!(f32);
impl_validate_numeric_exclusive_maximum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_exclusive_maximum_is_true() {
        // Unspecified generic type:
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 11).is_ok());
    }

    #[test]
    fn test_validate_numeric_exclusive_maximum_is_false() {
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&5, 4).is_err());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&10, 10).is_err());
    }

    #[test]
    fn test_validate_numeric_exclusive_maximum_specified_type() {
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&0.2, 0.5).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&0, 5u8).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&0, 4u16).is_ok());
        assert!(ValidateExclusiveMaximum::validate_exclusive_maximum(&0, 6u32).is_ok());
    }
}
