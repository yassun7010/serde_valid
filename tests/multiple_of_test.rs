use serde_json::json;
use serde_valid::Validate;

#[test]
fn multiple_of_integer_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5)]
        val: i32,
    }

    let s = TestStruct { val: 15 };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_float_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 1.0)]
        val: f32,
    }

    let s = TestStruct { val: 15.0 };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_integer_is_err_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 3)]
        val: i32,
    }

    let s = TestStruct { val: 16 };
    assert!(s.validate().is_err());
}

#[test]
fn multiple_of_float_is_err_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 0.5)]
        val: f32,
    }

    let s = TestStruct { val: 12.3 };
    assert!(s.validate().is_err());
}

#[test]
fn multiple_of_vec_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![12, 16] };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_nested_vec_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Vec<Vec<i32>>,
    }

    let s = TestStruct {
        val: vec![vec![4, 8], vec![12, 16]],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_optional_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Option<i32>,
    }

    let s = TestStruct { val: Some(12) };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_nested_optional_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Option<Option<i32>>,
    }

    let s = TestStruct {
        val: Some(Some(12)),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_vec_optional_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 4)]
        val: Vec<Option<i32>>,
    }

    let s = TestStruct {
        val: vec![Some(4), Some(8), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn multiple_of_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5)]
        val: i32,
    }

    let s = TestStruct { val: 14 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "`14` must be multiple of `5`, but not."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn multiple_of_custom_err_message_fn_test() {
    fn error_message(_params: &serde_valid::MultipleOfParams) -> String {
        "this is custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5, message_fn(error_message))]
        val: i32,
    }

    let s = TestStruct { val: 14 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is custom message."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn multiple_of_custom_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(multiple_of = 5, message = "this is custom message.")]
        val: i32,
    }

    let s = TestStruct { val: 14 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is custom message."
            ]
        }))
        .unwrap()
    );
}
