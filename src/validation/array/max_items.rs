/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
pub fn validate_array_max_items<T>(value: &[T], max_items: usize) -> bool {
    max_items >= value.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_vec_type() {
        assert!(validate_array_max_items(&vec!['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_array_max_items_array_type() {
        assert!(validate_array_max_items(&['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_array_max_items_is_true() {
        assert!(validate_array_max_items(&[1, 2, 3], 3));
    }

    #[test]
    fn test_validate_array_max_items_is_false() {
        assert!(!validate_array_max_items(&[1, 2, 3], 2));
    }
}
