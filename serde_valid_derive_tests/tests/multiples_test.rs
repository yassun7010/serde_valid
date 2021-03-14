use serde_valid::Validate;

#[test]
fn integer_multiple_of_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5)]
        val: i32,
    }

    let s = TestStruct { val: 15 };
    assert!(s.validate().is_ok());
}

#[test]
fn float_multiple_of_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(multiple_of = 1.0)]
        val: f32,
    }

    let s = TestStruct { val: 15.0 };
    assert!(s.validate().is_ok());
}

#[test]
fn integer_multiple_of_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(multiple_of = 3)]
        val: i32,
    }

    let s = TestStruct { val: 16 };
    assert!(s.validate().is_err());
}

#[test]
fn float_multiple_of_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(multiple_of = 0.5)]
        val: f32,
    }

    let s = TestStruct { val: 12.3 };
    assert!(s.validate().is_err());
}

#[test]
fn float_optional_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Option<i32>,
    }

    let s = TestStruct { val: Some(12) };
    assert!(s.validate().is_ok());
}

#[test]
fn float_nested_optional_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Option<Option<i32>>,
    }

    let s = TestStruct {
        val: Some(Some(12)),
    };
    assert!(s.validate().is_ok());
}
