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
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![
            TestInnerStruct {
                val: vec![1, 2, 3, 4],
            },
            TestInnerStruct {
                val: vec![5, 6, 7, 8],
            },
        ],
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

    let mut results = s.validate().unwrap_err().into_iter();
    let (field, errors) = results.next().unwrap();

    assert!(results.next().is_none());
    assert_eq!(field, "val");

    let mut errors = errors.iter();

    assert_eq!(
        format!("{}", errors.next().unwrap()),
        "items length of [1, 2, 3] must be in `4 <= length <= 4`, but `3`."
    );
    assert!(errors.next().is_none());
}
