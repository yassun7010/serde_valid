use serde_json::json;
use serde_valid::Validate;

#[test]
fn nested_validate_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        val: TestInnerStruct,
    }

    #[derive(Validate)]
    struct TestInnerStruct {
        #[validate(min_items = 4)]
        #[validate(max_items = 4)]
        inner_val: Vec<i32>,
    }

    let s = TestStruct {
        val: TestInnerStruct {
            inner_val: vec![1, 2, 3, 4],
        },
    };
    assert!(s.validate().is_ok());
}

#[test]
fn nested_validate_vec_type_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate]
        #[validate(min_items = 2)]
        #[validate(max_items = 2)]
        val: Vec<TestInnerStruct>,
    }

    #[derive(Debug, Validate)]
    struct TestInnerStruct {
        #[validate(min_items = 4)]
        #[validate(max_items = 4)]
        inner_val: Vec<i32>,
    }

    let s = TestStruct {
        val: vec![
            TestInnerStruct {
                inner_val: vec![1, 2, 3, 4],
            },
            TestInnerStruct {
                inner_val: vec![5, 6, 7, 8],
            },
        ],
    };
    assert!(s.validate().is_ok());
}

#[test]
fn nested_validate_option_type_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        val: Option<TestInnerStruct>,
    }

    #[derive(Validate)]
    struct TestInnerStruct {
        #[validate(min_items = 4)]
        #[validate(max_items = 4)]
        inner_val: Vec<i32>,
    }

    let s = TestStruct {
        val: Some(TestInnerStruct {
            inner_val: vec![1, 2, 3, 4],
        }),
    };
    assert!(s.validate().is_ok());
}

#[test]
fn nested_validate_err_message_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate]
        named_fields_struct: StructNamedFields,
        #[validate]
        unnamed_fields_struct: StructUnnamedFields,
        #[validate]
        single_unnamed_fields_struct: StructSingleUnnamedFields,
        #[validate]
        named_fields_enum: EnumNamedFields,
        #[validate]
        unnamed_fields_enum: EnumUnnamedFields,
        #[validate]
        single_unnamed_fields_enum: EnumSingleUnnamedFields,
    }

    #[derive(Validate)]
    struct StructNamedFields {
        #[validate(range(maximum = 0))]
        val: i32,
    }

    #[derive(Validate)]
    struct StructSingleUnnamedFields(#[validate(range(maximum = 0))] i32);

    #[derive(Validate)]
    struct StructUnnamedFields(
        #[validate(range(maximum = 0))] i32,
        #[validate(range(maximum = 0))] i32,
    );

    #[derive(Validate)]
    enum EnumNamedFields {
        Value {
            #[validate(range(maximum = 0))]
            val: i32,
        },
    }

    #[derive(Validate)]
    enum EnumSingleUnnamedFields {
        Value(#[validate(range(maximum = 0))] i32),
    }

    #[derive(Validate)]
    enum EnumUnnamedFields {
        Value(
            #[validate(range(maximum = 0))] i32,
            #[validate(range(maximum = 0))] i32,
        ),
    }

    let s = TestStruct {
        named_fields_struct: StructNamedFields { val: 5 },
        unnamed_fields_struct: StructUnnamedFields(5, 5),
        single_unnamed_fields_struct: StructSingleUnnamedFields(5),
        named_fields_enum: EnumNamedFields::Value { val: 5 },
        single_unnamed_fields_enum: EnumSingleUnnamedFields::Value(5),
        unnamed_fields_enum: EnumUnnamedFields::Value(5, 5),
    };

    assert_eq!(
        serde_json::to_value(&s.validate().unwrap_err()).unwrap(),
        json!({
            "named_fields_struct": [{
                "val": [
                    "`5` must be in `value <= 0`, but not."
                ]
            }],
            "unnamed_fields_struct": [{
                "0": [
                    "`5` must be in `value <= 0`, but not."
                ],
                "1": [
                    "`5` must be in `value <= 0`, but not."
                ]
            }],
            "single_unnamed_fields_struct": ["`5` must be in `value <= 0`, but not."],
            "named_fields_enum": [{
                "val": [
                    "`5` must be in `value <= 0`, but not."
                ]
            }],
            "unnamed_fields_enum": [{
                "0": [
                    "`5` must be in `value <= 0`, but not."
                ],
                "1": [
                    "`5` must be in `value <= 0`, but not."
                ]
            }],
            "single_unnamed_fields_enum": ["`5` must be in `value <= 0`, but not."],
        })
    );
}
