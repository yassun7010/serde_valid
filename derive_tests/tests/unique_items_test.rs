use serde_valid::Validate;

#[test]
fn unique_items_vec_type_test() {
    #[derive(Debug, Validate)]
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
fn unique_items_slice_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: [i32; 4],
    }

    let s = TestStruct { val: [1, 2, 3, 4] };
    assert!(s.validate().is_ok());
}

#[test]
fn unique_items_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 2] };
    assert!(s.validate().is_err());
}

#[test]
fn unique_items_err_message_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(unique_items)]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 2],
    };

    let mut results = s.validate().unwrap_err().into_iter();
    let (field, errors) = results.next().unwrap();

    assert!(results.next().is_none());
    assert_eq!(field, "val");

    let mut errors = errors.iter();

    assert_eq!(
        format!("{}", errors.next().unwrap()),
        "item of [1, 2, 3, 2] must be unique, but not."
    );
    assert!(errors.next().is_none());
}
