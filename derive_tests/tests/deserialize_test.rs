use serde::Deserialize;
use serde_json::json;
use serde_valid::Validate;

#[test]
fn deserialize_is_ok_test() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 2000))]
        val: i32,
    }

    serde_valid::from_value::<TestStruct, _>(json!({ "val": 1234 })).unwrap();
}

#[test]
fn deserialize_is_err_test() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 1000))]
        val: i32,
    }

    let err = serde_valid::from_value::<TestStruct, _>(json!({ "val": 1234 })).unwrap_err();

    assert_eq!(
        format!("{}", err),
        "{\"val\":[\"`1234` must be in `0 <= value <= 1000`, but not.\"]}".to_string()
    );

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({"val": ["`1234` must be in `0 <= value <= 1000`, but not."]})
    );
}
