use serde_json::json;
use serde_valid::Validate;

#[test]
fn unique_items_vec_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn unique_items_slice_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: [i32; 4],
    }

    let s = TestStruct { val: [1, 2, 3, 4] };
    assert!(s.validate().is_ok());
}

#[test]
fn unique_items_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 2] };
    assert!(s.validate().is_err());
}

#[test]
fn unique_items_err_message_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 2],
    };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "item of [1, 2, 3, 2] must be unique, but not."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn unique_items_custom_err_message_test() {
    fn error_message(_params: &serde_valid::validation::error::UniqueItemsErrorParams) -> String {
        "this is custom message.".to_string()
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(unique_items(message_fn(error_message)))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 2],
    };

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
