use crate::traits::Length;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
pub trait ValidateStringMaxLength {
    fn check(&self, max_length: usize) -> bool;
}

impl<T> ValidateStringMaxLength for T
where
    T: Length + ?Sized,
{
    fn check(&self, max_length: usize) -> bool {
        max_length >= self.length()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_max_length_ascii_is_true() {
        assert!(ValidateStringMaxLength::check(&"abcde", 5));
        assert!(ValidateStringMaxLength::check(&"abcde", 6));
    }

    #[test]
    fn test_validate_string_max_length_unicode_is_true() {
        assert!(ValidateStringMaxLength::check(&"a̐éö̲", 3));
    }

    #[test]
    fn test_validate_string_max_length_japanese_is_true() {
        assert!(ValidateStringMaxLength::check(&"あ堯", 2));
    }

    #[test]
    fn test_validate_string_max_length_emoji_is_true() {
        assert!(ValidateStringMaxLength::check(&"😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦", 5));
    }

    #[test]
    fn test_validate_string_max_length_string_type() {
        assert!(ValidateStringMaxLength::check(&String::from("abcde"), 5));
    }

    #[test]
    fn test_validate_string_max_length_cow_str_type() {
        assert!(ValidateStringMaxLength::check(&Cow::from("abcde"), 5));
    }

    #[test]
    fn test_validate_string_max_length_vec_u8_type() {
        assert!(ValidateStringMaxLength::check(
            &"abcde".as_bytes().to_vec(),
            5
        ));
    }

    #[test]
    fn test_validate_string_max_length_vec_char_type() {
        assert!(ValidateStringMaxLength::check(&vec!['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_string_max_length_u8_array_type() {
        assert!(ValidateStringMaxLength::check("abcde".as_bytes(), 5));
    }

    #[test]
    fn test_validate_string_max_length_char_array_type() {
        assert!(ValidateStringMaxLength::check(&['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_string_max_length_os_str_type() {
        assert!(ValidateStringMaxLength::check(&OsStr::new("fo�o"), 4));
    }

    #[test]
    fn test_validate_string_max_length_os_string_type() {
        assert!(ValidateStringMaxLength::check(&OsString::from("fo�o"), 4));
    }

    #[test]
    fn test_validate_string_max_length_path_type() {
        assert!(ValidateStringMaxLength::check(
            &Path::new("./foo/bar.txt"),
            13
        ));
    }

    #[test]
    fn test_validate_string_max_length_path_buf_type() {
        assert!(ValidateStringMaxLength::check(
            &PathBuf::from("./foo/bar.txt"),
            13
        ));
    }
}
