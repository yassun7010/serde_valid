use unicode_segmentation::UnicodeSegmentation;

pub trait Length {
    fn length(&self) -> usize;
}

macro_rules! impl_for_str {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.graphemes(true).count()
            }
        }
    };
}

impl_for_str!(str);
impl_for_str!(&str);
impl_for_str!(String);
impl_for_str!(std::borrow::Cow<'_, str>);

macro_rules! impl_for_os_str {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.to_string_lossy().length()
            }
        }
    };
}

impl_for_os_str!(std::ffi::OsStr);
impl_for_os_str!(&std::ffi::OsStr);
impl_for_os_str!(std::ffi::OsString);
impl_for_os_str!(std::borrow::Cow<'_, std::ffi::OsStr>);

macro_rules! impl_for_path {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.as_os_str().length()
            }
        }
    };
}

impl_for_path!(std::path::Path);
impl_for_path!(&std::path::Path);
impl_for_path!(std::path::PathBuf);
impl_for_os_str!(std::borrow::Cow<'_, std::path::Path>);

macro_rules! impl_for_bytes {
    ($ty:ty) => {
        impl Length for $ty {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
    ($ty:ty, $size:tt) => {
        impl<const $size: usize> Length for [$ty; $size] {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_for_bytes!(Vec<u8>);
impl_for_bytes!(Vec<char>);
impl_for_bytes!([u8]);
impl_for_bytes!([char]);
impl_for_bytes!(u8, N);
impl_for_bytes!(char, N);
