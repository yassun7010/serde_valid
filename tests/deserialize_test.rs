use serde::Deserialize;
use serde_json::json;
use serde_valid::json::FromJsonValue;
use serde_valid::Validate;

#[test]
fn json_error_to_string() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 1000)]
        val: i32,
    }

    let err = TestStruct::from_json_value(json!({ "val": 1234 })).unwrap_err();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&err.to_string()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": ["the number must be `<= 1000`."]
                }
            }
        })
    );
}

#[test]
fn json_error_as_validation_errors() {
    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 1000)]
        val: i32,
    }

    let err = TestStruct::from_json_value(json!({ "val": 1234 })).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": ["the number must be `<= 1000`."]
                }
            }
        })
    );
}

#[cfg(feature = "yaml")]
#[test]
fn yaml_error_as_validation_errors() {
    use serde::Deserialize;
    use serde_valid::yaml::FromYamlValue;
    use serde_valid::Validate;

    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(maximum = 10)]
        val: i32,
    }

    let err = TestStruct::from_yaml_value(serde_yaml::from_str("val: 15").unwrap()).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": ["the number must be `<= 10`."]
                }
            }
        })
    );
}

#[cfg(feature = "toml")]
#[test]
fn toml_error_as_validation_errors() {
    use serde::Deserialize;
    use serde_valid::toml::FromTomlValue;
    use serde_valid::Validate;

    #[derive(Debug, Validate, Deserialize)]
    struct TestStruct {
        #[validate(maximum = 10)]
        val: i32,
    }

    let err = TestStruct::from_toml_value(serde_toml::from_str("val = 15").unwrap()).unwrap_err();

    assert_eq!(
        serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
        json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": ["the number must be `<= 10`."]
                }
            }
        })
    );
}
