use crate::errors::{Error, Errors};
use crate::types::{Field, SingleIdentPath};
use crate::validator::Validator;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;

pub fn extract_validator_from_meta_path(
    field: &impl Field,
    _attribute: &syn::Attribute,
    validation: &syn::Path,
) -> Result<Validator, Errors> {
    let validation_ident = SingleIdentPath::new(validation).ident();
    match validation_ident.to_string().as_ref() {
        "validate" => return Ok(extract_validate_validator(field)),
        v => Err(vec![Error::new(
            validation.span(),
            format!("Unexpected path validator: {v:?}"),
        )]),
    }
}

fn extract_validate_validator(field: &impl Field) -> Validator {
    if let Some(array_field) = field.array_field() {
        Validator::Array(Box::new(extract_validate_validator(&array_field)))
    } else if let Some(option_field) = field.option_field() {
        Validator::Option(Box::new(extract_validate_validator(&option_field)))
    } else {
        Validator::Normal(extract_validate_validator_tokens(field))
    }
}

fn extract_validate_validator_tokens(field: &impl Field) -> TokenStream {
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
