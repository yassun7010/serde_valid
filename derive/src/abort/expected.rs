use super::abort_invalid_attribute_on_field;

pub fn abort_unexpected_path_argument(
    validation_label: &str,
    value: &str,
    expected_values: &[&str],
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
) -> ! {
    abort_invalid_attribute_on_field(
        field_ident,
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
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    _list: &syn::MetaList,
) -> ! {
    abort_unexpected_path_argument(validation_label, value, expected_values, field_ident, span)
}

pub fn abort_unexpected_name_value_argument(
    validation_label: &str,
    value: &str,
    expected_values: &[&str],
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    _name_value: &syn::MetaNameValue,
) -> ! {
    abort_unexpected_path_argument(validation_label, value, expected_values, field_ident, span)
}
