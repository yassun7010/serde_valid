/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub trait ValidateNumericMaximum<T>
where
    T: PartialOrd + PartialEq,
{
    fn check(&self, maximum: T) -> bool;
}

macro_rules! impl_validate_numeric_maximum {
    ($ty:ty) => {
        impl ValidateNumericMaximum<$ty> for $ty {
            fn check(&self, maximum: $ty) -> bool {
                *self <= maximum
            }
        }
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
impl_validate_numeric_maximum!(f32);
impl_validate_numeric_maximum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_maximum_is_true() {
        // Unspecified generic type:
        assert!(ValidateNumericMaximum::check(&10, 11));
        assert!(ValidateNumericMaximum::check(&10, 10));
    }

    #[test]
    fn test_validate_numeric_maximum_is_false() {
        assert!(!ValidateNumericMaximum::check(&5, 4));
    }

    #[test]
    fn test_validate_numeric_maximum_specified_type() {
        assert!(ValidateNumericMaximum::check(&0.2, 0.5));
        assert!(ValidateNumericMaximum::check(&0, 5u8));
        assert!(ValidateNumericMaximum::check(&0, 4u16));
        assert!(ValidateNumericMaximum::check(&0, 6u32));
    }
}
