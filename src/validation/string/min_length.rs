use crate::traits::Length;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
pub fn validate_string_min_length<T>(value: &T, min_length: usize) -> bool
where
    T: Length + ?Sized,
{
    min_length <= value.length()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::ffi::{OsStr, OsString};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_validate_string_min_length_ascii_is_true() {
        assert!(validate_string_min_length("abcde", 5));
        assert!(validate_string_min_length("abcde", 4));
    }

    #[test]
    fn test_validate_string_min_length_unicode_is_true() {
        assert!(validate_string_min_length("a̐éö̲", 3));
    }

    #[test]
    fn test_validate_string_min_length_japanese_is_true() {
        assert!(validate_string_min_length("あ堯", 2));
    }

    #[test]
    fn test_validate_string_min_length_emoji_is_true() {
        assert!(validate_string_min_length("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦", 5));
    }

    #[test]
    fn test_validate_string_min_length_string_type() {
        assert!(validate_string_min_length(&String::from("abcde"), 5));
    }

    #[test]
    fn test_validate_string_min_length_cow_str_type() {
        assert!(validate_string_min_length(&Cow::from("abcde"), 5));
    }

    #[test]
    fn test_validate_string_min_length_vec_u8_type() {
        assert!(validate_string_min_length(&"abcde".as_bytes().to_vec(), 5));
    }

    #[test]
    fn test_validate_string_min_length_vec_char_type() {
        assert!(validate_string_min_length(&vec!['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_string_min_length_u8_array_type() {
        assert!(validate_string_min_length("abcde".as_bytes(), 5));
    }

    #[test]
    fn test_validate_string_min_length_char_array_type() {
        assert!(validate_string_min_length(&['a', 'b', 'c'], 3));
    }

    #[test]
    fn test_validate_string_min_length_os_str_type() {
        assert!(validate_string_min_length(OsStr::new("fo�o"), 4));
    }

    #[test]
    fn test_validate_string_min_length_os_string_type() {
        assert!(validate_string_min_length(&OsString::from("fo�o"), 4));
    }

    #[test]
    fn test_validate_string_min_length_path_type() {
        assert!(validate_string_min_length(Path::new("./foo/bar.txt"), 13));
    }

    #[test]
    fn test_validate_string_min_length_path_buf_type() {
        assert!(validate_string_min_length(
            &PathBuf::from("./foo/bar.txt"),
            13
        ));
    }
}
