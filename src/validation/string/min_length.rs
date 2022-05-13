use crate::{traits::Length, MinLengthErrorParams};

/// Min length validation of the string.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
pub trait ValidateMinLength {
    fn validate_min_length(&self, min_length: usize) -> Result<(), MinLengthErrorParams>;
}

impl<T> ValidateMinLength for T
where
    T: Length + ?Sized,
{
    fn validate_min_length(&self, min_length: usize) -> Result<(), MinLengthErrorParams> {
        if min_length <= self.length() {
            Ok(())
        } else {
            Err(MinLengthErrorParams::new(min_length))
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
    fn test_validate_string_min_length_ascii_is_true() {
        assert!(ValidateMinLength::validate_min_length(&"abcde", 5).is_ok());
        assert!(ValidateMinLength::validate_min_length(&"abcde", 4).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_unicode_is_true() {
        assert!(ValidateMinLength::validate_min_length(&"a̐éö̲", 3).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_japanese_is_true() {
        assert!(ValidateMinLength::validate_min_length(&"あ堯", 2).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_emoji_is_true() {
        assert!(ValidateMinLength::validate_min_length(&"😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦", 5).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_string_type() {
        assert!(ValidateMinLength::validate_min_length(&String::from("abcde"), 5).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_cow_str_type() {
        assert!(ValidateMinLength::validate_min_length(&Cow::from("abcde"), 5).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_vec_u8_type() {
        assert!(ValidateMinLength::validate_min_length(&"abcde".as_bytes().to_vec(), 5).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_vec_char_type() {
        assert!(ValidateMinLength::validate_min_length(&vec!['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_u8_array_type() {
        assert!(ValidateMinLength::validate_min_length("abcde".as_bytes(), 5).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_char_array_type() {
        assert!(ValidateMinLength::validate_min_length(&['a', 'b', 'c'], 3).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_os_str_type() {
        assert!(ValidateMinLength::validate_min_length(&OsStr::new("fo�o"), 4).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_os_string_type() {
        assert!(ValidateMinLength::validate_min_length(&OsString::from("fo�o"), 4).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_path_type() {
        assert!(ValidateMinLength::validate_min_length(&Path::new("./foo/bar.txt"), 13).is_ok());
    }

    #[test]
    fn test_validate_string_min_length_path_buf_type() {
        assert!(
            ValidateMinLength::validate_min_length(&PathBuf::from("./foo/bar.txt"), 13).is_ok()
        );
    }
}
