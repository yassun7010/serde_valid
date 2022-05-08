/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub trait ValidateNumericMinimum<T>
where
    T: PartialOrd + PartialEq,
{
    fn check(&self, minimum: T) -> bool;
}

macro_rules! impl_validate_numeric_minimum {
    ($ty:ty) => {
        impl ValidateNumericMinimum<$ty> for $ty {
            fn check(&self, minimum: $ty) -> bool {
                *self >= minimum
            }
        }
    };
}

impl_validate_numeric_minimum!(i8);
impl_validate_numeric_minimum!(i16);
impl_validate_numeric_minimum!(i32);
impl_validate_numeric_minimum!(i64);
impl_validate_numeric_minimum!(i128);
impl_validate_numeric_minimum!(isize);
impl_validate_numeric_minimum!(u8);
impl_validate_numeric_minimum!(u16);
impl_validate_numeric_minimum!(u32);
impl_validate_numeric_minimum!(u64);
impl_validate_numeric_minimum!(u128);
impl_validate_numeric_minimum!(usize);
impl_validate_numeric_minimum!(f32);
impl_validate_numeric_minimum!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_minimum_is_true() {
        assert!(ValidateNumericMinimum::check(&10, 9));
        assert!(ValidateNumericMinimum::check(&10, 10));
    }

    #[test]
    fn test_validate_numeric_minimum_is_false() {
        assert!(!ValidateNumericMinimum::check(&5, 6));
    }

    #[test]
    fn test_validate_numeric_minimum_vec_is_true() {
        assert!(ValidateNumericMinimum::check(&vec![5], 3));
    }

    #[test]
    fn test_validate_numeric_minimum_vec_is_false() {
        assert!(!ValidateNumericMinimum::check(&vec![5], 7));
    }

    #[test]
    fn test_validate_numeric_minimum_array_is_true() {
        assert!(ValidateNumericMinimum::check(&[5, 6, 7], 3));
    }

    #[test]
    fn test_validate_numeric_minimum_array_is_false() {
        assert!(!ValidateNumericMinimum::check(&vec![5], 7));
    }

    #[test]
    fn test_validate_numeric_minimum_option_is_true() {
        assert!(ValidateNumericMinimum::check(&Some(5), 3));
    }

    #[test]
    fn test_validate_numeric_minimum_option_is_false() {
        assert!(!ValidateNumericMinimum::check(&Some(5), 7));
    }

    #[test]
    fn test_validate_numeric_minimum_specified_type() {
        assert!(ValidateNumericMinimum::check(&0.5, 0.2));
        assert!(ValidateNumericMinimum::check(&5u8, 0));
        assert!(ValidateNumericMinimum::check(&4u16, 0));
        assert!(ValidateNumericMinimum::check(&6u32, 0));
    }
}
