use serde_valid::Validate;

#[test]
fn enum_named_variant_validation_is_ok_test() {
    #[derive(Debug, Validate)]
    enum TestEnum {
        Named {
            a: i32,
            #[validate]
            b: TestStruct,
        },
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 10))]
        val: i32,
    }

    let s = TestEnum::Named {
        a: 12,
        b: TestStruct { val: 5 },
    };
    assert!(s.validate().is_ok());
}

#[test]
fn enum_unnamed_variant_validation_is_ok_test() {
    #[derive(Debug, Validate)]
    enum TestEnum {
        UnNamed(i32, #[validate] TestStruct),
    }

    #[derive(Debug, Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 10))]
        val: i32,
    }

    let s = TestEnum::UnNamed(12, TestStruct { val: 5 });
    assert!(s.validate().is_ok());
}
