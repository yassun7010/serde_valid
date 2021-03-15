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
