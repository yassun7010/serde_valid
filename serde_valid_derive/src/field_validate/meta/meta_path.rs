use crate::field_validate::Validator;
use crate::serde::rename::RenameMap;
use crate::types::Field;
use quote::quote;

pub fn extract_field_validator_from_meta_path(
    field: &impl Field,
    rename_map: &RenameMap,
) -> Result<Validator, crate::Errors> {
    let field_ident = field.ident();
    let field_name = field.name();
    let field_key = field.key();
    let rename = rename_map.get(field_name).unwrap_or(&field_key);
    let errors = field.errors_variable();

    Ok(quote!(
        if let Err(__inner_errors) = #field_ident.validate() {
            match __inner_errors {
                ::serde_valid::validation::Errors::Object(__object_errors) => {
                    #errors.entry(#rename).or_default().push(
                        ::serde_valid::validation::Error::Properties(__object_errors)
                    );
                }
                ::serde_valid::validation::Errors::Array(__array_errors) => {
                    #errors.entry(#rename).or_default().push(
                        ::serde_valid::validation::Error::Items(__array_errors)
                    );
                }
                ::serde_valid::validation::Errors::NewType(__new_type_errors) => {
                    #errors.entry(#rename).or_default().extend(__new_type_errors);
                }
            }
        }
    ))
}