use crate::lit::LitNumber;
use crate::validator::abort_invalid_attribute_on_field;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_multiples_validator(field_ident: &syn::Ident, lit: &syn::Lit) -> TokenStream {
    let multiple_of = match lit {
        syn::Lit::Int(l) => LitNumber::Int(l.to_owned()),
        syn::Lit::Float(l) => LitNumber::Float(l.to_owned()),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            "invalid argument type for `multiple_of` validator: only number literals are allowed",
        ),
    };
    let validator_param = quote!(self.#field_ident);

    quote!(
        if !::serde_valid::validate_multiples(
            #validator_param,
            #multiple_of,
        ) {
            errors.push(::serde_valid::Error::MultipleOfError);
        }
    )
}
