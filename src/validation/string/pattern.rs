use crate::traits::IsMatch;
use regex::Regex;

/// RegularExpressions validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#regular-expressions>
pub trait ValidateStringPattern {
    fn check(&self, pattern: &Regex) -> bool;
}

impl<T> ValidateStringPattern for T
where
    T: IsMatch + ?Sized,
{
    fn check(&self, pattern: &Regex) -> bool {
        self.is_match(pattern)
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
        assert!(ValidateStringPattern::check(
            "2020-09-10",
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_pattern_string_type() {
        assert!(ValidateStringPattern::check(
            &String::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_pattern_cow_str_type() {
        assert!(ValidateStringPattern::check(
            &Cow::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_pattern_os_str_type() {
        assert!(ValidateStringPattern::check(
            OsStr::new("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_pattern_os_string_type() {
        assert!(ValidateStringPattern::check(
            &OsString::from("2020-09-10"),
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_pattern_path_type() {
        assert!(ValidateStringPattern::check(
            Path::new("./foo/bar.txt"),
            &Regex::new(r"^*.txt$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_pattern_path_buf_type() {
        assert!(ValidateStringPattern::check(
            &PathBuf::from("./foo/bar.txt"),
            &Regex::new(r"^*.txt$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_pattern_is_false() {
        assert!(!ValidateStringPattern::check(
            "2020/09/10",
            &Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }
}
