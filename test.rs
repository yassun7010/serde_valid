#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use serde_valid::Validate;
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const empty_struct_with_braces_is_ok: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("empty_struct_with_braces_is_ok"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(empty_struct_with_braces_is_ok())),
};
fn empty_struct_with_braces_is_ok() {
    struct TestStruct {}
    impl ::serde_valid::Validate for TestStruct {
        fn validate(&self) -> std::result::Result<(), ::serde_valid::validation::Errors> {
            let mut __errors = ::serde_valid::validation::VecErrors::new();
            let mut __properties_errors = ::serde_valid::validation::MapErrors::new();
            if __errors.is_empty() && __properties_errors.is_empty() {
                Ok(())
            } else {
                Err(::serde_valid::validation::Errors::Object(
                    ::serde_valid::validation::ObjectErrors::new(
                        __errors,
                        ::serde_valid::validation::Errors::NewType(__properties_errors),
                    ),
                ))
            }
        }
    }
    let s = TestStruct {};
    if !s.validate().is_ok() {
        ::core::panicking::panic("assertion failed: s.validate().is_ok()")
    };
}
#[rustc_main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&empty_struct_with_braces_is_ok])
}
