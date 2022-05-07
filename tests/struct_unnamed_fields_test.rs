use serde_valid::Validate;

#[test]
fn multiple_of_integer_is_ok() {
    #[derive(Validate)]
    struct TestStruct(#[validate(multiple_of = 5)] i32);

    let s = TestStruct(15);
    assert!(s.validate().is_ok());
}
