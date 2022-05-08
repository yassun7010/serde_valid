use crate::traits::Length;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
pub fn validate_string_max_length<T>(value: &T, max_length: usize) -> bool
where
    T: Length + ?Sized,
{
    max_length >= value.length()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_max_length_ascii_is_true() {
        assert!(validate_string_max_length("abcde", 5));
        assert!(validate_string_max_length("abcde", 6));
    }

    #[test]
    fn test_validate_string_max_length_unicode_is_true() {
        assert!(validate_string_max_length("a̐éö̲", 3));
    }

    #[test]
    fn test_validate_string_max_length_japanese_is_true() {
        assert!(validate_string_max_length("あ堯", 2));
    }

    #[test]
    fn test_validate_string_max_length_emoji_is_true() {
        assert!(validate_string_max_length("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦", 5));
    }

    #[test]
    fn test_validate_string_max_length_string_type() {
        assert!(validate_string_max_length(&String::from("abcde"), 5));
    }

    #[test]
    fn test_validate_string_max_length_cow_str_type() {
        assert!(validate_string_max_length(&Cow::from("abcde"), 5));
    }

    #[test]
    fn test_validate_string_max_length_vec_u8_type() {
        assert!(validate_string_max_length(&"abcde".as_bytes().to_vec(), 5));
    }

    #[test]
    fn test_validate_string_max_length_vec_char_type() {
        assert!(validate_string_max_length(&vec!['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_string_max_length_u8_array_type() {
        assert!(validate_string_max_length("abcde".as_bytes(), 5));
    }

    #[test]
    fn test_validate_string_max_length_char_array_type() {
        assert!(validate_string_max_length(&['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_string_max_length_os_str_type() {
        assert!(validate_string_max_length(OsStr::new("fo�o"), 4));
    }

    #[test]
    fn test_validate_string_max_length_os_string_type() {
        assert!(validate_string_max_length(&OsString::from("fo�o"), 4));
    }

    #[test]
    fn test_validate_string_max_length_path_type() {
        assert!(validate_string_max_length(Path::new("./foo/bar.txt"), 13));
    }

    #[test]
    fn test_validate_string_max_length_path_buf_type() {
        assert!(validate_string_max_length(
            &PathBuf::from("./foo/bar.txt"),
            13
        ));
    }
}
