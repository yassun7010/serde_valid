use crate::ExclusiveMinimumErrorParams;

/// Exclusive minimum validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub trait ValidateExclusiveMinimum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate_exclusive_minimum(
        &self,
        exclusive_minimum: T,
    ) -> Result<(), ExclusiveMinimumErrorParams>;
}

macro_rules! impl_validate_numeric_exclusive_minimum {
    ($ty:ty) => {
        impl ValidateExclusiveMinimum<$ty> for $ty {
            fn validate_exclusive_minimum(
                &self,
                exclusive_minimum: $ty,
            ) -> Result<(), ExclusiveMinimumErrorParams> {
                if *self > exclusive_minimum {
                    Ok(())
                } else {
                    Err(ExclusiveMinimumErrorParams::new(exclusive_minimum))
                }
            }
        }
    };
}

impl_validate_numeric_exclusive_minimum!(i8);
impl_validate_numeric_exclusive_minimum!(i16);
impl_validate_numeric_exclusive_minimum!(i32);
impl_validate_numeric_exclusive_minimum!(i64);
impl_validate_numeric_exclusive_minimum!(i128);
impl_validate_numeric_exclusive_minimum!(isize);
impl_validate_numeric_exclusive_minimum!(u8);
impl_validate_numeric_exclusive_minimum!(u16);
impl_validate_numeric_exclusive_minimum!(u32);
impl_validate_numeric_exclusive_minimum!(u64);
impl_validate_numeric_exclusive_minimum!(u128);
impl_validate_numeric_exclusive_minimum!(usize);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI8);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI16);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI32);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI64);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroI128);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroIsize);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU8);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU16);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU32);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU64);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroU128);
impl_validate_numeric_exclusive_minimum!(std::num::NonZeroUsize);
impl_validate_numeric_exclusive_minimum!(f32);
impl_validate_numeric_exclusive_minimum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_exclusive_minimum_is_true() {
        // Unspecified generic type:
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&10, 9).is_ok());
    }

    #[test]
    fn test_validate_numeric_exclusive_minimum_is_false() {
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&5, 6).is_err());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&5, 5).is_err());
    }

    #[test]
    fn test_validate_numeric_exclusive_minimum_specified_type() {
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&0.5, 0.2).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&5u8, 0).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&4u16, 0).is_ok());
        assert!(ValidateExclusiveMinimum::validate_exclusive_minimum(&6u32, 0).is_ok());
    }
}
