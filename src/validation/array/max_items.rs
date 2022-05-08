/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/array.html#length>
pub trait ValidateArrayMaxItems {
    fn validate(&self, max_items: usize) -> Result<(), crate::MaxItemsErrorParams>;
}

impl<T> ValidateArrayMaxItems for Vec<T> {
    fn validate(&self, max_items: usize) -> Result<(), crate::MaxItemsErrorParams> {
        if max_items >= self.len() {
            Ok(())
        } else {
            Err(crate::MaxItemsErrorParams::new(max_items))
        }
    }
}

impl<T, const N: usize> ValidateArrayMaxItems for [T; N] {
    fn validate(&self, max_items: usize) -> Result<(), crate::MaxItemsErrorParams> {
        if max_items >= self.len() {
            Ok(())
        } else {
            Err(crate::MaxItemsErrorParams::new(max_items))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_array_vec_type() {
        assert!(ValidateArrayMaxItems::validate(&vec!['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_array_max_items_array_type() {
        assert!(ValidateArrayMaxItems::validate(&['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_array_max_items_is_true() {
        assert!(ValidateArrayMaxItems::validate(&[1, 2, 3], 3).is_ok());
    }

    #[test]
    fn test_validate_array_max_items_is_false() {
        assert!(ValidateArrayMaxItems::validate(&[1, 2, 3], 2).is_err());
    }
}
