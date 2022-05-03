/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
pub fn validate_array_min_items<T>(value: &[T], min_items: usize) -> bool {
    value.len() >= min_items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_vec_type() {
        assert!(validate_array_min_items(&vec!['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_array_min_items_array_type() {
        assert!(validate_array_min_items(&['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_array_min_items_is_true() {
        assert!(validate_array_min_items(&[1, 2, 3], 3));
    }

    #[test]
    fn test_validate_array_min_items_is_false() {
        assert!(!validate_array_min_items(&[1, 2, 3], 4));
    }
}
