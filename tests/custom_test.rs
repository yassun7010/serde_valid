use serde_json::json;
use serde_valid::Validate;

mod parenthesized {
    pub fn meta_path_validation(_val: &[i32]) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }
}

#[test]
fn custom_validation_is_ok() {
    fn user_validation(_val: &[i32]) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
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
fn custom_parenthesized_path_validation_is_ok() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom(parenthesized::meta_path_validation))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn custom_validation_error() {
    fn user_validation(_val: &[i32]) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "this is custom message.".to_string(),
        ))
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(custom(user_validation))]
        val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![1, 2, 3, 4],
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
