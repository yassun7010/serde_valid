/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub trait ValidateNumericExclusiveMinimum<T>
where
    T: PartialOrd + PartialEq,
{
    fn validate(&self, exclusive_minimum: T) -> bool;
}

macro_rules! impl_validate_numeric_exclusive_minimum {
    ($ty:ty) => {
        impl ValidateNumericExclusiveMinimum<$ty> for $ty {
            fn validate(&self, exclusive_minimum: $ty) -> bool {
                *self > exclusive_minimum
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
impl_validate_numeric_exclusive_minimum!(f32);
impl_validate_numeric_exclusive_minimum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_exclusive_minimum_is_true() {
        // Unspecified generic type:
        assert!(ValidateNumericExclusiveMinimum::validate(&10, 9));
    }

    #[test]
    fn test_validate_numeric_exclusive_minimum_is_false() {
        assert!(!ValidateNumericExclusiveMinimum::validate(&5, 6));
        assert!(!ValidateNumericExclusiveMinimum::validate(&5, 5));
    }

    #[test]
    fn test_validate_numeric_exclusive_minimum_specified_type() {
        assert!(ValidateNumericExclusiveMinimum::validate(&0.5, 0.2));
        assert!(ValidateNumericExclusiveMinimum::validate(&5u8, 0));
        assert!(ValidateNumericExclusiveMinimum::validate(&4u16, 0));
        assert!(ValidateNumericExclusiveMinimum::validate(&6u32, 0));
    }
}
