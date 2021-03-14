use crate::traits::Length;

pub fn validate_length<T>(value: &T, min_length: Option<usize>, max_length: Option<usize>) -> bool
where
    T: Length + ?Sized,
{
    let length = value.length();
    if let Some(max) = max_length {
        if max < length {
            return false;
        }
    }

    if let Some(min) = min_length {
        if length < min {
            return false;
        };
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_length_unicode_ok() {
        assert!(validate_length("abcde", Some(5), Some(5)));
        assert!(validate_length("a̐éö̲", Some(3), Some(3)));
        assert!(validate_length("あ堯", Some(2), Some(2)));
        assert!(validate_length("😍👺👻", Some(3), Some(3)));
    }

    #[test]
    fn test_validate_length_str_type() {
        assert!(validate_length("abcde", Some(5), Some(5)));
    }

    #[test]
    fn test_validate_length_string_type() {
        assert!(validate_length(&String::from("abcde"), Some(5), Some(5)));
    }

    #[test]
    fn test_validate_length_cow_str_type() {
        assert!(validate_length(&Cow::from("abcde"), Some(5), Some(5)));
    }

    #[test]
    fn test_validate_length_vec_u8_type() {
        assert!(validate_length(
            &"abcde".as_bytes().to_vec(),
            Some(5),
            Some(5)
        ));
    }

    #[test]
    fn test_validate_length_vec_char_type() {
        assert!(validate_length(&vec!['a', 'b', 'c'], Some(3), Some(3)));
    }

    #[test]
    fn test_validate_length_u8_array_type() {
        assert!(validate_length("abcde".as_bytes(), Some(5), Some(5)));
    }

    #[test]
    fn test_validate_length_char_array_type() {
        assert!(validate_length(&['a', 'b', 'c'], Some(3), Some(3)));
    }

    #[test]
    fn test_validate_length_os_str_type() {
        assert!(validate_length(OsStr::new("fo�o"), Some(4), Some(4)));
    }

    #[test]
    fn test_validate_length_os_string_type() {
        assert!(validate_length(&OsString::from("fo�o"), Some(4), Some(4)));
    }

    #[test]
    fn test_validate_length_path_type() {
        assert!(validate_length(
            Path::new("./foo/bar.txt"),
            Some(13),
            Some(13)
        ));
    }

    #[test]
    fn test_validate_length_path_buf_type() {
        assert!(validate_length(
            &PathBuf::from("./foo/bar.txt"),
            Some(13),
            Some(13)
        ));
    }

    #[test]
    fn test_validate_length_min_fail() {
        assert!(!validate_length("abcde", Some(6), None));
    }

    #[test]
    fn test_validate_length_min_ok() {
        assert!(validate_length("abcde", Some(5), None));
    }

    #[test]
    fn test_validate_length_max_fail() {
        assert!(!validate_length("abcde", None, Some(4)));
    }

    #[test]
    fn test_validate_length_max_ok() {
        assert!(validate_length("abcde", None, Some(5)));
    }
}
