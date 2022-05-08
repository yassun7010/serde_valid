use serde_json::json;
use serde_valid::Validate;

fn sample_ok_rule(_val: &i32) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}

fn sample_err_rule(_val: &i32) -> Result<(), serde_valid::validation::Error> {
    Err(serde_valid::validation::Error::Custom(
        "rule error.".to_owned(),
    ))
}

#[test]
fn rule_struct_named_fields_is_ok() {
    #[derive(Validate)]
    #[rule(sample_ok_rule(val))]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert!(s.validate().is_ok());
}

#[test]
fn rule_struct_named_fields_is_err() {
    #[derive(Validate)]
    #[rule(sample_err_rule(val))]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val": [
                "rule error."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn rule_struct_unnamed_fields_is_ok() {
    #[derive(Validate)]
    #[rule(sample_ok_rule(0))]
    struct TestStruct(i32, i32);

    let s = TestStruct(5, 5);
    assert!(s.validate().is_ok());
}

#[test]
fn rule_new_type_is_ok() {
    #[derive(Validate)]
    #[rule(sample_ok_rule(0))]
    struct TestNewType(i32);

    let s = TestNewType(5);
    assert!(s.validate().is_ok());
}

#[test]
fn rule_enum_is_ok() {
    #[derive(Validate)]
    enum TestEnum {
        #[rule(sample_ok_rule(val))]
        NamedFields {
            val: i32,
        },
        #[rule(sample_ok_rule(0))]
        UnnamedFields(i32, i32),
        #[rule(sample_ok_rule(0))]
        NewType(i32),
        NoField,
    }

    let s1 = TestEnum::NamedFields { val: 5 };
    assert!(s1.validate().is_ok());
    let s2 = TestEnum::UnnamedFields(5, 5);
    assert!(s2.validate().is_ok());
    let s3 = TestEnum::NewType(5);
    assert!(s3.validate().is_ok());
    let s4 = TestEnum::NoField;
    assert!(s4.validate().is_ok());
}
