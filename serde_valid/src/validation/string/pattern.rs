use crate::traits::IsMatch;
use regex::Regex;

#[allow(dead_code)]
pub fn validate_pattern<T>(value: &T, pattern: Regex) -> bool
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
    fn test_validate_pattern_str_type() {
        assert!(validate_pattern(
            "2020-09-10",
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_pattern_string_type() {
        assert!(validate_pattern(
            &String::from("2020-09-10"),
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_pattern_cow_str_type() {
        assert!(validate_pattern(
            &Cow::from("2020-09-10"),
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_pattern_os_str_type() {
        assert!(validate_pattern(
            OsStr::new("2020-09-10"),
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_pattern_os_string_type() {
        assert!(validate_pattern(
            &OsString::from("2020-09-10"),
            Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap()
        ));
    }

    #[test]
    fn test_validate_pattern_path_type() {
        assert!(validate_pattern(
            Path::new("./foo/bar.txt"),
            Regex::new(r"^*.txt$").unwrap()
        ));
    }

    #[test]
    fn test_validate_pattern_path_buf_type() {
        assert!(validate_pattern(
            &PathBuf::from("./foo/bar.txt"),
            Regex::new(r"^*.txt$").unwrap()
        ));
    }
}
