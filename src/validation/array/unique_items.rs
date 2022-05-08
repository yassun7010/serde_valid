use crate::traits::IsUnique;

/// Unique validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#unique_items>
pub trait ValidateArrayUniqueItems {
    fn check(&self) -> bool;
}

impl<T> ValidateArrayUniqueItems for Vec<T>
where
    T: std::cmp::Eq + std::hash::Hash,
{
    fn check(&self) -> bool {
        self.is_unique()
    }
}

impl<T, const N: usize> ValidateArrayUniqueItems for [T; N]
where
    T: std::cmp::Eq + std::hash::Hash,
{
    fn check(&self) -> bool {
        self.is_unique()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_unique_items_array_type_is_true() {
        assert!(ValidateArrayUniqueItems::check(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_validate_array_unique_items_vec_type_is_true() {
        assert!(ValidateArrayUniqueItems::check(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_validate_array_unique_items_is_false() {
        assert!(!ValidateArrayUniqueItems::check(&[1, 2, 3, 3]));
    }
}
