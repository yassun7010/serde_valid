/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub fn validate_numeric_exclusive_maximum<T>(value: T, exclusive_maximum: T) -> bool
where
    T: PartialOrd + PartialEq,
{
    value < exclusive_maximum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_exclusive_maximum_is_true() {
        // Unspecified generic type:
        assert!(validate_numeric_exclusive_maximum(10, 11));
    }

    #[test]
    fn test_validate_numeric_exclusive_maximum_is_false() {
        assert!(!validate_numeric_exclusive_maximum(5, 4));
        assert!(!validate_numeric_exclusive_maximum(10, 10));
    }

    #[test]
    fn test_validate_numeric_exclusive_maximum_specified_type() {
        assert!(validate_numeric_exclusive_maximum(0.2, 0.5));
        assert!(validate_numeric_exclusive_maximum(0, 5u8));
        assert!(validate_numeric_exclusive_maximum(0, 4u16));
        assert!(validate_numeric_exclusive_maximum(0, 6u32));
    }
}
