use crate::abort::{
    abort_unexpected_list_argument, abort_unexpected_lit_argument,
    abort_unexpected_name_value_argument, abort_unexpected_path_argument,
};
use crate::types::SingleIdentPath;

pub fn check_meta(
    validation_label: &str,
    field_ident: &syn::Ident,
    span: proc_macro2::Span,
    meta: &syn::Meta,
    allow_common_validation_args: bool,
) {
    match meta {
        syn::Meta::List(list) => {
            if !(allow_common_validation_args && check_common_list_argument(list)) {
                abort_unexpected_list_argument(validation_label, field_ident, span, list)
            }
        }
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

pub fn check_common_list_argument(list: &syn::MetaList) -> bool {
    let path_ident = SingleIdentPath::new(&list.path).ident();
    match path_ident.to_string().as_str() {
        "message_fn" => true,
        _ => false,
    }
}
