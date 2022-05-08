/// MultipleOf validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#multiple_of>
pub trait ValidateNumericMultipleOf<T>
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero,
{
    fn check(&self, multiple_of: T) -> bool;
}

macro_rules! impl_validate_numeric_multiple_of {
    ($ty:ty) => {
        impl ValidateNumericMultipleOf<$ty> for $ty {
            fn check(&self, multiple_of: $ty) -> bool {
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

impl<T, U> ValidateNumericMultipleOf<T> for Vec<U>
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero + Copy,
    U: ValidateNumericMultipleOf<T>,
{
    fn check(&self, multiple_of: T) -> bool {
        for item in self {
            if !item.check(multiple_of) {
                return false;
            }
        }

        true
    }
}

impl<T, U, const N: usize> ValidateNumericMultipleOf<T> for [U; N]
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero + Copy,
    U: ValidateNumericMultipleOf<T>,
{
    fn check(&self, multiple_of: T) -> bool {
        for item in self {
            if !item.check(multiple_of) {
                return false;
            }
        }

        true
    }
}

impl<T, U> ValidateNumericMultipleOf<T> for Option<U>
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero,
    U: ValidateNumericMultipleOf<T>,
{
    fn check(&self, multiple_of: T) -> bool {
        if let Some(value) = self {
            value.check(multiple_of)
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_true() {
        assert!(ValidateNumericMultipleOf::check(&10, 5));
    }

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_false() {
        assert!(!ValidateNumericMultipleOf::check(&10, 3));
    }

    #[test]
    fn test_validate_numeric_multiple_of_float_is_true() {
        assert!(ValidateNumericMultipleOf::check(&12.0, 1.0));
        assert!(ValidateNumericMultipleOf::check(&12.5, 0.5));
    }
    #[test]
    fn test_validate_numeric_multiple_of_float_is_false() {
        assert!(!ValidateNumericMultipleOf::check(&12.0, 5.0));
        assert!(!ValidateNumericMultipleOf::check(&12.5, 0.3));
    }

    #[test]
    fn test_validate_numeric_multiple_of_vec_is_true() {
        assert!(ValidateNumericMultipleOf::check(&vec![12.0], 1.0));
    }

    #[test]
    fn test_validate_numeric_multiple_of_vec_is_false() {
        assert!(!ValidateNumericMultipleOf::check(&vec![10], 3));
    }

    #[test]
    fn test_validate_numeric_multiple_of_array_is_true() {
        assert!(ValidateNumericMultipleOf::check(&[12.0], 1.0));
    }

    #[test]
    fn test_validate_numeric_multiple_of_array_is_false() {
        assert!(!ValidateNumericMultipleOf::check(&[10], 3));
    }

    #[test]
    fn test_validate_numeric_multiple_of_option_is_true() {
        assert!(ValidateNumericMultipleOf::check(&Some(12.0), 1.0));
    }

    #[test]
    fn test_validate_numeric_multiple_of_option_is_false() {
        assert!(!ValidateNumericMultipleOf::check(&Some(10), 3));
    }
}
