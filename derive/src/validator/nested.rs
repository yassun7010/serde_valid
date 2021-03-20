use crate::helper::{NamedField, SingleIdentPath};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_nested_validator(
    field: &NamedField,
    _attribute: &syn::Attribute,
    path: &syn::Path,
) -> Option<Validator> {
    let path_ident = SingleIdentPath::new(path).ident();
    match path_ident.to_string().as_ref() {
        "validate" => return Some(Validator::Normal(extract_nested_validator_tokens(field))),
        v => {
            abort!(path.span(), "Unexpected path validator: {:?}", v)
        }
    }
}

fn extract_nested_validator_tokens(field: &NamedField) -> TokenStream {
    let field_ident = field.ident();
    quote!(
        if let Err(errs) = #field_ident.validate() {
            errors.extend(errs);
        }

    )
}
