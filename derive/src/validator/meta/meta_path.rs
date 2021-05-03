use crate::types::{Field, SingleIdentPath};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_validator_from_meta_path<F: Field>(
    field: &F,
    _attribute: &syn::Attribute,
    validation: &syn::Path,
) -> Option<Validator> {
    let validation_ident = SingleIdentPath::new(validation).ident();
    match validation_ident.to_string().as_ref() {
        "validate" => return Some(extract_validate_validator(field)),
        v => {
            abort!(validation.span(), "Unexpected path validator: {:?}", v)
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
        if let Err(__inner_errors) = #field_ident.validate() {
            match __inner_errors {
                __fields_errors @ ::serde_valid::validation::Errors::Fields(_) => {
                    __errors.insert(
                        #field_name,
                        vec![::serde_valid::validation::Error::Nested(__fields_errors)]
                    );
                }
                ::serde_valid::validation::Errors::NewType(__new_type_errors) => {
                    __errors.insert(#field_name, __new_type_errors);
                }
            }
        }
    )
}
