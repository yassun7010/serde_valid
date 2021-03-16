use crate::traits::IsUnique;
/// Unique validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#uniqueness>
pub fn validate_array_uniqueness<T>(value: &[T]) -> bool
where
    T: std::cmp::Eq + std::hash::Hash,
{
    value.is_unique()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_uniqueness_array_type_is_true() {
        assert!(validate_array_uniqueness(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_validate_array_uniqueness_vec_type_is_true() {
        assert!(validate_array_uniqueness(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_validate_array_uniqueness_is_false() {
        assert!(!validate_array_uniqueness(&[1, 2, 3, 3]));
    }
}
