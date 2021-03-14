/// MultipleOf validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/numeric.html#id6>
pub fn validate_multiples<T>(value: T, multiple_of: T) -> bool
where
    T: PartialEq + std::ops::Rem<Output = T> + num_traits::Zero,
{
    value % multiple_of == T::zero()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_multiples_generic_ok() {
        assert!(validate_multiples(10, 5));
        assert!(validate_multiples(12.0, 1.0));
        assert!(validate_multiples(12.5, 0.5));
    }

    #[test]
    fn test_validate_multiples_generic_fail() {
        assert!(!validate_multiples(10, 3));
        assert!(!validate_multiples(12.0, 5.0));
        assert!(!validate_multiples(12.5, 0.3));
    }
}
