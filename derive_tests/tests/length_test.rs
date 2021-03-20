use serde_valid::Validate;
use std::borrow::Cow;
use std::ffi::{OsStr, OsString};

#[test]
fn length_string_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 4, max_length = 4))]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(length(min_length = 4, max_length = 4))]
        val: &'a str,
    }

    let s = TestStruct { val: "test" };
    assert!(s.validate().is_ok());
}

#[test]
fn length_cow_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(length(min_length = 4, max_length = 4))]
        val: Cow<'a, str>,
    }

    let s = TestStruct {
        val: Cow::from("test"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_vec_u8_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 4, max_length = 4))]
        val: Vec<u8>,
    }

    let s = TestStruct {
        val: "test".as_bytes().to_vec(),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_vec_char_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 4, max_length = 4))]
        val: Vec<char>,
    }

    let s = TestStruct {
        val: vec!['t', 'e', 's', 't'],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_u8_array_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 4, max_length = 4))]
        val: [u8; 4],
    }

    let s = TestStruct {
        val: [0x74, 0x65, 0x73, 0x74],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_char_array_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 4, max_length = 4))]
        val: [char; 4],
    }

    let s = TestStruct {
        val: ['t', 'e', 's', 't'],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_os_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(length(min_length = 4, max_length = 4))]
        val: &'a OsStr,
    }

    let s = TestStruct {
        val: OsStr::new("fo�o"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_os_string_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 4, max_length = 4))]
        val: OsString,
    }

    let s = TestStruct {
        val: OsString::from("fo�o"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_path_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(length(min_length = 13, max_length = 13))]
        val: &'a std::path::Path,
    }

    let s = TestStruct {
        val: std::path::Path::new("./foo/bar.txt"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_path_buf_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 13, max_length = 13))]
        val: std::path::PathBuf,
    }

    let s = TestStruct {
        val: std::path::PathBuf::from("./foo/bar.txt"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_min_length_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 0, max_length = 10))]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    assert!(s.validate().is_ok());
}

#[test]
fn length_min_length_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 1, max_length = 10))]
        val: String,
    }

    let s = TestStruct { val: String::new() };
    assert!(s.validate().is_err());
}

#[test]
fn length_max_length_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 0, max_length = 5))]
        val: String,
    }

    let s = TestStruct {
        val: String::from("abcde"),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_max_length_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 1, max_length = 3))]
        val: String,
    }

    let s = TestStruct {
        val: String::from("abcd"),
    };
    assert!(s.validate().is_err());
}

#[test]
fn length_vec_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 0, max_length = 4))]
        val: Vec<String>,
    }

    let s = TestStruct {
        val: vec![String::from("abcd"), String::from("efg")],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_nested_vec_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 0, max_length = 3))]
        val: Vec<Vec<String>>,
    }

    let s = TestStruct {
        val: vec![
            vec![String::from(""), String::from("1")],
            vec![String::from("12"), String::from("123")],
        ],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_option_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 0, max_length = 5))]
        val: Option<String>,
    }

    let s = TestStruct {
        val: Some(String::from("abcd")),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_nested_option_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 0, max_length = 5))]
        val: Option<Option<String>>,
    }

    let s = TestStruct {
        val: Some(Some(String::from("abcd"))),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_vec_optional_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 0, max_length = 5))]
        val: Vec<Option<String>>,
    }

    let s = TestStruct {
        val: vec![Some(String::from("abc")), Some(String::from("abcde")), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn length_err_message_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(length(min_length = 1, max_length = 3))]
        val: String,
    }

    let s = TestStruct {
        val: String::from("test"),
    };
    for (field, error) in s.validate().unwrap_err() {
        assert_eq!(field, "val");
        assert_eq!(
            format!("{}", error),
            "length of \"test\" must be in `1 <= length <= 3`, but not."
        )
    }
}
