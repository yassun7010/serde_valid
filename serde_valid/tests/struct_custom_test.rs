use serde_valid::Validate;

#[test]
fn named_struct_custom_is_ok() {
    fn sample_struct_validation(_val: &TestStruct) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom(sample_struct_validation))]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert_eq!(s.val, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn named_struct_custom_closure_is_ok() {
    fn sample_struct_validation(_val: i32) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom(|s| sample_struct_validation(s.val)))]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert_eq!(s.val, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn unnamed_struct_custom_is_ok() {
    fn sample_struct_validation(_val: &TestStruct) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom(sample_struct_validation))]
    struct TestStruct(i32);

    let s = TestStruct(5);
    assert_eq!(s.0, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn unnamed_struct_custom_closure_is_ok() {
    fn sample_struct_validation(_val: i32) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    #[derive(Validate)]
    #[validate(custom(|s| sample_struct_validation(s.0)))]
    struct TestStruct(i32);

    let s = TestStruct(5);
    assert_eq!(s.0, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn unnamed_struct_custom_closure_is_err() {
    fn sample_struct_validation(_val: i32) -> Result<(), serde_valid::validation::Error> {
        Err(serde_valid::validation::Error::Custom(
            "Struct Validation Error.".to_owned(),
        ))
    }

    #[derive(Validate)]
    #[validate(custom(|s| sample_struct_validation(s.0)))]
    struct TestStruct(i32);

    let s = TestStruct(5);
    assert_eq!(s.0, 5);
    assert!(s.validate().is_err());
}
