/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
pub trait ValidateArrayMaxItems {
    fn check(&self, max_items: usize) -> bool;
}

impl<T> ValidateArrayMaxItems for Vec<T> {
    fn check(&self, max_items: usize) -> bool {
        max_items >= self.len()
    }
}

impl<T, const N: usize> ValidateArrayMaxItems for [T; N] {
    fn check(&self, max_items: usize) -> bool {
        max_items >= self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_vec_type() {
        assert!(ValidateArrayMaxItems::check(&vec!['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_array_max_items_array_type() {
        assert!(ValidateArrayMaxItems::check(&['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_array_max_items_is_true() {
        assert!(ValidateArrayMaxItems::check(&[1, 2, 3], 3));
    }

    #[test]
    fn test_validate_array_max_items_is_false() {
        assert!(!ValidateArrayMaxItems::check(&[1, 2, 3], 2));
    }
}
