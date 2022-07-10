use crate::MultipleOfErrorParams;

use super::impl_literal_composited_validation;

/// Multipl validation of the number.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#multiples>
pub trait ValidateMultipleOf<T>
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + num_traits::Zero,
{
    fn validate_multiple_of(&self, multiple_of: T) -> Result<(), crate::MultipleOfErrorParams>;
}

macro_rules! impl_validate_numeric_multiple_of {
    ($ty:ty) => {
        impl ValidateMultipleOf<$ty> for $ty {
            fn validate_multiple_of(
                &self,
                multiple_of: $ty,
            ) -> Result<(), crate::MultipleOfErrorParams> {
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

pub trait ValidateCompositedMultipleOf<T> {
    fn validate_composited_multiple_of(
        &self,
        limit: T,
    ) -> Result<(), crate::validation::Composited<MultipleOfErrorParams>>;
}

macro_rules! impl_literal_composited_validate_multiple_of {
    ($type:tt) => {
        impl_literal_composited_validation!(
            ValidateCompositedMultipleOf,
            ValidateMultipleOf,
            MultipleOfErrorParams,
            validate_composited_multiple_of,
            validate_multiple_of,
            $type
        );
    };
}

impl_literal_composited_validate_multiple_of!(i8);
impl_literal_composited_validate_multiple_of!(i16);
impl_literal_composited_validate_multiple_of!(i32);
impl_literal_composited_validate_multiple_of!(i64);
impl_literal_composited_validate_multiple_of!(i128);
impl_literal_composited_validate_multiple_of!(isize);
impl_literal_composited_validate_multiple_of!(u8);
impl_literal_composited_validate_multiple_of!(u16);
impl_literal_composited_validate_multiple_of!(u32);
impl_literal_composited_validate_multiple_of!(u64);
impl_literal_composited_validate_multiple_of!(u128);
impl_literal_composited_validate_multiple_of!(usize);
impl_literal_composited_validate_multiple_of!(f32);
impl_literal_composited_validate_multiple_of!(f64);

impl<T, U> ValidateCompositedMultipleOf<T> for Vec<U>
where
    T: Copy,
    U: ValidateCompositedMultipleOf<T>,
{
    fn validate_composited_multiple_of(
        &self,
        limit: T,
    ) -> Result<(), crate::validation::Composited<MultipleOfErrorParams>> {
        let mut errors = vec![];
        self.iter().for_each(|item| {
            item.validate_composited_multiple_of(limit)
                .map_err(|error| errors.push(error))
                .ok();
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(crate::validation::Composited::Array(errors))
        }
    }
}

impl<T, U, const N: usize> ValidateCompositedMultipleOf<T> for [U; N]
where
    T: Copy,
    U: ValidateCompositedMultipleOf<T>,
{
    fn validate_composited_multiple_of(
        &self,
        limit: T,
    ) -> Result<(), crate::validation::Composited<MultipleOfErrorParams>> {
        let mut errors = vec![];
        self.iter().for_each(|item| {
            item.validate_composited_multiple_of(limit)
                .map_err(|error| errors.push(error))
                .ok();
        });

        if errors.is_empty() {
            Ok(())
        } else {
            Err(crate::validation::Composited::Array(errors))
        }
    }
}

impl<T, U> ValidateCompositedMultipleOf<T> for Option<U>
where
    T: Copy,
    U: ValidateCompositedMultipleOf<T>,
{
    fn validate_composited_multiple_of(
        &self,
        limit: T,
    ) -> Result<(), crate::validation::Composited<MultipleOfErrorParams>> {
        match self {
            Some(value) => value.validate_composited_multiple_of(limit),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_true() {
        assert!(ValidateMultipleOf::validate_multiple_of(&10, 5).is_ok());
    }

    #[test]
    fn test_validate_numeric_multiple_of_integer_is_false() {
        assert!(ValidateMultipleOf::validate_multiple_of(&10, 3).is_err());
    }

    #[test]
    fn test_validate_numeric_multiple_of_float_is_true() {
        assert!(ValidateMultipleOf::validate_multiple_of(&12.0, 1.0).is_ok());
        assert!(ValidateMultipleOf::validate_multiple_of(&12.5, 0.5).is_ok());
    }
    #[test]
    fn test_validate_numeric_multiple_of_float_is_false() {
        assert!(ValidateMultipleOf::validate_multiple_of(&12.0, 5.0).is_err());
        assert!(ValidateMultipleOf::validate_multiple_of(&12.5, 0.3).is_err());
    }
}
