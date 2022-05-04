use super::abort_invalid_attribute_on_field;
use crate::types::Field;

pub fn abort_unexpected_path_argument(
    validation_label: &str,
    value: &str,
    expected_values: &[&str],
    field: &impl Field,
    span: proc_macro2::Span,
) -> ! {
    abort_invalid_attribute_on_field(
        field,
        span,
        &format!(
            "Unexpected argument `{}` for validator `{}` (it only has {:?})",
            value, validation_label, expected_values
        ),
    )
}

#[allow(dead_code)]
pub fn abort_unexpected_list_argument(
    validation_label: &str,
    value: &str,
    expected_values: &[&str],
    field: &impl Field,
    span: proc_macro2::Span,
    _list: &syn::MetaList,
) -> ! {
    abort_unexpected_path_argument(validation_label, value, expected_values, field, span)
}
