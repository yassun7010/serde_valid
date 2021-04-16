use super::abort_invalid_attribute_on_field;
use crate::types::Field;

pub fn abort_required_path_argument<F: Field>(
    validation_label: &str,
    expected_values: &[&str],
    field: &F,
    span: proc_macro2::Span,
) -> ! {
    abort_invalid_attribute_on_field(
        field,
        span,
        &format!(
            "Validator `{}` requires at least 1 argument from {:?}",
            validation_label, expected_values
        ),
    );
}

#[allow(dead_code)]
pub fn abort_required_list_argument<F: Field>(
    validation_label: &str,
    expected_values: &[&str],
    field: &F,
    span: proc_macro2::Span,
    _list: &syn::MetaList,
) -> ! {
    abort_required_path_argument(validation_label, expected_values, field, span)
}

#[allow(dead_code)]
pub fn abort_required_name_value_argument<F: Field>(
    validation_label: &str,
    expected_values: &[&str],
    field: &F,
    span: proc_macro2::Span,
    _name_value: &syn::MetaNameValue,
) -> ! {
    abort_required_path_argument(validation_label, expected_values, field, span)
}
