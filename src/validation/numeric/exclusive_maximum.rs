/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub trait ValidateNumericExclusiveMaximum<T>
where
    T: PartialOrd + PartialEq,
{
    fn check(&self, exclusive_maximum: T) -> bool;
}

macro_rules! impl_validate_numeric_exclusive_maximum {
    ($ty:ty) => {
        impl ValidateNumericExclusiveMaximum<$ty> for $ty {
            fn check(&self, exclusive_maximum: $ty) -> bool {
                *self < exclusive_maximum
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
        assert!(ValidateNumericExclusiveMaximum::check(&10, 11));
    }

    #[test]
    fn test_validate_numeric_exclusive_maximum_is_false() {
        assert!(!ValidateNumericExclusiveMaximum::check(&5, 4));
        assert!(!ValidateNumericExclusiveMaximum::check(&10, 10));
    }

    #[test]
    fn test_validate_numeric_exclusive_maximum_specified_type() {
        assert!(ValidateNumericExclusiveMaximum::check(&0.2, 0.5));
        assert!(ValidateNumericExclusiveMaximum::check(&0, 5u8));
        assert!(ValidateNumericExclusiveMaximum::check(&0, 4u16));
        assert!(ValidateNumericExclusiveMaximum::check(&0, 6u32));
    }
}
