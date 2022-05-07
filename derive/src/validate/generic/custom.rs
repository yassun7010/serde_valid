use crate::types::Field;
use crate::validate::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_generic_custom_validator(
    field: &impl Field,
    syn::MetaList { path, nested, .. }: &syn::MetaList,
) -> Result<Validator, crate::Error> {
    let field_name = field.name();
    let field_ident = field.ident();

    let custom_fn_name = match nested.len() {
        0 => Err(crate::Error::validate_custom_need_item(path.span()))?,
        1 => extract_custom_fn_name(&nested[0])?,
        _ => Err(crate::Error::validate_custom_tail_error(nested.span()))?,
    };

    Ok(Validator::Normal(quote!(
        if let Err(__error) = #custom_fn_name(#field_ident) {
            __errors
                .entry(#field_name)
                .or_default()
                .push(__error);
        };
    )))
}

fn extract_custom_fn_name(nested_meta: &syn::NestedMeta) -> Result<TokenStream, crate::Error> {
    match nested_meta {
        syn::NestedMeta::Meta(meta) => match meta {
            syn::Meta::List(list) => {
                let fn_name = &list.path;
                Ok(quote!(#fn_name))
            }
            syn::Meta::NameValue(name_value) => {
                Err(crate::Error::meta_name_value_not_support(name_value))?
            }
            syn::Meta::Path(fn_name) => Ok(quote!(#fn_name)),
        },
        syn::NestedMeta::Lit(lit) => Err(crate::Error::literal_not_support(lit))?,
    }
}
