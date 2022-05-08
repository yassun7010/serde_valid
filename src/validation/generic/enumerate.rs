/// EnumeratedValues validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/generic.html#enumerated-values>
pub trait ValidateEnumerate<T>
where
    Self: std::cmp::PartialEq<T>,
{
    fn validate(&self, enumerate: &[T]) -> Result<(), crate::EnumerateErrorParams>;
}

impl<T, U> ValidateEnumerate<U> for T
where
    T: std::cmp::PartialEq<U>,
    U: std::cmp::PartialEq<T> + std::fmt::Debug,
{
    fn validate(&self, enumerate: &[U]) -> Result<(), crate::EnumerateErrorParams> {
        if enumerate.iter().any(|candidate| candidate == self) {
            Ok(())
        } else {
            Err(crate::EnumerateErrorParams::new(enumerate))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_integer_vec_type_is_true() {
        assert!(ValidateEnumerate::validate(&1, &[1, 2, 3]).is_ok());
    }

    #[test]
    fn test_validate_integer_vec_type_is_false() {
        assert!(ValidateEnumerate::validate(&1, &[2, 3, 4]).is_err());
    }

    #[test]
    fn test_validate_float_type_is_true() {
        assert!(ValidateEnumerate::validate(&0.9, &[0.9, 2.3, -3.0]).is_ok());
    }

    #[test]
    fn test_validate_float_type_is_false() {
        assert!(ValidateEnumerate::validate(&0.9, &[0.8, 2.3, -3.0]).is_err());
    }

    #[test]
    fn test_validate_str_type() {
        assert!(ValidateEnumerate::validate(&'a', &['a', 'b', 'c']).is_ok());
    }

    #[test]
    fn test_validate_string_type() {
        assert!(ValidateEnumerate::validate(&"a", &["a", "b", "c"]).is_ok());
    }

    #[test]
    fn test_validate_vec_type() {
        assert!(
            ValidateEnumerate::validate(&vec!["a"], &[vec!["a"], vec!["b"], vec!["c"]]).is_ok()
        );
    }
}
