use serde_json::json;
use serde_valid::Validate;

#[test]
fn struct_unnamed_fields_newtype_is_ok() {
    #[derive(Validate)]
    struct TestStruct(#[validate(multiple_of = 5)] i32);

    let s = TestStruct(15);
    assert!(s.validate().is_ok());
}

#[test]
fn struct_unnamed_fields_newtype_is_err() {
    #[derive(Validate)]
    struct TestStruct(#[validate(multiple_of = 5)] i32);

    let s = TestStruct(13);
    let err = s.validate().unwrap_err();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&err.to_string()).unwrap(),
        json!({
            "errors": ["the value must be multiple of `5`."]
        })
    );
}

#[test]
fn struct_unnamed_fields_is_ok() {
    #[derive(Validate)]
    struct TestStruct(
        #[validate(multiple_of = 5)] i32,
        #[validate(min_length = 1)] &'static str,
    );

    let s = TestStruct(15, "ababa");
    assert!(s.validate().is_ok());
}

#[test]
fn struct_unnamed_fields_is_err() {
    #[derive(Validate)]
    struct TestStruct(
        #[validate(multiple_of = 5)] i32,
        #[validate(min_length = 8)] &'static str,
    );

    let s = TestStruct(15, "abcde");
    let err = s.validate().unwrap_err();

    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&err.to_string()).unwrap(),
        json!({
            "errors": [],
            "items": {
                "1": {
                    "errors": ["the length of the value must be `>= 8`."]
                }
            }
        })
    );
}
