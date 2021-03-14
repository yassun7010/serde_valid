pub trait IsMatch {
    fn is_match(&self, pattern: regex::Regex) -> bool;
}

impl IsMatch for str {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        pattern.is_match(self)
    }
}

impl IsMatch for &str {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        pattern.is_match(self)
    }
}

impl IsMatch for String {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        pattern.is_match(self)
    }
}

impl<'a> IsMatch for std::borrow::Cow<'a, str> {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        pattern.is_match(self)
    }
}

impl IsMatch for std::ffi::OsStr {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        pattern.is_match(&self.to_string_lossy())
    }
}

impl<'a> IsMatch for &'a std::ffi::OsStr {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        pattern.is_match(&self.to_string_lossy())
    }
}

impl IsMatch for std::ffi::OsString {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        pattern.is_match(&self.to_string_lossy())
    }
}

impl IsMatch for std::path::Path {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        self.as_os_str().is_match(pattern)
    }
}

impl IsMatch for &std::path::Path {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        self.as_os_str().is_match(pattern)
    }
}

impl IsMatch for std::path::PathBuf {
    fn is_match(&self, pattern: regex::Regex) -> bool {
        self.as_os_str().is_match(pattern)
    }
}
