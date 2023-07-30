use serde_valid::Validate;

#[test]
fn test_raw_type_field() {
    #[derive(Validate)]
    // `"r#type"` is not a valid identifier.
    // It's proc_macro2 specification.
    // To avoid this, indicate `type`.
    #[rule(sample_rule(type))]
    struct MyStruct {
        #[validate(maximum = 10)]
        pub r#type: i32,
    }

    fn sample_rule(_type: &i32) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }

    let my_struct = MyStruct { r#type: 1 };
    assert!(my_struct.validate().is_ok());
}
