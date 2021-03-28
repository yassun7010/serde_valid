use super::abort_invalid_attribute_on_field;
use crate::types::SingleIdentPath;

pub fn abort_unexpected_lit_argument(
    validation_label: &str,
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    _lit: &syn::Lit,
) {
    abort_invalid_attribute_on_field(
        field_ident,
        span,
        &format!(
            "Unexpected literal value while parsing `{}` validation of field `{}`",
            validation_label, field_ident
        ),
    )
}

pub fn abort_unexpected_path_argument(
    validation_label: &str,
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    path: &syn::Path,
) {
    let path_ident = SingleIdentPath::new(&path).ident();
    abort_invalid_attribute_on_field(
        field_ident,
        span,
        &format!(
            "Unexpected item `{}` while parsing `{}` validation of field `{}`",
            path_ident, validation_label, field_ident
        ),
    )
}

pub fn abort_unexpected_list_argument(
    validation_label: &str,
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
    abort_unexpected_path_argument(validation_label, field_ident, span, &list.path)
}

pub fn abort_unexpected_name_value_argument(
    validation_label: &str,
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    name_value: &syn::MetaNameValue,
) {
    abort_unexpected_path_argument(validation_label, field_ident, span, &name_value.path)
}
