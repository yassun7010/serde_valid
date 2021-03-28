use crate::traits::IsUnique;

/// Unique validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#unique_items>
pub fn validate_array_unique_items<T>(value: &[T]) -> bool
where
    T: std::cmp::Eq + std::hash::Hash,
{
    value.is_unique()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_unique_items_array_type_is_true() {
        assert!(validate_array_unique_items(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_validate_array_unique_items_vec_type_is_true() {
        assert!(validate_array_unique_items(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_validate_array_unique_items_is_false() {
        assert!(!validate_array_unique_items(&[1, 2, 3, 3]));
    }
}
