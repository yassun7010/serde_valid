use serde::Deserialize;
use serde_json::json;
use serde_valid::json::FromJsonValue;
use serde_valid::Validate;

#[test]
fn serde_rename_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 100)]
        #[serde(rename = "value")]
        val: i32,
    }

    let s = TestStruct::from_json_value(json!({ "value": 123 }));

    assert!(s.is_ok())
}

#[test]
fn serde_rename_is_err() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(maximum = 100)]
        #[serde(rename = "value")]
        val: i32,
    }

    let err = TestStruct::from_json_value(json!({ "value": 123 })).unwrap_err();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&err.to_string()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "value": {
                    "errors": [
                        "the number must be `<= 100`."
                    ]
                }
            }
        })
    );
}

#[test]
fn serde_rename_deserialize_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 100)]
        #[serde(rename(deserialize = "value"))]
        val: i32,
    }

    let s = TestStruct::from_json_value(json!({ "value": 123 }));

    assert!(s.is_ok())
}

#[test]
fn serde_rename_deserialize_is_err() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(maximum = 100)]
        #[serde(rename(deserialize = "value"))]
        val: i32,
    }

    let err = TestStruct::from_json_value(json!({ "value": 123 })).unwrap_err();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&err.to_string()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "value": {
                    "errors": ["the number must be `<= 100`."]
                }
            }
        })
    );
}

#[test]
fn serde_rename_enume_is_ok() {
    #[derive(Debug, Validate, Deserialize)]
    enum TestEnum {
        Struct {
            #[validate(minimum = 100)]
            #[serde(rename = "value")]
            val: i32,
        },
    }

    let s = TestEnum::from_json_value(json!({ "Struct": { "value": 123 } }));

    assert!(s.is_ok())
}

#[test]
fn serde_rename_enume_is_err() {
    #[derive(Debug, Validate, Deserialize)]
    enum TestEnum {
        Struct {
            #[validate(maximum = 100)]
            #[serde(rename = "value")]
            val: i32,
        },
    }

    let err = TestEnum::from_json_value(json!({ "Struct": { "value": 123 } })).unwrap_err();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&err.to_string()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "value": {
                    "errors": ["the number must be `<= 100`."]
                }
            }
        })
    );
}
