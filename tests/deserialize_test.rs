use serde_json::json;
use serde_valid::{Deserialize, Validate};

#[test]
fn deserialize_with_validation_from_value_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 2000)]
        val: i32,
    }

    serde_valid::from_value::<TestStruct, _>(json!({ "val": 1234 })).unwrap();
}

#[test]
fn deserialize_with_validation_from_str_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 2000)]
        val: i32,
    }

    serde_valid::from_str::<TestStruct, serde_json::Value>(
        &serde_json::to_string(&json!({ "val": 1234 })).unwrap(),
    )
    .unwrap();
}

#[test]
fn deserialize_with_validation_from_slice_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 2000)]
        val: i32,
    }

    serde_valid::from_slice::<TestStruct, serde_json::Value>(b"{ \"val\": 1234 }").unwrap();
}

#[test]
fn deserialize_validation_err_to_string() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 1000)]
        val: i32,
    }

    let err = serde_valid::from_value::<TestStruct, _>(json!({ "val": 1234 })).unwrap_err();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&err.to_string()).unwrap(),
        json!({"val": ["the number must be `<= 1000`."]})
    );
}

#[test]
fn deserialize_validation_err_to_json_value() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 1000)]
        val: i32,
    }

    let err = serde_valid::from_value::<TestStruct, _>(json!({ "val": 1234 })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({"val": ["the number must be `<= 1000`."]})
    );
}
