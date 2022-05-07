use serde_valid::Validate;

fn sample_rule(_val: &i32) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}

#[test]
fn rule_struct_named_fields_is_ok() {
    #[derive(Validate)]
    #[rule(sample_rule(val))]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert!(s.validate().is_ok());
}

#[test]
fn rule_struct_unnamed_fields_is_ok() {
    #[derive(Validate)]
    #[rule(sample_rule(0))]
    struct TestStruct(
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        i32,
    );

    let s = TestStruct(5);
    assert!(s.validate().is_ok());
}
