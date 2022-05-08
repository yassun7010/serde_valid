use crate::traits::Length;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
pub trait ValidateStringMinLength {
    fn check(&self, min_length: usize) -> bool;
}

impl<T> ValidateStringMinLength for T
where
    T: Length + ?Sized,
{
    fn check(&self, min_length: usize) -> bool {
        min_length <= self.length()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_min_length_ascii_is_true() {
        assert!(ValidateStringMinLength::check(&"abcde", 5));
        assert!(ValidateStringMinLength::check(&"abcde", 4));
    }

    #[test]
    fn test_validate_string_min_length_unicode_is_true() {
        assert!(ValidateStringMinLength::check(&"a̐éö̲", 3));
    }

    #[test]
    fn test_validate_string_min_length_japanese_is_true() {
        assert!(ValidateStringMinLength::check(&"あ堯", 2));
    }

    #[test]
    fn test_validate_string_min_length_emoji_is_true() {
        assert!(ValidateStringMinLength::check(&"😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦", 5));
    }

    #[test]
    fn test_validate_string_min_length_string_type() {
        assert!(ValidateStringMinLength::check(&String::from("abcde"), 5));
    }

    #[test]
    fn test_validate_string_min_length_cow_str_type() {
        assert!(ValidateStringMinLength::check(&Cow::from("abcde"), 5));
    }

    #[test]
    fn test_validate_string_min_length_vec_u8_type() {
        assert!(ValidateStringMinLength::check(
            &"abcde".as_bytes().to_vec(),
            5
        ));
    }

    #[test]
    fn test_validate_string_min_length_vec_char_type() {
        assert!(ValidateStringMinLength::check(&vec!['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_string_min_length_u8_array_type() {
        assert!(ValidateStringMinLength::check("abcde".as_bytes(), 5));
    }

    #[test]
    fn test_validate_string_min_length_char_array_type() {
        assert!(ValidateStringMinLength::check(&['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_string_min_length_os_str_type() {
        assert!(ValidateStringMinLength::check(&OsStr::new("fo�o"), 4));
    }

    #[test]
    fn test_validate_string_min_length_os_string_type() {
        assert!(ValidateStringMinLength::check(&OsString::from("fo�o"), 4));
    }

    #[test]
    fn test_validate_string_min_length_path_type() {
        assert!(ValidateStringMinLength::check(
            &Path::new("./foo/bar.txt"),
            13
        ));
    }

    #[test]
    fn test_validate_string_min_length_path_buf_type() {
        assert!(ValidateStringMinLength::check(
            &PathBuf::from("./foo/bar.txt"),
            13
        ));
    }
}
