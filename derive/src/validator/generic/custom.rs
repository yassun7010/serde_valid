use crate::abort::{
    abort_unknown_list_argument, abort_unknown_lit_argument, abort_unknown_name_value_argument,
};
use crate::types::Field;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

const VALIDATION_LABEL: &'static str = "custom";

pub fn extract_generic_custom_validator<F: Field>(
    field: &F,
    attribute: &syn::Attribute,
    syn::MetaList {
        nested: validation_args,
        ..
    }: &syn::MetaList,
) -> Validator {
    let mut custom_validation_fn = None;
    let mut custom_validation_args = None;
    let field_ident = field.ident();

    for validation_arg in validation_args {
        match validation_arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::Path(fn_name) => {
                    update_custom_validator_from_meta_path(&mut custom_validation_fn, fn_name)
                }
                syn::Meta::List(fn_define) => update_custom_validator_from_meta_list(
                    &mut custom_validation_fn,
                    &mut custom_validation_args,
                    field,
                    fn_define,
                ),
                syn::Meta::NameValue(name_value) => abort_unknown_name_value_argument(
                    VALIDATION_LABEL,
                    field,
                    attribute.span(),
                    name_value,
                ),
            },
            syn::NestedMeta::Lit(lit) => {
                abort_unknown_lit_argument(VALIDATION_LABEL, field, attribute.span(), &lit)
            }
        }
    }

    let custom_validation_fn = custom_validation_fn.unwrap_or_else(|| {
        abort!(
            attribute.span(),
            &format!(
                "'{}' literal meta items size must be greater than 0.",
                VALIDATION_LABEL
            )
        )
    });

    let field_name = field.name();
    return Validator::Normal(quote!(
        if let Err(__error) = #custom_validation_fn(#field_ident, #custom_validation_args) {
            __errors
                .entry(#field_name)
                .or_default()
                .push(__error);
        };
    ));
}

fn update_custom_validator_from_meta_path(
    custom_validation_fn: &mut Option<TokenStream>,
    fn_name: &syn::Path,
) {
    check_duplicated_custom_validation_fn(custom_validation_fn, fn_name);
    *custom_validation_fn = Some(quote!(#fn_name));
}

fn update_custom_validator_from_meta_list<F: Field>(
    custom_validation_fn: &mut Option<TokenStream>,
    custom_validation_args: &mut Option<TokenStream>,
    field: &F,
    fn_define: &syn::MetaList,
) {
    let fn_name = &fn_define.path;
    let fn_args = extract_custom_validator_args(field, &fn_define.nested);
    check_duplicated_custom_validation_fn(custom_validation_fn, fn_name);
    check_duplicated_custom_validation_args(custom_validation_args, fn_name, &fn_args);

    *custom_validation_fn = Some(quote!(#fn_name));
    *custom_validation_args = Some(quote!(#fn_args));
}

fn extract_custom_validator_args<F: Field>(
    field: &F,
    fn_args: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
) -> syn::punctuated::Punctuated<TokenStream, syn::token::Comma> {
    fn_args
        .iter()
        .map(|fn_arg| extract_custom_validator_arg(field, fn_arg))
        .collect()
}

fn extract_custom_validator_arg<F: Field>(field: &F, fn_arg: &syn::NestedMeta) -> TokenStream {
    match fn_arg {
        syn::NestedMeta::Lit(lit) => quote!(#lit),
        syn::NestedMeta::Meta(meta) => match meta {
            syn::Meta::Path(field) => quote!(&self.#field),
            syn::Meta::List(list) => {
                abort_unknown_list_argument(VALIDATION_LABEL, field, fn_arg.span(), &list)
            }
            syn::Meta::NameValue(name_value) => abort_unknown_name_value_argument(
                VALIDATION_LABEL,
                field,
                fn_arg.span(),
                &name_value,
            ),
        },
    }
}

fn check_duplicated_custom_validation_fn(
    custom_validation_fn: &Option<TokenStream>,
    fn_name: &syn::Path,
) {
    if custom_validation_fn.is_some() {
        abort!(
            fn_name.span(),
            &format!("'{}' validator allow 1 custom function.", VALIDATION_LABEL)
        )
    }
}

fn check_duplicated_custom_validation_args(
    custom_validation_args: &Option<TokenStream>,
    fn_name: &syn::Path,
    fn_args: &syn::punctuated::Punctuated<TokenStream, syn::token::Comma>,
) {
    if custom_validation_args.is_some() {
        abort!(
            fn_name.span(),
            &format!(
                "'{}' validator allow to define custom function args only once.",
                VALIDATION_LABEL
            )
        )
    }
    if fn_args.is_empty() {
        abort!(
            fn_name.span(),
            &format!(
                "'{}' validator need 1 or more custom function args.",
                VALIDATION_LABEL
            )
        )
    }
}
