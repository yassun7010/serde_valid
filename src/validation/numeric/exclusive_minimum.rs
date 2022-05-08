/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub fn validate_numeric_exclusive_minimum<T>(value: T, exclusive_minimum: T) -> bool
where
    T: PartialOrd + PartialEq,
{
    value > exclusive_minimum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_exclusive_minimum_is_true() {
        // Unspecified generic type:
        assert!(validate_numeric_exclusive_minimum(10, 9));
    }

    #[test]
    fn test_validate_numeric_exclusive_minimum_is_false() {
        assert!(!validate_numeric_exclusive_minimum(5, 6));
        assert!(!validate_numeric_exclusive_minimum(5, 5));
    }

    #[test]
    fn test_validate_numeric_exclusive_minimum_specified_type() {
        assert!(validate_numeric_exclusive_minimum(0.5, 0.2));
        assert!(validate_numeric_exclusive_minimum(5u8, 0));
        assert!(validate_numeric_exclusive_minimum(4u16, 0));
        assert!(validate_numeric_exclusive_minimum(6u32, 0));
    }
}
