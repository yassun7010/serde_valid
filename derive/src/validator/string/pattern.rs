use crate::abort::abort_invalid_attribute_on_field;
use crate::helper::NamedField;
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;

pub fn extract_string_pattern_validator(field: &NamedField, lit: &syn::Lit) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_string_pattern_validator(
            &array_field,
            lit,
        )))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_string_pattern_validator(
            &option_field,
            lit,
        )))
    } else {
        Validator::Normal(inner_extract_string_pattern_validator(field.ident(), lit))
    }
}

fn inner_extract_string_pattern_validator(field_ident: &syn::Ident, lit: &syn::Lit) -> TokenStream {
    let field_string = field_ident.to_string();
    let pattern = match lit {
        syn::Lit::Str(l) => l.to_owned(),
        _ => abort_invalid_attribute_on_field(
            field_ident,
            lit.span(),
            "invalid argument type for `pattern` validator: only str literals are allowed",
        ),
    };
    let pattern_ident = syn::Ident::new(
        &format!("{}_PATTERN", &field_ident).to_uppercase(),
        field_ident.span(),
    );

    quote!(
        static #pattern_ident : once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        let pattern = #pattern_ident.get_or_init(|| regex::Regex::new(#pattern).unwrap());
        if !::serde_valid::validate_string_regular_expressions(
            #field_ident,
            pattern,
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(::serde_valid::validation::Error::PatternError(
                    ::serde_valid::validation::error::RegularExpressionErrorMessage::new(
                        #field_ident,
                        pattern,
                    )
                ));
        }
    )
}
