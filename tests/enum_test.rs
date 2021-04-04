use serde_valid::Validate;

#[test]
fn custom_meta_path_is_ok_test() {
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
