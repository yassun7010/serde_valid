use serde_valid::Validate;

#[test]
fn empty_struct_with_braces_is_ok() {
    #[derive(Validate)]
    struct TestStruct {}

    let s = TestStruct {};
    assert!(s.validate().is_ok());
}
