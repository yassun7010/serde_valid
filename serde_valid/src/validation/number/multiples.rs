/// MultipleOf validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#multiples>
pub fn validate_number_multiples<T>(value: T, multiple_of: T) -> bool
where
    T: PartialEq + std::ops::Rem<Output = T> + num_traits::Zero,
{
    value % multiple_of == T::zero()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_number_multiples_integer_is_true() {
        assert!(validate_number_multiples(10, 5));
    }

    #[test]
    fn test_validate_number_multiples_float_is_true() {
        assert!(validate_number_multiples(12.0, 1.0));
        assert!(validate_number_multiples(12.5, 0.5));
    }

    #[test]
    fn test_validate_number_multiples_integer_is_false() {
        assert!(!validate_number_multiples(10, 3));
    }

    #[test]
    fn test_validate_number_multiples_float_is_false() {
        assert!(!validate_number_multiples(12.0, 5.0));
        assert!(!validate_number_multiples(12.5, 0.3));
    }
}
