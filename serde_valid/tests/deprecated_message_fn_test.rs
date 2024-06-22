#[allow(deprecated)]
mod test {

    use serde_json::json;
    use serde_valid::Validate;

    use serde::Deserialize;

    #[test]
    fn deprecated_enumerate_custom_err_message_fn() {
        fn error_message(_params: &serde_valid::error::EnumerateError) -> String {
            "this is custom message.".to_string()
        }

        #[derive(Validate)]
        struct TestStruct {
            #[validate(enumerate(1, 2, 3), message_fn(error_message))]
            val: i32,
        }

        let s = TestStruct { val: 4 };

        assert_eq!(
            s.validate().unwrap_err().to_string(),
            json!({
                "errors": [],
                "properties": {
                    "val": {
                        "errors": [
                        "this is custom message."
                        ]
                    }
                }
            })
            .to_string()
        );
    }

    #[test]
    fn enumerate_custom_err_message_fn() {
        fn error_message(_params: &serde_valid::error::EnumerateError) -> String {
            "this is custom message.".to_string()
        }

        #[derive(Validate)]
        struct TestStruct {
            #[validate(enumerate = [1, 2, 3], message_fn(error_message))]
            val: i32,
        }

        let s = TestStruct { val: 4 };

        assert_eq!(
            s.validate().unwrap_err().to_string(),
            json!({
                "errors": [],
                "properties": {
                    "val": {
                        "errors": [
                        "this is custom message."
                        ]
                    }
                }
            })
            .to_string()
        );
    }

    #[test]
    fn items_custom_err_message_fn() {
        #[inline]
        fn min_error_message(_params: &serde_valid::MinItemsError) -> String {
            "this is min custom message.".to_string()
        }
        #[inline]
        fn max_error_message(_params: &serde_valid::MaxItemsError) -> String {
            "this is max custom message.".to_string()
        }

        #[derive(Validate)]
        struct TestStruct {
            #[validate(min_items = 4, message_fn(min_error_message))]
            #[validate(max_items = 2, message_fn(max_error_message))]
            val: Vec<i32>,
        }

        let s = TestStruct { val: vec![1, 2, 3] };

        assert_eq!(
            s.validate().unwrap_err().to_string(),
            json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "this is min custom message.",
                        "this is max custom message."
                    ]
                }
            }
            })
            .to_string()
        );
    }

    #[test]
    fn length_custom_err_message_fn() {
        fn custom_min_error_message(_params: &serde_valid::MinLengthError) -> String {
            "this is min custom message.".to_string()
        }

        fn custom_max_error_message(_params: &serde_valid::MaxLengthError) -> String {
            "this is max custom message.".to_string()
        }

        #[derive(Validate)]
        struct TestStruct {
            #[validate(min_length = 5, message_fn(custom_min_error_message))]
            #[validate(max_length = 3, message_fn(custom_max_error_message))]
            val: String,
        }

        let s = TestStruct {
            val: String::from("test"),
        };

        assert_eq!(
            s.validate().unwrap_err().to_string(),
            json!({
                "errors": [],
                "properties": {
                    "val": {
                        "errors": [
                            "this is min custom message.",
                            "this is max custom message."
                        ]
                    }
                }
            })
            .to_string()
        );
    }

    #[test]
    fn multiple_of_custom_err_message_fn() {
        fn error_message(_params: &serde_valid::MultipleOfError) -> String {
            "this is custom message.".to_string()
        }

        #[derive(Validate)]
        struct TestStruct {
            #[validate(multiple_of = 5, message_fn(error_message))]
            val: i32,
        }

        let s = TestStruct { val: 14 };

        assert_eq!(
            s.validate().unwrap_err().to_string(),
            json!({
                "errors": [],
                "properties": {
                    "val": {
                        "errors": [
                            "this is custom message."
                        ]
                    }
                }
            })
            .to_string()
        );
    }

    #[test]
    fn pattern_custom_err_message_fn() {
        fn error_message(_params: &serde_valid::PatternError) -> String {
            "this is custom message.".to_string()
        }

        #[derive(Validate)]
        struct TestStruct {
            #[validate(pattern = r"^\d{4}-\d{2}-\d{2}$", message_fn = error_message)]
            val: String,
        }

        let s = TestStruct {
            val: String::from("2020/09/10"),
        };

        assert_eq!(
            s.validate().unwrap_err().to_string(),
            json!({
                "errors": [],
                "properties": {
                    "val": {
                        "errors": [
                            "this is custom message."
                        ]
                    }
                }
            })
            .to_string()
        );
    }

    #[test]
    fn properties_custom_err_message_fn() {
        fn min_custom_error_message(_params: &serde_valid::MinPropertiesError) -> String {
            "this is min custom message.".to_string()
        }

        fn max_custom_error_message(_params: &serde_valid::MaxPropertiesError) -> String {
            "this is max custom message.".to_string()
        }

        #[derive(Deserialize, Validate)]
        struct TestStruct {
            #[validate(min_properties = 3, message_fn(min_custom_error_message))]
            #[validate(max_properties = 1, message_fn(max_custom_error_message))]
            val: serde_json::Map<String, serde_json::Value>,
        }

        let s: TestStruct = serde_json::from_value(json!({
            "val": {
                "key1": "value1",
                "key2": "value2",
            }
        }))
        .unwrap();

        assert_eq!(
            s.validate().unwrap_err().to_string(),
            json!({
                "errors": [],
                "properties": {
                    "val": {
                        "errors": [
                            "this is min custom message.",
                            "this is max custom message."
                        ]
                    }
                }
            })
            .to_string()
        );
    }

    #[test]
    fn range_custom_err_message_fn() {
        fn custom_min_error_message(_params: &serde_valid::MinimumError) -> String {
            "this is min custom message.".to_string()
        }

        fn custom_max_error_message(_params: &serde_valid::MaximumError) -> String {
            "this is max custom message.".to_string()
        }

        #[derive(Validate)]
        struct TestStruct {
            #[validate(minimum = 5, message_fn(custom_min_error_message))]
            #[validate(maximum = 3, message_fn(custom_max_error_message))]
            val: i32,
        }

        let s = TestStruct { val: 4 };

        assert_eq!(
            s.validate().unwrap_err().to_string(),
            json!({
                "errors": [],
                "properties": {
                    "val": {
                        "errors": [
                            "this is min custom message.",
                            "this is max custom message."
                        ]
                    }
                }
            })
            .to_string()
        );
    }

    #[test]
    fn unique_items_custom_err_message_fn() {
        fn error_message(_params: &serde_valid::UniqueItemsError) -> String {
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
            s.validate().unwrap_err().to_string(),
            json!({
                "errors": [],
                "properties": {
                    "val": {
                        "errors": [
                            "this is custom message."
                        ]
                    }
                }
            })
            .to_string()
        );
    }
}
