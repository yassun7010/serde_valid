#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde_valid::Validate;
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const enum_named_variant_validation_is_ok: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("enum_named_variant_validation_is_ok"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(enum_named_variant_validation_is_ok())),
};
fn enum_named_variant_validation_is_ok() {
    enum TestEnum {
        Named {
            #[validate]
            a: TestStruct,
        },
    }
    impl ::serde_valid::Validate for TestEnum {
        fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
            if let TestEnum::Named { a } = &self {
                let mut __errors = ::serde_valid::validation::VecErrors::new();
                let mut __properties_errors = ::serde_valid::validation::MapErrors::new();
                if let Err(__inner_errors) = a.validate() {
                    match __inner_errors {
                        __property_errors @ ::serde_valid::validation::Errors::Object(_) => {
                            __properties_errors.insert(
                                "a",
                                <[_]>::into_vec(box [::serde_valid::validation::Error::Nested(
                                    __property_errors,
                                )]),
                            );
                        }
                        __property_errors @ ::serde_valid::validation::Errors::Array(_) => {
                            ::core::panicking::panic("not implemented");
                        }
                        ::serde_valid::validation::Errors::NewType(__new_type_errors) => {
                            __properties_errors.insert("a", __new_type_errors);
                        }
                    }
                }
                if !__errors.is_empty() {
                    Err(::serde_valid::validation::Errors::Object(
                        ::serde_valid::validation::ObjectErrors::new(
                            __errors,
                            __properties_errors
                                .into_iter()
                                .map(|(field, errors)| {
                                    (field, ::serde_valid::validation::Errors::NewType(errors))
                                })
                                .collect(),
                        ),
                    ))?
                }
            }
            Ok(())
        }
    }
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }
    impl ::serde_valid::Validate for TestStruct {
        fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
            let mut __errors = ::serde_valid::validation::VecErrors::new();
            let mut __properties_errors = ::serde_valid::validation::MapErrors::new();
            let val = &self.val;
            if let Err(error_params) = ::serde_valid::ValidateMinimum::validate_minimum(val, 0) {
                use ::serde_valid::error::ToDefaultMessage;
                __properties_errors.entry("val").or_default().push(
                    ::serde_valid::validation::Error::Minimum(::serde_valid::error::Message::new(
                        error_params,
                        ::serde_valid::MinimumErrorParams::to_default_message,
                    )),
                );
            }
            if let Err(error_params) = ::serde_valid::ValidateMaximum::validate_maximum(val, 10) {
                use ::serde_valid::error::ToDefaultMessage;
                __properties_errors.entry("val").or_default().push(
                    ::serde_valid::validation::Error::Maximum(::serde_valid::error::Message::new(
                        error_params,
                        ::serde_valid::MaximumErrorParams::to_default_message,
                    )),
                );
            }
            if __errors.is_empty() && __properties_errors.is_empty() {
                Ok(())
            } else {
                Err(::serde_valid::validation::Errors::Object(
                    ::serde_valid::validation::ObjectErrors::new(
                        __errors,
                        __properties_errors
                            .into_iter()
                            .map(|(field, errors)| {
                                (field, ::serde_valid::validation::Errors::NewType(errors))
                            })
                            .collect(),
                    ),
                ))
            }
        }
    }
    let s = TestEnum::Named {
        a: TestStruct { val: 12 },
    };
    if !s.validate().is_err() {
        ::core::panicking::panic("assertion failed: s.validate().is_err()")
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const enum_unnamed_variant_validation_is_ok: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("enum_unnamed_variant_validation_is_ok"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        || test::assert_test_result(enum_unnamed_variant_validation_is_ok()),
    ),
};
fn enum_unnamed_variant_validation_is_ok() {
    enum TestEnum {
        UnNamed(
            #[validate(minimum = 0)]
            #[validate(maximum = 10)]
            i32,
            #[validate] TestStruct,
        ),
    }
    impl ::serde_valid::Validate for TestEnum {
        fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
            if let TestEnum::UnNamed(__0, __1) = &self {
                let mut __errors = ::serde_valid::validation::VecErrors::new();
                let mut __properties_errors = ::serde_valid::validation::MapErrors::new();
                if let Err(error_params) = ::serde_valid::ValidateMinimum::validate_minimum(__0, 0)
                {
                    use ::serde_valid::error::ToDefaultMessage;
                    __properties_errors.entry("0").or_default().push(
                        ::serde_valid::validation::Error::Minimum(
                            ::serde_valid::error::Message::new(
                                error_params,
                                ::serde_valid::MinimumErrorParams::to_default_message,
                            ),
                        ),
                    );
                }
                if let Err(error_params) = ::serde_valid::ValidateMaximum::validate_maximum(__0, 10)
                {
                    use ::serde_valid::error::ToDefaultMessage;
                    __properties_errors.entry("0").or_default().push(
                        ::serde_valid::validation::Error::Maximum(
                            ::serde_valid::error::Message::new(
                                error_params,
                                ::serde_valid::MaximumErrorParams::to_default_message,
                            ),
                        ),
                    );
                }
                if let Err(__inner_errors) = __1.validate() {
                    match __inner_errors {
                        __property_errors @ ::serde_valid::validation::Errors::Object(_) => {
                            __properties_errors.insert(
                                "1",
                                <[_]>::into_vec(box [::serde_valid::validation::Error::Nested(
                                    __property_errors,
                                )]),
                            );
                        }
                        __property_errors @ ::serde_valid::validation::Errors::Array(_) => {
                            ::core::panicking::panic("not implemented");
                        }
                        ::serde_valid::validation::Errors::NewType(__new_type_errors) => {
                            __properties_errors.insert("1", __new_type_errors);
                        }
                    }
                }
                if !__errors.is_empty() {
                    Err(::serde_valid::validation::Errors::Object(
                        ::serde_valid::validation::ObjectErrors::new(
                            __errors,
                            __properties_errors
                                .into_iter()
                                .map(|(field, errors)| {
                                    (field, ::serde_valid::validation::Errors::NewType(errors))
                                })
                                .collect(),
                        ),
                    ))?
                }
            }
            Ok(())
        }
    }
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }
    impl ::serde_valid::Validate for TestStruct {
        fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
            let mut __errors = ::serde_valid::validation::VecErrors::new();
            let mut __properties_errors = ::serde_valid::validation::MapErrors::new();
            let val = &self.val;
            if let Err(error_params) = ::serde_valid::ValidateMinimum::validate_minimum(val, 0) {
                use ::serde_valid::error::ToDefaultMessage;
                __properties_errors.entry("val").or_default().push(
                    ::serde_valid::validation::Error::Minimum(::serde_valid::error::Message::new(
                        error_params,
                        ::serde_valid::MinimumErrorParams::to_default_message,
                    )),
                );
            }
            if let Err(error_params) = ::serde_valid::ValidateMaximum::validate_maximum(val, 10) {
                use ::serde_valid::error::ToDefaultMessage;
                __properties_errors.entry("val").or_default().push(
                    ::serde_valid::validation::Error::Maximum(::serde_valid::error::Message::new(
                        error_params,
                        ::serde_valid::MaximumErrorParams::to_default_message,
                    )),
                );
            }
            if __errors.is_empty() && __properties_errors.is_empty() {
                Ok(())
            } else {
                Err(::serde_valid::validation::Errors::Object(
                    ::serde_valid::validation::ObjectErrors::new(
                        __errors,
                        __properties_errors
                            .into_iter()
                            .map(|(field, errors)| {
                                (field, ::serde_valid::validation::Errors::NewType(errors))
                            })
                            .collect(),
                    ),
                ))
            }
        }
    }
    let s = TestEnum::UnNamed(5, TestStruct { val: 5 });
    if !s.validate().is_ok() {
        ::core::panicking::panic("assertion failed: s.validate().is_ok()")
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const enum_newtype_variant_validation_is_ok: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("enum_newtype_variant_validation_is_ok"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(
        || test::assert_test_result(enum_newtype_variant_validation_is_ok()),
    ),
};
fn enum_newtype_variant_validation_is_ok() {
    enum TestEnum {
        NewType(
            #[validate(minimum = 0)]
            #[validate(maximum = 10)]
            i32,
        ),
    }
    impl ::serde_valid::Validate for TestEnum {
        fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
            if let TestEnum::NewType(__0) = &self {
                let mut __errors = ::serde_valid::validation::VecErrors::new();
                let mut __properties_errors = ::serde_valid::validation::MapErrors::new();
                if let Err(error_params) = ::serde_valid::ValidateMinimum::validate_minimum(__0, 0)
                {
                    use ::serde_valid::error::ToDefaultMessage;
                    __properties_errors.entry("0").or_default().push(
                        ::serde_valid::validation::Error::Minimum(
                            ::serde_valid::error::Message::new(
                                error_params,
                                ::serde_valid::MinimumErrorParams::to_default_message,
                            ),
                        ),
                    );
                }
                if let Err(error_params) = ::serde_valid::ValidateMaximum::validate_maximum(__0, 10)
                {
                    use ::serde_valid::error::ToDefaultMessage;
                    __properties_errors.entry("0").or_default().push(
                        ::serde_valid::validation::Error::Maximum(
                            ::serde_valid::error::Message::new(
                                error_params,
                                ::serde_valid::MaximumErrorParams::to_default_message,
                            ),
                        ),
                    );
                }
                if !__errors.is_empty() {
                    Err(::serde_valid::validation::Errors::NewType(
                        __properties_errors.remove("0").unwrap(),
                    ))?
                }
            }
            Ok(())
        }
    }
    let s = TestEnum::NewType(15);
    if !s.validate().is_err() {
        ::core::panicking::panic("assertion failed: s.validate().is_err()")
    };
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &enum_named_variant_validation_is_ok,
        &enum_unnamed_variant_validation_is_ok,
        &enum_newtype_variant_validation_is_ok,
    ])
}
