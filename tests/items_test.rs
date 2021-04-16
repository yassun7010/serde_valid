use serde_json::json;
use serde_valid::Validate;

#[test]
fn items_vec_type_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_array_type_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        val: [i32; 4],
    }

    let s = TestStruct { val: [1, 2, 3, 4] };
    assert!(s.validate().is_ok());
}

#[test]
fn items_min_items_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 3, max_items = 5))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_ok());
}

#[test]
fn items_min_items_is_err_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 5))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_err());
}

#[test]
fn items_max_items_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 0, max_items = 4))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_ok());
}

#[test]
fn items_max_items_is_err_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 1, max_items = 2))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_err());
}

#[test]
fn items_vec_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 2, max_items = 2))]
        val: Vec<Vec<i32>>,
    }

    let s = TestStruct {
        val: vec![vec![], vec![1, 2, 3]],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_option_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        val: Option<Vec<i32>>,
    }

    let s = TestStruct {
        val: Some(vec![1, 2, 3, 4]),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_nested_option_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 3, max_items = 3))]
        val: Option<Option<Vec<i32>>>,
    }

    let s = TestStruct {
        val: Some(Some(vec![1, 2, 3])),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_vec_optional_type_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 3, max_items = 3))]
        val: Vec<Option<Vec<i32>>>,
    }

    let s = TestStruct {
        val: vec![Some(vec![1, 2, 3, 4]), Some(vec![1, 2, 3, 4]), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "items length of [1, 2, 3] must be in `4 <= length <= 4`, but `3`."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn items_custom_err_message_fn_test() {
    fn error_message(_params: &serde_valid::validation::error::ItemsParams) -> String {
        "this is custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 4, message_fn(error_message)))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };

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
fn items_custom_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 4, message = "this is custom message."))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };

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
