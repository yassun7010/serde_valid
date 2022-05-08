use serde_json::json;
use serde_valid::{
    Validate, ValidateNumericExclusiveMaximum, ValidateNumericExclusiveMinimum,
    ValidateNumericMaximum, ValidateNumericMinimum,
};

#[test]
fn range_integer() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 2000)]
        val: i32,
    }

    let s = TestStruct { val: 1234 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_float() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0.0)]
        #[validate(maximum = 2000.0)]
        val: f32,
    }

    let s = TestStruct { val: 1234.5678 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 0)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_minimum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_minimum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_err());
}

#[test]
fn range_exclusive_minimum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 1 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_minimum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };
    assert!(s.validate().is_err());
}

#[test]
fn range_maximum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 10 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_maximum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 11 };
    assert!(s.validate().is_err());
}

#[test]
fn range_exclusive_maximum_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_maximum = 10)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 9 };
    assert!(s.validate().is_ok());
}

#[test]
fn range_exclusive_maximum_is_err() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_maximum = 10)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 10 };
    assert!(s.validate().is_err());
}

#[test]
fn range_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 20)]
        val: Vec<i32>,
    }

    let s = TestStruct { val: vec![12, 16] };
    assert!(s.validate().is_ok());
}

#[test]
fn range_nested_vec_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 20)]
        val: Vec<Vec<i32>>,
    }

    let s = TestStruct {
        val: vec![vec![4, 8], vec![12, 16]],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn range_option_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: Option<i32>,
    }

    let s = TestStruct { val: Some(5) };
    assert!(s.validate().is_ok());
}

#[test]
fn range_nested_option_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: Option<Option<i32>>,
    }

    let s = TestStruct { val: Some(Some(5)) };
    assert!(s.validate().is_ok());
}

#[test]
fn range_vec_optional_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: Vec<Option<i32>>,
    }

    let s = TestStruct {
        val: vec![Some(4), Some(8), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn range_array_optional_type_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: [Option<i32>; 3],
    }

    let s = TestStruct {
        val: [Some(4), Some(8), None],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn range_inclusive_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 1)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "the number must be `>= 1`."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn range_exclusive_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(exclusive_minimum = 1)]
        #[validate(exclusive_maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 0 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "the number must be `> 1`."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn range_custom_err_message_fn() {
    fn custom_min_error_message(_params: &serde_valid::MinimumErrorParams) -> String {
        "this is min custom message.".to_string()
    }

    fn custom_max_error_message(_params: &serde_valid::MaximumErrorParams) -> String {
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
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is min custom message.",
                "this is max custom message."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn range_custom_err_message() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 5, message = "this is min custom message.")]
        #[validate(maximum = 3, message = "this is max custom message.")]
        val: i32,
    }

    let s = TestStruct { val: 4 };

    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is min custom message.",
                "this is max custom message."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn range_trait() {
    struct MyType(i32);

    impl ValidateNumericMinimum<i32> for MyType {
        fn validate(&self, minimum: i32) -> Result<(), serde_valid::MinimumErrorParams> {
            ValidateNumericMinimum::validate(&self.0, minimum)
        }
    }

    impl ValidateNumericMaximum<i32> for MyType {
        fn validate(&self, maximum: i32) -> Result<(), serde_valid::MaximumErrorParams> {
            ValidateNumericMaximum::validate(&self.0, maximum)
        }
    }

    impl ValidateNumericExclusiveMinimum<i32> for MyType {
        fn validate(
            &self,
            exclusive_minimum: i32,
        ) -> Result<(), serde_valid::ExclusiveMinimumErrorParams> {
            ValidateNumericExclusiveMinimum::validate(&self.0, exclusive_minimum)
        }
    }

    impl ValidateNumericExclusiveMaximum<i32> for MyType {
        fn validate(
            &self,
            exclusive_maximum: i32,
        ) -> Result<(), serde_valid::ExclusiveMaximumErrorParams> {
            ValidateNumericExclusiveMaximum::validate(&self.0, exclusive_maximum)
        }
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 3)]
        #[validate(maximum = 5)]
        val: MyType,
    }

    let s = TestStruct { val: MyType(4) };

    assert!(s.validate().is_ok());
}
