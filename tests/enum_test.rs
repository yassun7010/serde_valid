use serde_json::json;
use serde_valid::Validate;

#[test]
fn enum_named_variant_validation_is_ok() {
    fn ok_rule(_a: &TestStruct, _b: &TestStruct) -> Result<(), serde_valid::validation::Error> {
        Ok(())
    }
    #[derive(Validate)]
    enum TestEnum {
        #[rule(ok_rule(a, b))]
        Named {
            #[validate]
            a: TestStruct,
            #[validate]
            b: TestStruct,
        },
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestEnum::Named {
        a: TestStruct { val: 12 },
        b: TestStruct { val: 12 },
    };
    assert_eq!(
        serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
        serde_json::to_string(&json!({
            "errors": [],
            "properties": {
                "a": {
                    "errors": [
                        {
                            "errors": [],
                            "properties": {
                                "val": {
                                    "errors": ["the number must be `<= 10`."]
                                }
                            }
                        }
                    ]
                },
                "b": {
                    "errors": [
                        {
                            "errors": [],
                            "properties": {
                                "val": {
                                    "errors": ["the number must be `<= 10`."]
                                }
                            }
                        }
                    ]
                }
            }
        }))
        .unwrap()
    );
}

#[test]
fn enum_unnamed_variant_validation_is_ok() {
    #[derive(Validate)]
    enum TestEnum {
        UnNamed(
            #[validate(minimum = 0)]
            #[validate(maximum = 10)]
            i32,
            #[validate] TestStruct,
        ),
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestEnum::UnNamed(5, TestStruct { val: 5 });
    assert!(s.validate().is_ok());
}

#[test]
fn enum_newtype_variant_validation_is_ok() {
    #[derive(Validate)]
    enum TestEnum {
        NewType(
            #[validate(minimum = 0)]
            #[validate(maximum = 10)]
            i32,
        ),
    }

    let s = TestEnum::NewType(15);
    assert!(s.validate().is_err());
}
