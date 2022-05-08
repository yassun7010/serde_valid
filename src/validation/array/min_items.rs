/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
pub trait ValidateArrayMinItems {
    fn check(&self, min_items: usize) -> bool;
}

impl<T> ValidateArrayMinItems for Vec<T> {
    fn check(&self, min_items: usize) -> bool {
        min_items <= self.len()
    }
}

impl<T, const N: usize> ValidateArrayMinItems for [T; N] {
    fn check(&self, min_items: usize) -> bool {
        min_items <= self.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_min_items_is_true() {
        assert!(ValidateArrayMinItems::check(&[1, 2, 3], 3));
    }

    #[test]
    fn test_validate_array_min_items_is_false() {
        assert!(!ValidateArrayMinItems::check(&[1, 2, 3], 4));
    }

    #[test]
    fn test_validate_array_min_items_vec_is_true() {
        assert!(ValidateArrayMinItems::check(&vec!['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_array_min_items_array_is_true() {
        assert!(ValidateArrayMinItems::check(&['a', 'b', 'c'], 3));
    }
}
