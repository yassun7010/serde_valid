use serde_valid::Validate;

#[test]
fn rule_is_ok_test() {
    #[derive(Validate)]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }

    let s = TestStruct { val: 5 };
    assert!(s.validate().is_ok());
}
