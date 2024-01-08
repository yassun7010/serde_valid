use crate::field_validate::common::get_numeric;
use crate::field_validate::{common::CustomMessageToken, Validator};
use crate::serde::rename::RenameMap;
use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;

/// Length validation.
///
/// See <https://json-schema.org/understanding-json-schema/reference/string.html#length>
macro_rules! extract_string_length_validator{
    ($ErrorType:ident) => {
        paste::paste! {
            pub fn [<extract_string_ $ErrorType:snake _validator>](
                field: &impl Field,
                validation_value: &syn::Lit,
                custom_message: CustomMessageToken,
                rename_map: &RenameMap,
            ) -> Result<Validator, crate::Errors> {
                [<inner_extract_string_ $ErrorType:snake _validator>](field, validation_value, custom_message, rename_map)
            }

            fn [<inner_extract_string_ $ErrorType:snake _validator>](
                field: &impl Field,
                validation_value: &syn::Lit,
                custom_message: CustomMessageToken,
                rename_map: &RenameMap,
            ) -> Result<TokenStream, crate::Errors> {
                let field_name = field.name();
                let field_ident = field.ident();
                let field_key = field.key();
                let rename = rename_map.get(field_name).unwrap_or(&field_key);
                let errors = field.errors_variable();
                let [<$ErrorType:snake>] = get_numeric(validation_value)?;
                let custom_message = custom_message.into_token();

                Ok(quote!(
                    if let Err(__composited_error_params) = ::serde_valid::validation::[<ValidateComposited $ErrorType>]::[<validate_composited_ $ErrorType:snake>](
                        #field_ident,
                        #[<$ErrorType:snake>],
                    ) {
                        use ::serde_valid::validation::{IntoError, ToDefaultMessage};

                        #errors
                            .entry(#rename)
                            .or_default()
                            .push(__composited_error_params.into_error_by(#custom_message));
                    }
                ))
            }
        }
    }
}

extract_string_length_validator!(MaxLength);
extract_string_length_validator!(MinLength);