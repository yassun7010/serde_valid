# Serde Valid

This is [JSON Schema](https://json-schema.org/) based validation tool using with [serde](https://github.com/serde-rs/serde).

## Usage

You derive `Validate` trait, and write validations.

```rust
#[derive(serde_valid::Validate)]
struct SampleStruct {
    #[validate(minimum = 0)]
    #[validate(maximum = 10)]
    val: i32,
}

#[derive(serde_valid::Validate)]
enum SampleEnum {
    Named {
        #[validate]
        a: SampleStruct,
    },
    UnNamed(
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        i32,
    ),
}

let s = SampleEnum::Named {
    a: SampleStruct { val: 5 },
};

assert!(s.validate().is_ok());
```

## Validations

Serde Valid support standard validation based JSON Schema.

| Type | Serde Valid(validate derive)  | Serde Valid (validate trait) | Json Schema |
| :---: | :--- |:--- | :--- |
| String | `#[validate(max_length = 5)]` | `ValidateMaxLength` | [maxLength](https://json-schema.org/understanding-json-schema/reference/string.html#length) |
| String | `#[validate(min_length = 5)]` | `ValidateMinLength` | [minLength](https://json-schema.org/understanding-json-schema/reference/string.html#length) |
| String | `#[validate(pattern = r"^\d{5}$")]` | `ValidatePattern` | [pattern](https://json-schema.org/understanding-json-schema/reference/string.html#regular-expressions) |
| Numeric | `#[validate(maximum = 5)]` | `ValidateExclusiveMaximum` | [maximum](https://json-schema.org/understanding-json-schema/reference/numeric.html#range) |
| Numeric | `#[validate(minimum = 5)]` | `ValidateMinimum` | [minimum](https://json-schema.org/understanding-json-schema/reference/numeric.html#range) |
| Numeric | `#[validate(exclusive_maximum = 5)]` | `ValidateExclusiveMaximum` | [exclusiveMaximum](https://json-schema.org/understanding-json-schema/reference/numeric.html#range) |
| Numeric | `#[validate(exclusive_minimum = 5)]` | `ValidateExclusiveMinimum` | [exclusiveMinimum](https://json-schema.org/understanding-json-schema/reference/numeric.html#range) |
| Numeric | `#[validate(multiple_of = 5)]` | `ValidateMultipleOf` | [multipleOf](https://json-schema.org/understanding-json-schema/reference/numeric.html#multiples) |
| Object | `#[validate(max_properties = 5)]` | `ValidateMaxProperties` | [maxProperties](https://json-schema.org/understanding-json-schema/reference/object.html#size) |
| Object | `#[validate(min_properties = 5)]` | `ValidateMinProperties` | [minProperties](https://json-schema.org/understanding-json-schema/reference/object.html#size) |
| Array | `#[validate(max_items = 5)]` | `ValidateMaxItems` | [maxItems](https://json-schema.org/understanding-json-schema/reference/array.html#length) |
| Array | `#[validate(min_items = 5)]` | `ValidateMinItems` | [minItems](https://json-schema.org/understanding-json-schema/reference/array.html#length) |
| Array | `#[validate(unique_items)]` | `ValidateUniqueItems` | [uniqueItems](https://json-schema.org/understanding-json-schema/reference/array.html#unique_items) |
| Generic | `#[validate(enumerate(5, 10, 15))]` | `ValidateEnumerate` | [enum](https://json-schema.org/understanding-json-schema/reference/generic.html#enumerated-values) |

## Complete Constructor

Serde Valid support complete constructor method using by `serde_valid::from_value`/ `serde_valid::from_str` / `serde_valid::from_slice` / `serde_valid::from_reader`.

```rust
#[derive(Debug, serde_valid::Validate, serde_valid::Deserialize)]
struct SampleStruct {
    #[validate(minimum = 0)]
    #[validate(maximum = 1000)]
    val: i32,
}

// Deserializing and Validation!! 🚀
let err = serde_valid::from_value::<SampleStruct, _>(json!({ "val": 1234 })).unwrap_err();

assert_eq!(
    serde_json::to_value(err.as_validation_errors().unwrap()).unwrap(),
    json!({"val": ["the number must be `<= 1000`."]})
);
```

You can force validation by only deserializing through `serde_valid`, and removing `serde_json` from `Cargo.toml` of your project.


## Custom Message

For user custom message, Serde Valid provides `message_fn` or `message`.

```rust
fn min_error_message(_params: &serde_valid::MinItemsErrorParams) -> String {
    "this is min custom message_fn.".to_string()
}

#[derive(serde_valid::Validate)]
struct SampleStruct {
    #[validate(min_items = 4, message_fn(min_error_message))]
    #[validate(max_items = 2, message = "this is max custom message.")]
    val: Vec<i32>,
}

let s = SampleStruct { val: vec![1, 2, 3] };

assert_eq!(
    serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
    serde_json::to_string(&json!({
        "val": [
            "this is min custom message_fn.",
            "this is max custom message."
        ]
    }))
    .unwrap()
);
```

## Custom method

You can use your custom validation using by `#[validate(custom)]` validation.

```rust
fn user_validation(_val: &Vec<i32>) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}

#[derive(serde_valid::Validate)]
struct SampleStruct {
    #[validate(custom(user_validation))]
    val: i32,
}

let s = SampleStruct { val: 1 };

assert!(s.validate().is_ok());
```

## Rules

If you want to check multi fields validation, you can use `#[rule]`.

```rust
fn sample_rule(_val1: &i32, _val2: &str) -> Result<(), serde_valid::validation::Error> {
    Err(serde_valid::validation::Error::Custom(
        "Rule error is added to the first arg of the rule_method.".to_owned(),
    ))
}

#[derive(serde_valid::Validate)]
#[rule(sample_rule(val2, val1))]
struct SampleStruct {
    val1: String,
    val2: i32,
}

let s = SampleStruct {
    val1: "val1".to_owned(),
    val2: 1,
};

assert_eq!(
    serde_json::to_string(&s.validate().unwrap_err()).unwrap(),
    serde_json::to_string(&json!({
        "val2": [
            "Rule error is added to the first arg of the rule_method."
        ]
    }))
    .unwrap()
);
```

If you want to use rule to unnamed fields struct, just like this,

```rust
fn sample_rule(_val1: &i32, _val2: &str) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}

#[derive(serde_valid::Validate)]
#[rule(sample_rule(0, 1))]
struct SampleStruct(i32, String);

let s = SampleStruct(0, "1".to_owned());

assert!(s.validate().is_ok());
```

## Validate Traits

By implementing the validation trait, Your original type can uses Serde Valid validations.

```rust
struct MyType(String);

impl serde_valid::ValidateMaxLength for MyType {
    fn validate_max_length(&self, max_length: usize) -> Result<(), serde_valid::MaxLengthErrorParams> {
        self.0.validate_max_length(max_length)
    }
}

impl serde_valid::ValidateMinLength for MyType {
    fn validate_min_length(&self, min_length: usize) -> Result<(), serde_valid::MinLengthErrorParams> {
        self.0.validate_min_length(min_length)
    }
}

#[derive(serde_valid::Validate)]
struct SampleStruct {
    #[validate(min_length = 5)]
    #[validate(max_length = 5)]
    val: MyType,
}

let s = SampleStruct {
    val: MyType(String::from("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦")),
};

assert!(s.validate().is_ok());
```
