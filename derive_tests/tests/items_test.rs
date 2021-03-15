use serde_valid::Validate;

#[test]
fn items_vec_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_array_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        val: [i32; 4],
    }

    let s = TestStruct { val: [1, 2, 3, 4] };
    assert!(s.validate().is_ok());
}

#[test]
fn items_min_items_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 3, max_items = 5))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_ok());
}

#[test]
fn items_min_items_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 5))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_err());
}

#[test]
fn items_max_items_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 0, max_items = 4))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_ok());
}

#[test]
fn items_max_items_is_err_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 1, max_items = 2))]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![1, 2, 3] };
    assert!(s.validate().is_err());
}

#[test]
fn items_vec_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 2, max_items = 2))]
        val: Vec<Vec<i32>>,
    }

    let s = TestStruct {
        val: vec![vec![], vec![1, 2, 3]],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_option_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 4, max_items = 4))]
        val: Option<Vec<i32>>,
    }

    let s = TestStruct {
        val: Some(vec![1, 2, 3, 4]),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_nested_option_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 3, max_items = 3))]
        val: Option<Option<Vec<i32>>>,
    }

    let s = TestStruct {
        val: Some(Some(vec![1, 2, 3])),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn items_vec_optional_type_is_ok_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(items(min_items = 3, max_items = 3))]
        val: Vec<Option<Vec<i32>>>,
    }

    let s = TestStruct {
        val: vec![Some(vec![1, 2, 3, 4]), Some(vec![1, 2, 3, 4]), None],
    };
    assert!(s.validate().is_ok());
}
