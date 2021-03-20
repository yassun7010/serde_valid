use serde_valid::Validate;

#[test]
fn nested_validate_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate]
        val: TestInnerStruct,
    }

    #[derive(Debug, Validate)]
    struct TestInnerStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: TestInnerStruct {
            val: vec![1, 2, 3, 4],
        },
    };
    assert!(s.validate().is_ok());
}

#[test]
fn nested_validate_err_message_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate]
        val: TestInnerStruct,
    }

    #[derive(Debug, Validate)]
    struct TestInnerStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: TestInnerStruct { val: vec![1, 2, 3] },
    };
    for (field, error) in s.validate().unwrap_err() {
        assert_eq!(field, "val");
        assert_eq!(
            format!("{}", error),
            "items length of [1, 2, 3] must be in `4 <= length <= 4`, but `3`."
        )
    }
}
