use crate::types::{Field, SingleIdentPath};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_validator_from_meta_path<F: Field>(
    field: &F,
    _attribute: &syn::Attribute,
    path: &syn::Path,
) -> Option<Validator> {
    let path_ident = SingleIdentPath::new(path).ident();
    match path_ident.to_string().as_ref() {
        "validate" => return Some(extract_validate_validator(field)),
        v => {
            abort!(path.span(), "Unexpected path validator: {:?}", v)
        }
    }
}

fn extract_validate_validator<F: Field>(field: &F) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_validate_validator(&array_field)))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_validate_validator(&option_field)))
    } else {
        Validator::Normal(extract_validate_validator_tokens(field))
    }
}

fn extract_validate_validator_tokens<F: Field>(field: &F) -> TokenStream {
    let field_ident = field.ident();
    let field_name = field.name();
    quote!(
        if let Err(inner_errors) = #field_ident.validate() {
            match inner_errors {
                fields_errors @ ::serde_valid::validation::Errors::Fields(_) => {
                    errors
                        .entry(::serde_valid::FieldName::new(#field_name))
                        .or_default()
                        .push(::serde_valid::validation::Error::Nested(
                            fields_errors
                        ));
                }
                ::serde_valid::validation::Errors::NewType(new_type_errors) => {
                    errors
                        .entry(::serde_valid::FieldName::new(#field_name))
                        .or_default()
                        .extend(new_type_errors);
                }
            }
        }
    )
}
