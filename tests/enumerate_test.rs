use serde_json::json;
use serde_valid::Validate;

#[test]
fn enumerate_integer_type_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate(1, 2, 3))]
        val: i32,
    }

    let s = TestStruct { val: 1 };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_float_type_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate(0.3, 1.2, 1.5))]
        val: f32,
    }

    let s = TestStruct { val: 0.3 };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_str_type_test() {
    #[derive(Validate)]
    struct TestStruct<'a> {
        #[validate(enumerate("a", "b"))]
        val: &'a str,
    }

    let s = TestStruct { val: "a" };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_string_type_test() {
    #[derive(Validate)]
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
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate(1, 2, 3, 4, 5))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![3, 4] };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_option_type_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate(1, 2, 3))]
        val: Option<i32>,
    }

    let s = TestStruct { val: Some(3) };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_vec_option_type_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate(3))]
        val: Vec<Option<i32>>,
    }

    let s = TestStruct { val: vec![Some(3)] };
    assert!(s.validate().is_ok());
}

#[test]
fn enumerate_is_err_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate(0.3, 1.2, 1.5))]
        val: f32,
    }

    let s = TestStruct { val: 0.1 };
    assert!(s.validate().is_err());
}

#[test]
fn enumerate_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate(1, 2, 3))]
        val: i32,
    }

    let s = TestStruct { val: 4 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "`4` must be in [1, 2, 3], but not."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn enumerate_custom_err_message_test() {
    fn error_message(_params: &serde_valid::validation::error::EnumerateParams) -> String {
        "this is custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(enumerate(1, 2, 3, message_fn(error_message)))]
        val: i32,
    }

    let s = TestStruct { val: 4 };

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
