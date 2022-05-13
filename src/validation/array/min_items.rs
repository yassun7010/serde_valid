/// Min length validation of the array items.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
pub trait ValidateMinItems {
    fn validate_min_items(&self, min_items: usize) -> Result<(), crate::MinItemsErrorParams>;
}

impl<T> ValidateMinItems for Vec<T> {
    fn validate_min_items(&self, min_items: usize) -> Result<(), crate::MinItemsErrorParams> {
        if min_items <= self.len() {
            Ok(())
        } else {
            Err(crate::MinItemsErrorParams::new(min_items))
        }
    }
}

impl<T, const N: usize> ValidateMinItems for [T; N] {
    fn validate_min_items(&self, min_items: usize) -> Result<(), crate::MinItemsErrorParams> {
        if min_items <= self.len() {
            Ok(())
        } else {
            Err(crate::MinItemsErrorParams::new(min_items))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_min_items_is_true() {
        assert!(ValidateMinItems::validate_min_items(&[1, 2, 3], 3).is_ok());
    }

    #[test]
    fn test_validate_array_min_items_is_false() {
        assert!(ValidateMinItems::validate_min_items(&[1, 2, 3], 4).is_err());
    }

    #[test]
    fn test_validate_array_min_items_vec_is_true() {
        assert!(ValidateMinItems::validate_min_items(&vec!['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_array_min_items_array_is_true() {
        assert!(ValidateMinItems::validate_min_items(&['a', 'b', 'c'], 3).is_ok());
    }
}
