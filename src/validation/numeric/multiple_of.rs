/// MultipleOf validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#multiple_of>
pub trait ValidateNumericMultipleOf<T>
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero,
{
    fn validate(&self, multiple_of: T) -> bool;
}

macro_rules! impl_validate_numeric_multiple_of {
    ($ty:ty) => {
        impl ValidateNumericMultipleOf<$ty> for $ty {
            fn validate(&self, multiple_of: $ty) -> bool {
                std::cmp::PartialEq::<$ty>::eq(&(*self % multiple_of), &num_traits::Zero::zero())
            }
        }
    };
}

impl_validate_numeric_multiple_of!(i8);
impl_validate_numeric_multiple_of!(i16);
impl_validate_numeric_multiple_of!(i32);
impl_validate_numeric_multiple_of!(i64);
impl_validate_numeric_multiple_of!(i128);
impl_validate_numeric_multiple_of!(isize);
impl_validate_numeric_multiple_of!(u8);
impl_validate_numeric_multiple_of!(u16);
impl_validate_numeric_multiple_of!(u32);
impl_validate_numeric_multiple_of!(u64);
impl_validate_numeric_multiple_of!(u128);
impl_validate_numeric_multiple_of!(usize);
impl_validate_numeric_multiple_of!(f32);
impl_validate_numeric_multiple_of!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_true() {
        assert!(ValidateNumericMultipleOf::validate(&10, 5));
    }

    #[test]
    fn test_validate_numeric_multiple_of_float_is_true() {
        assert!(ValidateNumericMultipleOf::validate(&12.0, 1.0));
        assert!(ValidateNumericMultipleOf::validate(&12.5, 0.5));
    }

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_false() {
        assert!(!ValidateNumericMultipleOf::validate(&10, 3));
    }

    #[test]
    fn test_validate_numeric_multiple_of_float_is_false() {
        assert!(!ValidateNumericMultipleOf::validate(&12.0, 5.0));
        assert!(!ValidateNumericMultipleOf::validate(&12.5, 0.3));
    }
}
