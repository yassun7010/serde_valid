use serde_valid::Validate;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};

#[test]
fn pattern_string_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: String,
    }

    let s = TestStruct {
        val: String::from("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: &'a str,
    }

    let s = TestStruct { val: "2020-09-10" };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_cow_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: Cow<'a, str>,
    }

    let s = TestStruct {
        val: Cow::from("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_os_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: &'a OsStr,
    }

    let s = TestStruct {
        val: OsStr::new("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_os_string_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: OsString,
    }

    let s = TestStruct {
        val: OsString::from("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_path_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: &'a std::path::Path,
    }

    let s = TestStruct {
        val: std::path::Path::new("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_path_buf_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: std::path::PathBuf,
    }

    let s = TestStruct {
        val: std::path::PathBuf::from("2020-09-10"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn pattern_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$")]
        val: String,
    }

    let s = TestStruct {
        val: String::from("2020/09/10"),
    };
    assert!(s.validate().is_err());
}
