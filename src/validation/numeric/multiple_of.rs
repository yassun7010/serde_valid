/// MultipleOf validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#multiples>
pub trait ValidateNumericMultipleOf<T>
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero,
{
    fn validate(&self, multiple_of: T) -> Result<(), crate::MultipleOfErrorParams>;
}

macro_rules! impl_validate_numeric_multiple_of {
    ($ty:ty) => {
        impl ValidateNumericMultipleOf<$ty> for $ty {
            fn validate(&self, multiple_of: $ty) -> Result<(), crate::MultipleOfErrorParams> {
                if std::cmp::PartialEq::<$ty>::eq(&(*self % multiple_of), &num_traits::Zero::zero())
                {
                    Ok(())
                } else {
                    Err(crate::MultipleOfErrorParams::new(multiple_of))
                }
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
    fn validate(&self, multiple_of: T) -> Result<(), crate::MultipleOfErrorParams> {
        for item in self {
            item.validate(multiple_of)?
        }

        Ok(())
    }
}

impl<T, U, const N: usize> ValidateNumericMultipleOf<T> for [U; N]
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero + Copy,
    U: ValidateNumericMultipleOf<T>,
{
    fn validate(&self, multiple_of: T) -> Result<(), crate::MultipleOfErrorParams> {
        for item in self {
            item.validate(multiple_of)?
        }

        Ok(())
    }
}

impl<T, U> ValidateNumericMultipleOf<T> for Option<U>
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero,
    U: ValidateNumericMultipleOf<T>,
{
    fn validate(&self, multiple_of: T) -> Result<(), crate::MultipleOfErrorParams> {
        if let Some(value) = self {
            value.validate(multiple_of)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_true() {
        assert!(ValidateNumericMultipleOf::validate(&10, 5).is_ok());
    }

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_false() {
        assert!(ValidateNumericMultipleOf::validate(&10, 3).is_err());
    }

    #[test]
    fn test_validate_numeric_multiple_of_float_is_true() {
        assert!(ValidateNumericMultipleOf::validate(&12.0, 1.0).is_ok());
        assert!(ValidateNumericMultipleOf::validate(&12.5, 0.5).is_ok());
    }
    #[test]
    fn test_validate_numeric_multiple_of_float_is_false() {
        assert!(ValidateNumericMultipleOf::validate(&12.0, 5.0).is_err());
        assert!(ValidateNumericMultipleOf::validate(&12.5, 0.3).is_err());
    }

    #[test]
    fn test_validate_numeric_multiple_of_vec_is_true() {
        assert!(ValidateNumericMultipleOf::validate(&vec![12.0], 1.0).is_ok());
    }

    #[test]
    fn test_validate_numeric_multiple_of_vec_is_false() {
        assert!(ValidateNumericMultipleOf::validate(&vec![10], 3).is_err());
    }

    #[test]
    fn test_validate_numeric_multiple_of_array_is_true() {
        assert!(ValidateNumericMultipleOf::validate(&[12.0], 1.0).is_ok());
    }

    #[test]
    fn test_validate_numeric_multiple_of_array_is_false() {
        assert!(ValidateNumericMultipleOf::validate(&[10], 3).is_err());
    }

    #[test]
    fn test_validate_numeric_multiple_of_option_is_true() {
        assert!(ValidateNumericMultipleOf::validate(&Some(12.0), 1.0).is_ok());
    }

    #[test]
    fn test_validate_numeric_multiple_of_none_is_true() {
        assert!(ValidateNumericMultipleOf::validate(&Option::<f32>::None, 1.0).is_ok());
    }

    #[test]
    fn test_validate_numeric_multiple_of_option_is_false() {
        assert!(ValidateNumericMultipleOf::validate(&Some(10), 3).is_err());
    }
}
