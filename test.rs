#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde_valid::Validate;
fn sample_rule(val: &i32) -> Result<(), serde_valid::validation::Error> {
    Ok(())
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const rule_named_struct_is_ok_test: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("rule_named_struct_is_ok_test"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(rule_named_struct_is_ok_test())),
};
fn rule_named_struct_is_ok_test() {
    #[rule(sample_rule(val))]
    struct TestStruct {
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        val: i32,
    }
    impl ::serde_valid::Validate for TestStruct {
        fn validate(&self) -> Result<(), ::serde_valid::validation::Errors> {
            let mut __errors = ::serde_valid::validation::MapErrors::new();
            let val = &self.val;
            if !::serde_valid::validate_numeric_minimum(*val, 0) {
                use ::serde_valid::error::ToDefaultMessage;
                __errors
                    .entry("val")
                    .or_default()
                    .push(::serde_valid::validation::Error::Minimum(
                        ::serde_valid::error::Message::new(
                            ::serde_valid::MinimumErrorParams::new(*val, 0),
                            ::serde_valid::MinimumErrorParams::to_default_message,
                        ),
                    ));
            }
            if !::serde_valid::validate_numeric_maximum(*val, 10) {
                use ::serde_valid::error::ToDefaultMessage;
                __errors
                    .entry("val")
                    .or_default()
                    .push(::serde_valid::validation::Error::Maximum(
                        ::serde_valid::error::Message::new(
                            ::serde_valid::MaximumErrorParams::new(*val, 10),
                            ::serde_valid::MaximumErrorParams::to_default_message,
                        ),
                    ));
            }
            if let Err(__error) = sample_rule(val) {
                __errors.entry("val").or_default().push(__error);
            };
            if __errors.is_empty() {
                Result::Ok(())
            } else {
                Result::Err(::serde_valid::validation::Errors::Fields(__errors))
            }
        }
    }
    let s = TestStruct { val: 5 };
    if !s.validate().is_ok() {
        ::core::panicking::panic("assertion failed: s.validate().is_ok()")
    };
}
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const rule_struct_unnamed_is_ok_test: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("rule_struct_unnamed_is_ok_test"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(rule_struct_unnamed_is_ok_test())),
};
fn rule_struct_unnamed_is_ok_test() {
    #[rule(sample_rule(0))]
    struct TestStruct(
        #[validate(minimum = 0)]
        #[validate(maximum = 10)]
        i32,
    );
    impl ::serde_valid::Validate for TestStruct {
        fn validate(&self) -> Result<(), ::serde_valid::validation::Errors> {
            let mut __errors = ::serde_valid::validation::MapErrors::new();
            let __0 = &self.0;
            if !::serde_valid::validate_numeric_minimum(*__0, 0) {
                use ::serde_valid::error::ToDefaultMessage;
                __errors
                    .entry("0")
                    .or_default()
                    .push(::serde_valid::validation::Error::Minimum(
                        ::serde_valid::error::Message::new(
                            ::serde_valid::MinimumErrorParams::new(*__0, 0),
                            ::serde_valid::MinimumErrorParams::to_default_message,
                        ),
                    ));
            }
            if !::serde_valid::validate_numeric_maximum(*__0, 10) {
                use ::serde_valid::error::ToDefaultMessage;
                __errors
                    .entry("0")
                    .or_default()
                    .push(::serde_valid::validation::Error::Maximum(
                        ::serde_valid::error::Message::new(
                            ::serde_valid::MaximumErrorParams::new(*__0, 10),
                            ::serde_valid::MaximumErrorParams::to_default_message,
                        ),
                    ));
            }
            if __errors.is_empty() {
                Result::Ok(())
            } else {
                Result::Err(::serde_valid::validation::Errors::NewType(
                    __errors.remove("0").unwrap(),
                ))
            }
        }
    }
    let s = TestStruct(5);
    if !s.validate().is_ok() {
        ::core::panicking::panic("assertion failed: s.validate().is_ok()")
    };
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[
        &rule_named_struct_is_ok_test,
        &rule_struct_unnamed_is_ok_test,
    ])
}
