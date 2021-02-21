use serde_valid::Validate;

#[test]
fn range_test() {
    #[derive(Debug, Validate)]
    struct TestStruct {
        val: f32,
    }

    let s = TestStruct { val: 1234.5678 };
    assert!(s.validate().is_ok());
}
