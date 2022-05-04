use super::abort_invalid_attribute_on_field;
use crate::types::{Field, SingleIdentPath};

pub fn abort_duplicated_argument(
    validation_label: &str,
    field: &impl Field,
    span: proc_macro2::Span,
    path_ident: &syn::Ident,
) -> ! {
    abort_invalid_attribute_on_field(
        field,
        span,
        &format!(
            "Duplicated `{}` argument of `{}` validator: only unique argument is allowed",
            path_ident, validation_label
        ),
    );
}

pub fn abort_duplicated_path_argument(
    validation_label: &str,
    field: &impl Field,
    span: proc_macro2::Span,
    path: &syn::Path,
) -> ! {
    let path_ident = SingleIdentPath::new(&path).ident();
    abort_duplicated_argument(validation_label, field, span, path_ident);
}

#[allow(dead_code)]
pub fn abort_duplicated_list_argument(
    validation_label: &str,
    field: &impl Field,
    span: proc_macro2::Span,
    list: &syn::MetaList,
) -> ! {
    abort_duplicated_path_argument(validation_label, field, span, &list.path)
}

#[allow(dead_code)]
pub fn abort_duplicated_name_value_argument(
    validation_label: &str,
    field: &impl Field,
    span: proc_macro2::Span,
    name_value: &syn::MetaNameValue,
) -> ! {
    abort_duplicated_path_argument(validation_label, field, span, &name_value.path)
}
