/// Range validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#range>
pub fn validate_numeric_minimum<T>(value: T, minimum: T) -> bool
where
    T: PartialOrd + PartialEq,
{
    value >= minimum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_numeric_minimum_is_true() {
        assert!(validate_numeric_minimum(10, 9));
        assert!(validate_numeric_minimum(10, 10));
    }

    #[test]
    fn test_validate_numeric_minimum_is_false() {
        assert!(!validate_numeric_minimum(5, 6));
    }

    #[test]
    fn test_validate_numeric_minimum_specified_type() {
        assert!(validate_numeric_minimum(0.5, 0.2));
        assert!(validate_numeric_minimum(5u8, 0));
        assert!(validate_numeric_minimum(4u16, 0));
        assert!(validate_numeric_minimum(6u32, 0));
    }
}
