use crate::traits::IsMatch;
use regex::Regex;

/// RegularExpressions validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#regular-expressions>
pub fn validate_string_regular_expressions<T>(value: &T, pattern: Regex) -> bool
where
    T: IsMatch + ?Sized,
{
    value.is_match(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_regular_expressions_str_type() {
        assert!(validate_string_regular_expressions(
            "2020-09-10",
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_regular_expressions_string_type() {
        assert!(validate_string_regular_expressions(
            &String::from("2020-09-10"),
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_regular_expressions_cow_str_type() {
        assert!(validate_string_regular_expressions(
            &Cow::from("2020-09-10"),
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_regular_expressions_os_str_type() {
        assert!(validate_string_regular_expressions(
            OsStr::new("2020-09-10"),
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_regular_expressions_os_string_type() {
        assert!(validate_string_regular_expressions(
            &OsString::from("2020-09-10"),
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_regular_expressions_path_type() {
        assert!(validate_string_regular_expressions(
            Path::new("./foo/bar.txt"),
            Regex::new(r"^*.txt$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_regular_expressions_path_buf_type() {
        assert!(validate_string_regular_expressions(
            &PathBuf::from("./foo/bar.txt"),
            Regex::new(r"^*.txt$").unwrap()
        ));
    }

    #[test]
    fn test_validate_string_regular_expressions_is_false() {
        assert!(!validate_string_regular_expressions(
            "2020/09/10",
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }
}
