/// Enumerate validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/generic.html#enumerated-values>
pub trait ValidateEnumerate<T> {
    fn validate_enumerate(&self, enumerate: &[T]) -> Result<(), crate::EnumerateErrorParams>;
}

macro_rules! impl_validate_generic_enumerate_literal {
    ($ty:ty) => {
        impl ValidateEnumerate<$ty> for $ty {
            fn validate_enumerate(
                &self,
                enumerate: &[$ty],
            ) -> Result<(), crate::EnumerateErrorParams> {
                if enumerate.iter().any(|candidate| candidate == self) {
                    Ok(())
                } else {
                    Err(crate::EnumerateErrorParams::new(enumerate))
                }
            }
        }
    };
}

impl_validate_generic_enumerate_literal!(i8);
impl_validate_generic_enumerate_literal!(u8);
impl_validate_generic_enumerate_literal!(i16);
impl_validate_generic_enumerate_literal!(u16);
impl_validate_generic_enumerate_literal!(i32);
impl_validate_generic_enumerate_literal!(u32);
impl_validate_generic_enumerate_literal!(i64);
impl_validate_generic_enumerate_literal!(u64);
impl_validate_generic_enumerate_literal!(i128);
impl_validate_generic_enumerate_literal!(u128);
impl_validate_generic_enumerate_literal!(isize);
impl_validate_generic_enumerate_literal!(usize);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU32);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI16);
impl_validate_generic_enumerate_literal!(f32);
impl_validate_generic_enumerate_literal!(f64);
impl_validate_generic_enumerate_literal!(char);

macro_rules! impl_validate_generic_enumerate_str {
    ($ty:ty) => {
        impl ValidateEnumerate<&str> for $ty {
            fn validate_enumerate(
                &self,
                enumerate: &[&str],
            ) -> Result<(), crate::EnumerateErrorParams> {
                if enumerate.iter().any(|candidate| candidate == self) {
                    Ok(())
                } else {
                    Err(crate::EnumerateErrorParams::new(enumerate))
                }
            }
        }
    };
}

impl_validate_generic_enumerate_str!(&str);
impl_validate_generic_enumerate_str!(String);
impl_validate_generic_enumerate_str!(std::borrow::Cow<'_, str>);
impl_validate_generic_enumerate_str!(&std::ffi::OsStr);
impl_validate_generic_enumerate_str!(std::ffi::OsString);

macro_rules! impl_validate_generic_enumerate_path {
    ($ty:ty) => {
        impl ValidateEnumerate<&str> for $ty {
            fn validate_enumerate(
                &self,
                enumerate: &[&str],
            ) -> Result<(), crate::EnumerateErrorParams> {
                if enumerate
                    .iter()
                    .any(|candidate| &std::path::Path::new(candidate) == self)
                {
                    Ok(())
                } else {
                    Err(crate::EnumerateErrorParams::new(enumerate))
                }
            }
        }
    };
}

impl_validate_generic_enumerate_path!(&std::path::Path);
impl_validate_generic_enumerate_path!(std::path::PathBuf);

impl<T, U> ValidateEnumerate<U> for Vec<T>
where
    T: ValidateEnumerate<U>,
{
    fn validate_enumerate(&self, enumerate: &[U]) -> Result<(), crate::EnumerateErrorParams> {
        for item in self {
            item.validate_enumerate(enumerate)?
        }

        Ok(())
    }
}

impl<T, U, const N: usize> ValidateEnumerate<U> for [T; N]
where
    T: ValidateEnumerate<U>,
{
    fn validate_enumerate(&self, enumerate: &[U]) -> Result<(), crate::EnumerateErrorParams> {
        for item in self {
            item.validate_enumerate(enumerate)?
        }

        Ok(())
    }
}

impl<T, U> ValidateEnumerate<U> for Option<T>
where
    T: ValidateEnumerate<U>,
    U: std::cmp::PartialEq<U>,
{
    fn validate_enumerate(&self, enumerate: &[U]) -> Result<(), crate::EnumerateErrorParams> {
        match self {
            Some(value) => value.validate_enumerate(enumerate),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_integer_vec_type_is_true() {
        assert!(ValidateEnumerate::validate_enumerate(&1, &[1, 2, 3]).is_ok());
    }

    #[test]
    fn test_validate_integer_vec_type_is_false() {
        assert!(ValidateEnumerate::validate_enumerate(&1, &[2, 3, 4]).is_err());
    }

    #[test]
    fn test_validate_float_type_is_true() {
        assert!(ValidateEnumerate::validate_enumerate(&0.9, &[0.9, 2.3, -3.0]).is_ok());
    }

    #[test]
    fn test_validate_float_type_is_false() {
        assert!(ValidateEnumerate::validate_enumerate(&0.9, &[0.8, 2.3, -3.0]).is_err());
    }

    #[test]
    fn test_validate_unsigned_int_type() {
        assert!(ValidateEnumerate::validate_enumerate(&1, &[-1, 0, 1, 2, 3]).is_ok());
    }

    #[test]
    fn test_validate_str_type() {
        assert!(ValidateEnumerate::validate_enumerate(&'a', &['a', 'b', 'c']).is_ok());
    }

    #[test]
    fn test_validate_string_type() {
        assert!(ValidateEnumerate::validate_enumerate(&'a', &['a', 'b', 'c']).is_ok());
    }

    #[test]
    fn test_validate_os_str_type() {
        assert!(ValidateEnumerate::validate_enumerate(
            &std::ffi::OsStr::new("a"),
            &["a", "b", "c"]
        )
        .is_ok());
    }

    #[test]
    fn test_validate_os_string_type() {
        assert!(ValidateEnumerate::validate_enumerate(
            &std::ffi::OsString::from("a"),
            &["a", "b", "c"]
        )
        .is_ok());
    }

    #[test]
    fn test_validate_path_type() {
        assert!(ValidateEnumerate::validate_enumerate(
            &std::path::Path::new("a"),
            &["a", "b", "c"]
        )
        .is_ok());
    }

    #[test]
    fn test_validate_path_buf_type() {
        assert!(ValidateEnumerate::validate_enumerate(
            &std::path::PathBuf::from("a"),
            &["a", "b", "c"]
        )
        .is_ok());
    }

    #[test]
    fn test_validate_vec_type() {
        assert!(ValidateEnumerate::validate_enumerate(&vec!["a"], &["a", "b", "c"]).is_ok());
    }
}
