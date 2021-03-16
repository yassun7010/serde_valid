/// EnumeratedValues validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/generic.html#id4>
pub fn validate_generic_enumerated_values<T, U>(value: &T, candidates: &[U]) -> bool
where
    T: std::cmp::PartialEq + std::cmp::PartialEq<U>,
    U: std::cmp::PartialEq<T>,
{
    candidates.iter().any(|v| v == value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_integer_vec_type_is_true() {
        assert!(validate_generic_enumerated_values(&1, &[1, 2, 3]));
    }

    #[test]
    fn test_validate_integer_vec_type_is_false() {
        assert!(!validate_generic_enumerated_values(&1, &[2, 3, 4]));
    }

    #[test]
    fn test_validate_float_type_is_true() {
        assert!(validate_generic_enumerated_values(&0.9, &[0.9, 2.3, -3.0]));
    }

    #[test]
    fn test_validate_float_type_is_false() {
        assert!(!validate_generic_enumerated_values(&0.9, &[0.8, 2.3, -3.0]));
    }

    #[test]
    fn test_validate_str_type() {
        assert!(validate_generic_enumerated_values(&'a', &['a', 'b', 'c']));
    }

    #[test]
    fn test_validate_string_type() {
        assert!(validate_generic_enumerated_values(&"a", &["a", "b", "c"]));
    }

    #[test]
    fn test_validate_vec_type() {
        assert!(validate_generic_enumerated_values(
            &vec!["a"],
            &[vec!["a"], vec!["b"], vec!["c"]]
        ));
    }
}
