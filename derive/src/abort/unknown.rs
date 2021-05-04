use super::abort_invalid_attribute_on_field;
use crate::types::{Field, SingleIdentPath};

pub fn abort_unknown_lit_argument(
    validation_label: &str,
    field: &impl Field,
    span: proc_macro2::Span,
    _lit: &syn::Lit,
) -> ! {
    abort_invalid_attribute_on_field(
        field,
        span,
        &format!(
            "Unknown literal value while parsing `{}` validation of field `{}`",
            validation_label,
            field.ident()
        ),
    )
}

pub fn abort_unknown_path_argument(
    validation_label: &str,
    field: &impl Field,
    span: proc_macro2::Span,
    path: &syn::Path,
) -> ! {
    let path_ident = SingleIdentPath::new(&path).ident();
    abort_invalid_attribute_on_field(
        field,
        span,
        &format!(
            "Unknown item `{}` while parsing `{}` validation of field `{}`",
            path_ident,
            validation_label,
            field.ident()
        ),
    )
}

pub fn abort_unknown_list_argument(
    validation_label: &str,
    field: &impl Field,
    span: proc_macro2::Span,
    _list: &syn::MetaList,
) -> ! {
    abort_invalid_attribute_on_field(
        field,
        span,
        &format!(
            "Unknown item while parsing `{}` validation of field `{}`",
            validation_label,
            field.ident()
        ),
    )
}

pub fn abort_unknown_name_value_argument(
    validation_label: &str,
    field: &impl Field,
    span: proc_macro2::Span,
    name_value: &syn::MetaNameValue,
) -> ! {
    abort_unknown_path_argument(validation_label, field, span, &name_value.path)
}
