use serde_valid::Validate;

#[test]
fn length_test() {
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
fn length_array_type_is_ok_test() {
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
fn length_nested_array_type_is_ok_test() {
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
fn length_array_optional_type_is_ok_test() {
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
