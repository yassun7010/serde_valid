use serde_json::json;
use serde_valid::Validate;

#[test]
fn nested_validate_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        val: TestInnerStruct,
    }

    #[derive(Validate)]
    struct TestInnerStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        inner_val: Vec<i32>,
    }

    let s = TestStruct {
        val: TestInnerStruct {
            inner_val: vec![1, 2, 3, 4],
        },
    };
    assert!(s.validate().is_ok());
}

#[test]
fn nested_validate_vec_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate]
        #[validate(items(min_items = 2, max_items = 2))]
        val: Vec<TestInnerStruct>,
    }

    #[derive(Debug, Validate)]
    struct TestInnerStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        inner_val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![
            TestInnerStruct {
                inner_val: vec![1, 2, 3, 4],
            },
            TestInnerStruct {
                inner_val: vec![5, 6, 7, 8],
            },
        ],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn nested_validate_option_type_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        val: Option<TestInnerStruct>,
    }

    #[derive(Validate)]
    struct TestInnerStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        inner_val: Vec<i32>,
    }

    let s = TestStruct {
        val: Some(TestInnerStruct {
            inner_val: vec![1, 2, 3, 4],
        }),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn nested_validate_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        val1: TestInnerStructNamedFields,
        #[validate]
        val2: TestInnerStructSingleUnnamedFields,
        #[validate]
        val3: TestInnerStructUnnamedFields,
        #[validate]
        val4: TestInnerEnumSingleUnnamedFields,
        #[validate]
        val5: TestInnerEnumUnnamedFields,
    }

    #[derive(Validate)]
    struct TestInnerStructNamedFields {
        #[validate(items(min_items = 4, max_items = 4))]
        inner_val: Vec<i32>,
    }

    #[derive(Validate)]
    struct TestInnerStructSingleUnnamedFields(#[validate(range(maximum = 0))] i32);

    #[derive(Validate)]
    struct TestInnerStructUnnamedFields(
        #[validate(range(maximum = 0))] i32,
        #[validate(range(maximum = 0))] i32,
    );

    #[derive(Validate)]
    enum TestInnerEnumSingleUnnamedFields {
        Value(#[validate(range(maximum = 0))] i32),
    }

    #[derive(Validate)]
    enum TestInnerEnumUnnamedFields {
        Value(
            #[validate(range(maximum = 0))] i32,
            #[validate(range(maximum = 0))] i32,
        ),
    }

    let s = TestStruct {
        val1: TestInnerStructNamedFields {
            inner_val: vec![1, 2, 3],
        },
        val2: TestInnerStructSingleUnnamedFields(5),
        val3: TestInnerStructUnnamedFields(5, 5),
        val4: TestInnerEnumSingleUnnamedFields::Value(5),
        val5: TestInnerEnumUnnamedFields::Value(5, 5),
    };

    assert_eq!(
        serde_json::to_value(&s.validate().unwrap_err()).unwrap(),
        json!({
            "val1": [{
                "inner_val": [
                    "items length of [1, 2, 3] must be in `4 <= length <= 4`, but `3`."
                ]
            }],
            "val2":["`5` must be in `value <= 0`, but not."],
            "val3":[{
                "0":[
                    "`5` must be in `value <= 0`, but not."
                ],
                "1":[
                    "`5` must be in `value <= 0`, but not."
                ]
            }],
            "val4":["`5` must be in `value <= 0`, but not."],
            "val5":[{
                "0":[
                    "`5` must be in `value <= 0`, but not."
                ],
                "1":[
                    "`5` must be in `value <= 0`, but not."
                ]
            }],
        })
    );
}
