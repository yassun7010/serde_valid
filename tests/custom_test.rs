use serde_json::json;
use serde_valid::Validate;

#[test]
fn custom_meta_path_is_ok_test() {
    fn user_validation(_val: &Vec<i32>) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(user_validation))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn custom_meta_list_is_ok_test() {
    fn user_validation(
        _val1: &Vec<i32>,
        _val2: &i32,
        _lit1: f32,
        _val3: &f32,
        _lit2: bool,
    ) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(user_validation(val2, 1.234, val3, true)))]
        val1: Vec<i32>,
        val2: i32,
        val3: f32,
    }

    let s = TestStruct {
        val1: vec![1, 2, 3, 4],
        val2: 5,
        val3: 1.234,
    };
    assert!(s.validate().is_ok());
}

#[test]
fn custom_meta_list_when_meta_path_arg_is_ok_test() {
    fn user_validation(
        _val1: &Vec<i32>,
        _val2: &i32,
    ) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(user_validation(val2)))]
        val1: Vec<i32>,
        val2: i32,
    }

    let s = TestStruct {
        val1: vec![1, 2, 3, 4],
        val2: 5,
    };
    assert!(s.validate().is_ok());
}

#[test]
fn custom_meta_list_when_literal_arg_is_ok_test() {
    fn user_validation(_val1: &Vec<i32>, _val2: i32) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(user_validation(10)))]
        val1: Vec<i32>,
        val2: i32,
    }

    let s = TestStruct {
        val1: vec![1, 2, 3, 4],
        val2: 5,
    };
    assert!(s.validate().is_ok());
}

#[test]
fn custom_is_err_test() {
    fn user_validation(_val: &Vec<i32>) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "this is custom message.".to_string(),
        ))
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(custom(user_validation))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "this is custom message."
            ]
        }))
        .unwrap()
    );
}
