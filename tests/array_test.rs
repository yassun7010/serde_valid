use serde_json::json;
use serde_valid::Validate;

#[test]
fn items_err_message() {
    fn rule_sample(_a: &i32) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "Rule error.".to_owned(),
        ))
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(min_items = 5)]
        #[validate(max_items = 2)]
        #[validate]
        val: Vec<TestChildStruct>,
    }

    #[derive(Validate)]
    #[rule(rule_sample(val))]
    struct TestChildStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct {
        val: vec![
            TestChildStruct { val: 0 },
            TestChildStruct { val: 5 },
            TestChildStruct { val: 15 },
        ],
    };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "errors": [],
            "properties": {
                "val": {
                    "errors": [
                        "the length of the items must be `>= 5`.",
                        "the length of the items must be `<= 2`.",
                    ],
                    "items": {
                        "0": {
                            "errors": ["Rule error."],
                            "properties": {
                                "val": {
                                    "errors": ["the number must be `>= 1`."]
                                }
                            }
                        }
                        ,"1": {
                            "errors": ["Rule error."],
                            "properties": {}
                        }
                        ,"2": {
                            "errors": ["Rule error."],
                            "properties": {
                                "val": {
                                    "errors": ["the number must be `<= 10`."]
                                }
                            }
                        }
                    }
                }
            }
        }))
        .unwrap()
    );
}
