use crate::helper::NamedField;
use crate::validator::{abort_invalid_attribute_on_field, Validator};
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_pattern_validator(field: &NamedField, lit: &syn::Lit) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_pattern_validator(&array_field, lit)))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_pattern_validator(&option_field, lit)))
    } else {
        Validator::Normal(inner_extract_pattern_validator(field.ident(), lit))
    }
}

fn inner_extract_pattern_validator(field_ident: &syn::Ident, lit: &syn::Lit) -> TokenStream {
    let pattern = match lit {
        syn::Lit::Str(l) => l.to_owned(),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            "invalid argument type for `pattern` validator: only str literals are allowed",
        ),
    };
    let token = quote!(
        let pattern = regex::Regex::new(#pattern).unwrap();
        if !::serde_valid::validate_pattern(
            #field_ident,
            pattern,
        ) {
            errors.push(::serde_valid::Error::PatternError);
        }
    );
    token
}