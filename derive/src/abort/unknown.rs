use super::abort_invalid_attribute_on_field;
use crate::types::SingleIdentPath;

pub fn abort_unknown_path_argument(
    validation_label: &str,
    unkown_value: &str,
    expected_values: &[&str],
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
) {
    abort_invalid_attribute_on_field(
        field_ident,
        span,
        &format!(
            "Unknown argument `{}` for validator `{}` (it only has {:?})",
            unkown_value, validation_label, expected_values
        ),
    )
}

#[allow(dead_code)]
pub fn abort_unknown_list_argument(
    validation_label: &str,
    unkown_value: &str,
    expected_values: &[&str],
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    list: &syn::MetaList,
    allow_common_validation_args: bool,
) {
    if allow_common_validation_args {
        let path_ident = SingleIdentPath::new(&list.path).ident();
        match path_ident.to_string().as_str() {
            "message_fn" => return,
            _ => (),
        }
    }
    abort_unknown_path_argument(
        validation_label,
        unkown_value,
        expected_values,
        field_ident,
        span,
    )
}

pub fn abort_unknown_name_value_argument(
    validation_label: &str,
    unkown_value: &str,
    expected_values: &[&str],
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    _name_value: &syn::MetaNameValue,
) {
    abort_unknown_path_argument(
        validation_label,
        unkown_value,
        expected_values,
        field_ident,
        span,
    )
}
