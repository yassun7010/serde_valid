use unicode_segmentation::UnicodeSegmentation;

pub trait Length {
    fn length(&self) -> usize;
}

impl Length for str {
    fn length(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl Length for &str {
    fn length(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl Length for String {
    fn length(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl<'a> Length for std::borrow::Cow<'a, str> {
    fn length(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl Length for Vec<u8> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Length for Vec<char> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Length for [u8] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<const N: usize> Length for [u8; N] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Length for [char] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<const N: usize> Length for [char; N] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl Length for std::ffi::OsStr {
    fn length(&self) -> usize {
        self.to_string_lossy().length()
    }
}

impl Length for &std::ffi::OsStr {
    fn length(&self) -> usize {
        self.to_string_lossy().length()
    }
}

impl Length for std::ffi::OsString {
    fn length(&self) -> usize {
        self.as_os_str().length()
    }
}

impl Length for std::path::Path {
    fn length(&self) -> usize {
        self.as_os_str().length()
    }
}

impl Length for &std::path::Path {
    fn length(&self) -> usize {
        self.as_os_str().length()
    }
}

impl Length for std::path::PathBuf {
    fn length(&self) -> usize {
        self.as_os_str().length()
    }
}
