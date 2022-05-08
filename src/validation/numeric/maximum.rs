/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub fn validate_numeric_maximum<T>(value: T, maximum: T) -> bool
where
    T: PartialOrd + PartialEq,
{
    value <= maximum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_maximum_is_true() {
        // Unspecified generic type:
        assert!(validate_numeric_maximum(10, 11));
        assert!(validate_numeric_maximum(10, 10));
    }

    #[test]
    fn test_validate_numeric_maximum_is_false() {
        assert!(!validate_numeric_maximum(5, 4));
    }

    #[test]
    fn test_validate_numeric_maximum_specified_type() {
        assert!(validate_numeric_maximum(0.2, 0.5));
        assert!(validate_numeric_maximum(0, 5u8));
        assert!(validate_numeric_maximum(0, 4u16));
        assert!(validate_numeric_maximum(0, 6u32));
    }
}
