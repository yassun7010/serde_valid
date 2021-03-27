use super::abort_invalid_attribute_on_field;
use crate::helper::SingleIdentPath;

pub fn abort_required_path_argument(
    validation_label: &str,
    expected_values: &[&str],
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
) {
    abort_invalid_attribute_on_field(
        field_ident,
        span,
        &format!(
            "Validator `{}` requires at least 1 argument from {:?}",
            validation_label, expected_values
        ),
    );
}

pub fn abort_required_list_argument(
    validation_label: &str,
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
    abort_required_path_argument(validation_label, expected_values, field_ident, span)
}

#[allow(dead_code)]
pub fn abort_required_name_value_argument(
    validation_label: &str,
    expected_values: &[&str],
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    _name_value: &syn::MetaNameValue,
) {
    abort_required_path_argument(validation_label, expected_values, field_ident, span)
}
