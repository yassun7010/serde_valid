use serde_valid::Validate;

#[test]
fn enumerate_integer_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(enumerate(1, 2, 3))]
        val: i32,
    }

    let s = TestStruct { val: 1 };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_float_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(enumerate(0.3, 1.2, 1.5))]
        val: f32,
    }

    let s = TestStruct { val: 0.3 };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_str_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        #[validate(enumerate("a", "b"))]
        val: &'a str,
    }

    let s = TestStruct { val: "a" };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_string_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(enumerate("a", "b"))]
        val: String,
    }

    let s = TestStruct {
        val: "a".to_string(),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_vec_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(enumerate(1, 2, 3, 4, 5))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![3, 4] };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_option_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(enumerate(1, 2, 3))]
        val: Option<i32>,
    }

    let s = TestStruct { val: Some(3) };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_vec_option_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(enumerate(3))]
        val: Vec<Option<i32>>,
    }

    let s = TestStruct { val: vec![Some(3)] };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(enumerate(0.3, 1.2, 1.5))]
        val: f32,
    }

    let s = TestStruct { val: 0.1 };
    assert!(s.validate().is_err());
}

#[test]
fn enumerate_err_message_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(enumerate(1, 2, 3))]
        val: i32,
    }

    let s = TestStruct { val: 4 };

    let mut results = s.validate().unwrap_err().into_iter();
    let (field, errors) = results.next().unwrap();

    assert!(results.next().is_none());
    assert_eq!(field, "val");

    let mut errors = errors.iter();

    assert_eq!(
        format!("{}", errors.next().unwrap()),
        "`4` must be in [1, 2, 3], but not."
    );
    assert!(errors.next().is_none());
}
