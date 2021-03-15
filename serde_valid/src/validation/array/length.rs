/// Length validation.
///
/// See https://json-schema.org/understanding-json-schema/reference/array.html#id7
#[allow(dead_code)]
pub fn validate_array_length<T>(
    value: &[T],
    min_items: Option<usize>,
    max_items: Option<usize>,
) -> bool {
    let len = value.len();
    if let Some(max) = max_items {
        if max < len {
            return false;
        }
    }

    if let Some(min) = min_items {
        if len < min {
            return false;
        };
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_vec_type() {
        assert!(validate_array_length(
            &vec!['a', 'b', 'c'],
            Some(3),
            Some(3)
        ));
    }

    #[test]
    fn test_validate_array_length_array_type() {
        assert!(validate_array_length(&['a', 'b', 'c'], Some(3), Some(3)));
    }

    #[test]
    fn test_validate_array_length_min_is_true() {
        assert!(validate_array_length(&[1, 2, 3], Some(3), None));
    }

    #[test]
    fn test_validate_array_length_min_is_false() {
        assert!(!validate_array_length(&[1, 2, 3], Some(4), None));
    }

    #[test]
    fn test_validate_array_length_max_is_true() {
        assert!(validate_array_length(&[1, 2, 3], None, Some(3)));
    }

    #[test]
    fn test_validate_array_length_max_is_false() {
        assert!(!validate_array_length(&[1, 2, 3], None, Some(2)));
    }
}
