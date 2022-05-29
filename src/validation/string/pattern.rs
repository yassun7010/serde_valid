use crate::{traits::IsMatch, PatternErrorParams};
use regex::Regex;

/// Pattern validation of the string.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#regular-expressions>
pub trait ValidatePattern {
    fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternErrorParams>;
}

impl<T> ValidatePattern for T
where
    T: IsMatch + ?Sized,
{
    fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternErrorParams> {
        if self.is_match(pattern) {
            Ok(())
        } else {
            Err(PatternErrorParams::new(pattern))
        }
    }
}

impl<T> ValidatePattern for Vec<T>
where
    T: ValidatePattern,
{
    fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternErrorParams> {
        for item in self {
            item.validate_pattern(pattern)?
        }

        Ok(())
    }
}

impl<T, const N: usize> ValidatePattern for [T; N]
where
    T: ValidatePattern,
{
    fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternErrorParams> {
        for item in self {
            item.validate_pattern(pattern)?
        }

        Ok(())
    }
}

impl<T> ValidatePattern for Option<T>
where
    T: ValidatePattern,
{
    fn validate_pattern(&self, pattern: &Regex) -> Result<(), PatternErrorParams> {
        match self {
            Some(value) => value.validate_pattern(pattern),
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_pattern_str_type() {
        assert!(ValidatePattern::validate_pattern(
            "2020-09-10",
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_string_type() {
        assert!(ValidatePattern::validate_pattern(
            &String::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_cow_str_type() {
        assert!(ValidatePattern::validate_pattern(
            &Cow::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_os_str_type() {
        assert!(ValidatePattern::validate_pattern(
            OsStr::new("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_os_string_type() {
        assert!(ValidatePattern::validate_pattern(
            &OsString::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_path_type() {
        assert!(ValidatePattern::validate_pattern(
            Path::new("./foo/bar.txt"),
            &Regex::new(r"^*.txt$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_path_buf_type() {
        assert!(ValidatePattern::validate_pattern(
            &PathBuf::from("./foo/bar.txt"),
            &Regex::new(r"^*.txt$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_is_false() {
        assert!(ValidatePattern::validate_pattern(
            "2020/09/10",
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_err());
    }
}
