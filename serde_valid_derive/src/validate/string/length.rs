use crate::serde::rename::RenameMap;
use crate::types::Field;
use crate::validate::common::get_numeric;
use crate::validate::{common::CustomMessageToken, Validator};
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
                let message_fn = custom_message
                    .message_fn.unwrap_or(quote!(::serde_valid::[<$ErrorType Error>]::to_default_message));

                Ok(quote!(
                    if let Err(__composited_error_params) = ::serde_valid::validation::[<ValidateComposited $ErrorType>]::[<validate_composited_ $ErrorType:snake>](
                        #field_ident,
                        #[<$ErrorType:snake>],
                    ) {
                        use ::serde_valid::error::ToDefaultMessage;
                        use ::serde_valid::validation::IntoError;

                        #errors
                            .entry(#rename)
                            .or_default()
                            .push(__composited_error_params.into_error_by(#message_fn)
                        );
                    }
                ))
            }
        }
    }
}

extract_string_length_validator!(MaxLength);
extract_string_length_validator!(MinLength);
