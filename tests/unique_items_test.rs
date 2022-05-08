use serde_json::json;
use serde_valid::{Validate, ValidateUniqueItems};

#[test]
fn unique_items_vec_type() {
    #[derive(Validate)]
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
fn unique_items_slice_type() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: [i32; 4],
    }

    let s = TestStruct { val: [1, 2, 3, 4] };
    assert!(s.validate().is_ok());
}

#[test]
fn unique_items_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 2] };
    assert!(s.validate().is_err());
}

#[test]
fn unique_items_err_message() {
    #[derive(Validate)]
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
                "items must be unique."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn unique_items_custom_err_message_fn() {
    fn error_message(_params: &serde_valid::UniqueItemsErrorParams) -> String {
        "this is custom message.".to_string()
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(unique_items, message_fn(error_message))]
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

#[test]
fn unique_items_custom_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(unique_items, message = "this is custom message.")]
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

#[test]
fn unique_items_trait() {
    struct MyType(Vec<i32>);

    impl ValidateUniqueItems for MyType {
        fn validate_unique_items(&self) -> Result<(), serde_valid::UniqueItemsErrorParams> {
            self.0.validate_unique_items()
        }
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: MyType,
    }

    let s = TestStruct {
        val: MyType(vec![1, 2, 3]),
    };

    assert!(s.validate().is_ok());
}
