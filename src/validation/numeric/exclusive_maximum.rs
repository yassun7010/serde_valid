/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub trait ValidateExclusiveMaximum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_exclusive_maximum(
        &self,
        exclusive_maximum: T,
    ) -> Result<(), crate::ExclusiveMaximumErrorParams>;
}

macro_rules! impl_validate_numeric_exclusive_maximum {
    ($ty:ty) => {
        impl ValidateExclusiveMaximum<$ty> for $ty {
            fn validate_exclusive_maximum(
                &self,
                exclusive_maximum: $ty,
            ) -> Result<(), crate::ExclusiveMaximumErrorParams> {
                if *self < exclusive_maximum {
                    Ok(())
                } else {
                    Err(crate::ExclusiveMaximumErrorParams::new(exclusive_maximum))
                }
            }
        }
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
