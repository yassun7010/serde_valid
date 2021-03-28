mod array;
mod common;
mod generic;
mod meta;
mod numeric;
mod object;
mod string;

pub use crate::types::NamedField;
pub use meta::extract_meta_validator;
use proc_macro2::TokenStream;
use quote::quote;
use std::iter::FromIterator;

pub enum Validator {
    Normal(TokenStream),
    Option(Box<Validator>),
    Array(Box<Validator>),
}

pub struct FieldValidators {
    field: NamedField,
    validators: Vec<TokenStream>,
    optional_validators: Option<Box<FieldValidators>>,
    array_validators: Option<Box<FieldValidators>>,
}

impl FieldValidators {
    pub fn new(field: syn::Field) -> Self {
        Self::inner_new(NamedField::new(field))
    }

    fn inner_new(field: NamedField) -> Self {
        Self {
            field,
            validators: vec![],
            optional_validators: None,
            array_validators: None,
        }
    }

    pub fn push(&mut self, validator: Validator) {
        match validator {
            Validator::Normal(token) => self.validators.push(token),
            Validator::Option(ty) => match self.optional_validators.as_mut() {
                Some(optional_validator) => optional_validator.push(*ty),
                None => {
                    if let Some(field) = self.field.option_field() {
                        let mut option_validators = Box::new(Self::inner_new(field));
                        option_validators.push(*ty);
                        self.optional_validators = Some(option_validators);
                    }
                }
            },
            Validator::Array(ty) => match self.array_validators.as_mut() {
                Some(array_validator) => array_validator.push(*ty),
                None => {
                    if let Some(field) = self.field.array_field() {
                        let mut array_validators = Box::new(Self::inner_new(field));
                        array_validators.push(*ty);
                        self.array_validators = Some(array_validators);
                    }
                }
            },
        }
    }

    pub fn to_token(&self) -> TokenStream {
        let ident = self.field.ident();

        // Nomal Tokens
        let normal_tokens = if !self.validators.is_empty() {
            let validators = TokenStream::from_iter(self.validators.clone());
            quote! (#validators)
        } else {
            quote! {}
        };

        // Optional Tokens
        let optional_tokens = if let Some(optional_validators) = &self.optional_validators {
            let option_ident = optional_validators.field.ident();
            let option_validators = optional_validators.to_token();
            quote!(
                if let Some(#option_ident) = #ident {
                    #option_validators
                }
            )
        } else {
            quote!()
        };

        // Array Tokens
        let array_tokens = if let Some(array_validators) = &self.array_validators {
            let array_ident = array_validators.field.ident();
            let array_validators = array_validators.to_token();
            quote!(
                for #array_ident in #ident {
                    #array_validators
                }
            )
        } else {
            quote!()
        };

        quote!(
            #normal_tokens
            #optional_tokens
            #array_tokens
        )
    }

    pub fn generate_token(&self) -> TokenStream {
        let field_ident = self.field.ident();
        let validation = self.to_token();
        quote!(
            let #field_ident = &self.#field_ident;
            #validation
        )
    }
}
