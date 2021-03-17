use crate::helper::NamedField;
use crate::lit::LitNumeric;
use crate::validator::{abort_invalid_attribute_on_field, Validator};
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_numeric_multiples_validator(field: &NamedField, lit: &syn::Lit) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_numeric_multiples_validator(
            &array_field,
            lit,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_numeric_multiples_validator(
            &option_field,
            lit,
        )))
    } else {
        Validator::Normal(inner_extract_numeric_multiples_validator(
            field.ident(),
            lit,
        ))
    }
}

fn inner_extract_numeric_multiples_validator(
    field_ident: &syn::Ident,
    lit: &syn::Lit,
) -> TokenStream {
    let multiple_of = match lit {
        syn::Lit::Int(l) => LitNumeric::Int(l.to_owned()),
        syn::Lit::Float(l) => LitNumeric::Float(l.to_owned()),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            "invalid argument type for `multiple_of` validator: only numeric literals are allowed",
        ),
    };
    let token = quote!(
        if !::serde_valid::validate_numeric_multiples(
            *#field_ident,
            #multiple_of,
        ) {
            errors.push(::serde_valid::Error::MultipleOfError);
        }
    );
    token
}
