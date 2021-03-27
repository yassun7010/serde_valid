use crate::abort::{
    abort_unexpected_list_argument, abort_unexpected_lit_argument,
    abort_unexpected_name_value_argument, abort_unexpected_path_argument,
};

pub fn check_meta(
    validation_label: &str,
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    meta: &syn::Meta,
    allow_common_validation_args: bool,
) {
    match meta {
        syn::Meta::List(list) => abort_unexpected_list_argument(
            validation_label,
            field_ident,
            span,
            list,
            allow_common_validation_args,
        ),
        syn::Meta::NameValue(name_value) => {
            abort_unexpected_name_value_argument(validation_label, field_ident, span, name_value)
        }
        syn::Meta::Path(path) => {
            abort_unexpected_path_argument(validation_label, field_ident, span, path)
        }
    }
}

pub fn check_lit(
    validation_label: &str,
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    lit: &syn::Lit,
) {
    abort_unexpected_lit_argument(validation_label, field_ident, span, lit)
}
