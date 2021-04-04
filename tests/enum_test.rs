use serde_valid::Validate;

#[test]
fn enum_named_variant_validation_is_ok_test() {
    #[derive(Validate)]
    enum TestEnum {
        Named {
            #[validate]
            a: TestStruct,
        },
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 10))]
        val: i32,
    }

    let s = TestEnum::Named {
        a: TestStruct { val: 5 },
    };
    assert!(s.validate().is_ok());
}

#[test]
fn enum_unnamed_variant_validation_is_ok_test() {
    #[derive(Validate)]
    enum TestEnum {
        UnNamed(i32, #[validate] TestStruct),
    }

    #[derive(Validate)]
    struct TestStruct {
        #[validate(range(minimum = 0, maximum = 10))]
        val: i32,
    }

    let s = TestEnum::UnNamed(12, TestStruct { val: 5 });
    assert!(s.validate().is_ok());
}
