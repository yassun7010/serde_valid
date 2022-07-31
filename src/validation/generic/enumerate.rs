use crate::validation::ValidateCompositedEnumerate;
use crate::EnumerateErrorParams;

/// Enumerate validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/generic.html#enumerated-values>
///
/// ```rust
/// use serde_json::json;
/// use serde_valid::{Validate, ValidateEnumerate};
///
/// struct MyType(String);
///
/// impl ValidateEnumerate<&'static str> for MyType {
///     fn validate_enumerate(
///         &self,
///         enumerate: &[&'static str],
///     ) -> Result<(), serde_valid::EnumerateErrorParams> {
///         self.0.validate_enumerate(enumerate)
///     }
/// }
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(enumerate("1", "2", "3"))]
///     val: MyType,
/// }
///
/// let s = TestStruct {
///     val: MyType("4".to_string()),
/// };
///
/// assert_eq!(
///     serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
///     serde_json::to_string(&json!({
///         "errors": [],
///         "properties": {
///             "val": {
///                 "errors": ["The value must be in [1, 2, 3]."]
///             }
///         }
///     }))
///     .unwrap()
/// );
/// ```
pub trait ValidateEnumerate<T> {
    fn validate_enumerate(&self, enumerate: &[T]) -> Result<(), EnumerateErrorParams>;
}

macro_rules! impl_validate_generic_enumerate_literal {
    ($type:ty) => {
        impl ValidateEnumerate<$type> for $type {
            fn validate_enumerate(&self, enumerate: &[$type]) -> Result<(), EnumerateErrorParams> {
                if enumerate.iter().any(|candidate| candidate == self) {
                    Ok(())
                } else {
                    Err(EnumerateErrorParams::new(enumerate))
                }
            }
        }

        impl<T> ValidateCompositedEnumerate<&[$type]> for T
        where
            T: ValidateEnumerate<$type>,
        {
            fn validate_composited_enumerate(
                &self,
                limit: &[$type],
            ) -> Result<(), crate::validation::Composited<EnumerateErrorParams>> {
                self.validate_enumerate(limit)
                    .map_err(|error| crate::validation::Composited::Single(error))
            }
        }
    };
}

impl_validate_generic_enumerate_literal!(i8);
impl_validate_generic_enumerate_literal!(i16);
impl_validate_generic_enumerate_literal!(i32);
impl_validate_generic_enumerate_literal!(i64);
impl_validate_generic_enumerate_literal!(i128);
impl_validate_generic_enumerate_literal!(isize);
impl_validate_generic_enumerate_literal!(u8);
impl_validate_generic_enumerate_literal!(u16);
impl_validate_generic_enumerate_literal!(u32);
impl_validate_generic_enumerate_literal!(u64);
impl_validate_generic_enumerate_literal!(u128);
impl_validate_generic_enumerate_literal!(usize);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI8);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI16);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI32);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI64);
impl_validate_generic_enumerate_literal!(std::num::NonZeroI128);
impl_validate_generic_enumerate_literal!(std::num::NonZeroIsize);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU8);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU16);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU32);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU64);
impl_validate_generic_enumerate_literal!(std::num::NonZeroU128);
impl_validate_generic_enumerate_literal!(std::num::NonZeroUsize);
impl_validate_generic_enumerate_literal!(f32);
impl_validate_generic_enumerate_literal!(f64);
impl_validate_generic_enumerate_literal!(char);

macro_rules! impl_validate_generic_enumerate_str {
    ($type:ty) => {
        impl ValidateEnumerate<&'static str> for $type {
            fn validate_enumerate(
                &self,
                enumerate: &[&'static str],
            ) -> Result<(), EnumerateErrorParams> {
                if enumerate.iter().any(|candidate| candidate == self) {
                    Ok(())
                } else {
                    Err(EnumerateErrorParams::new(enumerate))
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
    ($type:ty) => {
        impl ValidateEnumerate<&'static str> for $type {
            fn validate_enumerate(
                &self,
                enumerate: &[&'static str],
            ) -> Result<(), EnumerateErrorParams> {
                if enumerate
                    .iter()
                    .any(|candidate| &std::path::Path::new(candidate) == self)
                {
                    Ok(())
                } else {
                    Err(EnumerateErrorParams::new(enumerate))
                }
            }
        }
    };
}

impl_validate_generic_enumerate_path!(&std::path::Path);
impl_validate_generic_enumerate_path!(std::path::PathBuf);

impl<T> ValidateCompositedEnumerate<&[&'static str]> for T
where
    T: ValidateEnumerate<&'static str>,
{
    fn validate_composited_enumerate(
        &self,
        limit: &[&'static str],
    ) -> Result<(), crate::validation::Composited<EnumerateErrorParams>> {
        self.validate_enumerate(limit)
            .map_err(|error| crate::validation::Composited::Single(error))
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
}
