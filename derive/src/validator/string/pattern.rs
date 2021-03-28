mod pattern_from_meta_list;
mod pattern_from_meta_name_value;

pub use pattern_from_meta_list::extract_string_pattern_of_validator_from_meta_list;
pub use pattern_from_meta_name_value::extract_string_pattern_validator_from_meta_name_value;
use proc_macro2::TokenStream;
use quote::quote;

const VALIDATION_LABEL: &'static str = "pattern";

fn inner_extract_string_pattern_validator(
    field_ident: &syn::Ident,
    pattern: &syn::LitStr,
    message: &TokenStream,
) -> TokenStream {
    let field_string = field_ident.to_string();
    let pattern_ident = syn::Ident::new(
        &format!("{}_PATTERN", &field_ident).to_uppercase(),
        field_ident.span(),
    );

    quote!(
        static #pattern_ident : once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        let pattern = #pattern_ident.get_or_init(|| regex::Regex::new(#pattern).unwrap());
        if !::serde_valid::validate_string_pattern(
            #field_ident,
            pattern,
        ) {
            errors
                .entry(::serde_valid::FieldName::new(#field_string))
                .or_default()
                .push(::serde_valid::validation::Error::PatternError(
                    ::serde_valid::validation::error::Message::new(
                        ::serde_valid::validation::error::PatternErrorParams::new(
                            #field_ident,
                            pattern,
                        ),
                        #message
                    )
                ));
        }
    )
}
