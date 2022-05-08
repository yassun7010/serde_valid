use crate::{traits::IsMatch, PatternErrorParams};
use regex::Regex;

/// RegularExpressions validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#regular-expressions>
pub trait ValidateStringPattern {
    fn validate(&self, pattern: &Regex) -> Result<(), PatternErrorParams>;
}

impl<T> ValidateStringPattern for T
where
    T: IsMatch + ?Sized,
{
    fn validate(&self, pattern: &Regex) -> Result<(), PatternErrorParams> {
        if self.is_match(pattern) {
            Ok(())
        } else {
            Err(PatternErrorParams::new(pattern))
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
        assert!(ValidateStringPattern::validate(
            "2020-09-10",
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_string_type() {
        assert!(ValidateStringPattern::validate(
            &String::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_cow_str_type() {
        assert!(ValidateStringPattern::validate(
            &Cow::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_os_str_type() {
        assert!(ValidateStringPattern::validate(
            OsStr::new("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_os_string_type() {
        assert!(ValidateStringPattern::validate(
            &OsString::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_path_type() {
        assert!(ValidateStringPattern::validate(
            Path::new("./foo/bar.txt"),
            &Regex::new(r"^*.txt$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_path_buf_type() {
        assert!(ValidateStringPattern::validate(
            &PathBuf::from("./foo/bar.txt"),
            &Regex::new(r"^*.txt$").unwrap()
        )
        .is_ok());
    }

    #[test]
    fn test_validate_string_pattern_is_false() {
        assert!(ValidateStringPattern::validate(
            "2020/09/10",
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        )
        .is_err());
    }
}
