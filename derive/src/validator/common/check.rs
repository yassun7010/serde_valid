use crate::abort::{
    abort_unexpected_list_argument, abort_unexpected_lit_argument,
    abort_unexpected_name_value_argument, abort_unexpected_path_argument,
};
use crate::types::SingleIdentPath;
use syn::spanned::Spanned;

pub fn check_validation_arg_meta(
    validation_label: &str,
    field_ident: &syn::Ident,
    arg: &syn::Meta,
    allow_common_validation_args: bool,
) {
    match arg {
        syn::Meta::List(list) => {
            if !(allow_common_validation_args && check_common_meta_list_argument(list)) {
                abort_unexpected_list_argument(validation_label, field_ident, list.span(), list)
            }
        }
        syn::Meta::NameValue(name_value) => {
            if !(allow_common_validation_args && check_common_meta_name_value_argument(name_value))
            {
                abort_unexpected_name_value_argument(
                    validation_label,
                    field_ident,
                    name_value.span(),
                    name_value,
                )
            }
        }
        syn::Meta::Path(path) => {
            abort_unexpected_path_argument(validation_label, field_ident, arg.span(), path)
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

pub fn check_common_meta_list_argument(list: &syn::MetaList) -> bool {
    let path_ident = SingleIdentPath::new(&list.path).ident();
    match path_ident.to_string().as_str() {
        "message_fn" => true,
        _ => false,
    }
}

pub fn check_common_meta_name_value_argument(name_value: &syn::MetaNameValue) -> bool {
    let path_ident = SingleIdentPath::new(&name_value.path).ident();
    match path_ident.to_string().as_str() {
        "message" => true,
        _ => false,
    }
}
