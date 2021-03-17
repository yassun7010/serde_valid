#[test]
fn validation_test() {
    use serde_valid::Validate;

    #[derive(Debug, Validate)]
    struct TestStruct<'a> {
        // Generic validator
        #[validate(enumerate(5, 10, 15))]
        // Numeric validator
        #[validate(multiple_of = 5)]
        #[validate(range(minimum = 5, maximum = 5))]
        int_value: i32,

        // Generic validator
        #[validate(enumerate(5.0, 10.0, 15.0))]
        // Numeric validator
        #[validate(multiple_of = 5.0)]
        #[validate(range(minimum = 5.0, maximum = 5.0))]
        float_value: f32,

        // Generic validator
        #[validate(enumerate("12345", "67890"))]
        // String validator
        #[validate(length(min_length = 5, max_length = 5))]
        #[validate(pattern = r"^\d{5}$")]
        string_value: String,

        // Generic validator
        #[validate(enumerate("12345", "67890"))]
        // String validator
        #[validate(length(min_length = 5, max_length = 5))]
        #[validate(pattern = r"^\d{5}$")]
        str_value: &'a str,

        // Generic validator
        #[validate(enumerate(5, 10, 15))]
        // Numeric validator
        #[validate(multiple_of = 5)]
        #[validate(range(minimum = 5, maximum = 5))]
        optional_value: Option<i32>,

        // Generic validator
        #[validate(enumerate(5, 10, 15))]
        // Array validator
        #[validate(unique_items)]
        #[validate(items(min_items = 3, max_items = 3))]
        // Numeric validator
        #[validate(multiple_of = 5)]
        #[validate(range(minimum = 5, maximum = 15))]
        vec_value: Vec<i32>,
    }

    let s = TestStruct {
        int_value: 5,
        float_value: 5.0,
        string_value: "12345".to_string(),
        str_value: "12345",
        optional_value: Some(5),
        vec_value: vec![5, 10, 15],
    };
    assert!(s.validate().is_ok());
}
