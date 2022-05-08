/// EnumeratedValues validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/generic.html#enumerated-values>
pub trait ValidateGenericEnumerate<T>
where
    Self: std::cmp::PartialEq<T>,
{
    fn check(&self, enumerate: &[T]) -> bool;
}

impl<T, U> ValidateGenericEnumerate<U> for T
where
    T: std::cmp::PartialEq<U>,
    U: std::cmp::PartialEq<T>,
{
    fn check(&self, enumerate: &[U]) -> bool {
        enumerate.iter().any(|candidate| candidate == self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_integer_vec_type_is_true() {
        assert!(ValidateGenericEnumerate::check(&1, &[1, 2, 3]));
    }

    #[test]
    fn test_validate_integer_vec_type_is_false() {
        assert!(!ValidateGenericEnumerate::check(&1, &[2, 3, 4]));
    }

    #[test]
    fn test_validate_float_type_is_true() {
        assert!(ValidateGenericEnumerate::check(&0.9, &[0.9, 2.3, -3.0]));
    }

    #[test]
    fn test_validate_float_type_is_false() {
        assert!(!ValidateGenericEnumerate::check(&0.9, &[0.8, 2.3, -3.0]));
    }

    #[test]
    fn test_validate_str_type() {
        assert!(ValidateGenericEnumerate::check(&'a', &['a', 'b', 'c']));
    }

    #[test]
    fn test_validate_string_type() {
        assert!(ValidateGenericEnumerate::check(&"a", &["a", "b", "c"]));
    }

    #[test]
    fn test_validate_vec_type() {
        assert!(ValidateGenericEnumerate::check(
            &vec!["a"],
            &[vec!["a"], vec!["b"], vec!["c"]]
        ));
    }
}
