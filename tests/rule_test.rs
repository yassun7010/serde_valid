use serde_json::json;
use serde_valid::Validate;

fn sample_ok_rule(_val: &i32) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}

fn sample_ok_rule2(_val1: &i32, _val2: &str) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}

fn sample_err_rule(_val1: &i32, _val2: &str) -> Result<(), serde_valid::validation::Error> {
    Err(serde_valid::validation::Error::Custom(
        "Rule error is added to the first arg of the rule_method.".to_owned(),
    ))
}

#[test]
fn rule_struct_named_field_is_ok() {
    #[derive(Validate)]
    #[rule(sample_ok_rule(val))]
    struct TestStruct {
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert!(s.validate().is_ok());
}

#[test]
fn rule_struct_named_fields_is_ok() {
    #[derive(Validate)]
    #[rule(sample_ok_rule2(val1, val2))]
    struct TestStruct {
        val1: i32,
        val2: String,
    }

    let s = TestStruct {
        val1: 5,
        val2: "val2".to_owned(),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn rule_struct_named_fields_is_err() {
    #[derive(Validate)]
    #[rule(sample_err_rule(val2, val1))]
    struct TestStruct {
        val1: String,
        val2: i32,
    }

    let s = TestStruct {
        val1: "val1".to_owned(),
        val2: 1,
    };
    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "val2": [
                "Rule error is added to the first arg of the rule_method."
            ]
        }))
        .unwrap()
    );
}

#[test]
fn rule_struct_unnamed_fields_is_ok() {
    #[derive(Validate)]
    #[rule(sample_ok_rule(0))]
    #[rule(sample_ok_rule2(0, 1))]
    struct TestStruct(i32, String);

    let s = TestStruct(0, "1".to_owned());
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
        #[rule(sample_ok_rule2(0, 1))]
        UnnamedFields(i32, String),
        #[rule(sample_ok_rule(0))]
        NewType(i32),
        NoField,
    }

    let s1 = TestEnum::NamedFields { val: 5 };
    assert!(s1.validate().is_ok());
    let s2 = TestEnum::UnnamedFields(0, "1".to_owned());
    assert!(s2.validate().is_ok());
    let s3 = TestEnum::NewType(5);
    assert!(s3.validate().is_ok());
    let s4 = TestEnum::NoField;
    assert!(s4.validate().is_ok());
}
