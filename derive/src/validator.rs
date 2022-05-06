mod array;
mod common;
mod generic;
mod meta;
mod numeric;
mod object;
mod string;

pub use crate::types::Field;
pub use generic::collect_rules;
pub use meta::extract_meta_validator;
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::iter::FromIterator;

pub enum Validator {
    Normal(TokenStream),
    Option(Box<Validator>),
    Array(Box<Validator>),
}

pub struct FieldValidators<'a, F: Field + Clone + 'a> {
    field: Cow<'a, F>,
    validators: Vec<TokenStream>,
    optional_validators: Option<Box<FieldValidators<'a, F>>>,
    array_validators: Option<Box<FieldValidators<'a, F>>>,
}

impl<'a, F: Field + Clone> FieldValidators<'a, F> {
    pub fn new(field: Cow<'a, F>) -> Self {
        Self {
            field,
            validators: vec![],
            optional_validators: None,
            array_validators: None,
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        self.field.ident()
    }

    pub fn push(&mut self, validator: Validator) {
        match validator {
            Validator::Normal(token) => self.validators.push(token),
            Validator::Option(ty) => match self.optional_validators.as_mut() {
                Some(optional_validator) => optional_validator.push(*ty),
                None => {
                    if let Some(field) = self.field.option_field() {
                        let mut option_validators = Box::new(Self::new(Cow::Owned(field)));
                        option_validators.push(*ty);
                        self.optional_validators = Some(option_validators);
                    }
                }
            },
            Validator::Array(ty) => match self.array_validators.as_mut() {
                Some(array_validator) => array_validator.push(*ty),
                None => {
                    if let Some(field) = self.field.array_field() {
                        let mut array_validators = Box::new(Self::new(Cow::Owned(field)));
                        array_validators.push(*ty);
                        self.array_validators = Some(array_validators);
                    }
                }
            },
        }
    }

    pub fn get_tokens(&self) -> Option<TokenStream> {
        let normal_tokens = self.normal_tokens();
        let optional_tokens = self.optional_tokens();
        let array_tokens = self.array_tokens();

        if normal_tokens.is_some() || optional_tokens.is_some() || array_tokens.is_some() {
            Some(quote!(
                #normal_tokens
                #optional_tokens
                #array_tokens
            ))
        } else {
            None
        }
    }

    pub fn to_tokens(&self) -> TokenStream {
        let normal_tokens = self.normal_tokens();
        let optional_tokens = self.optional_tokens();
        let array_tokens = self.array_tokens();

        quote!(
            #normal_tokens
            #optional_tokens
            #array_tokens
        )
    }

    pub fn generate_tokens(&self) -> TokenStream {
        let normal_tokens = self.normal_tokens();
        let optional_tokens = self.optional_tokens();
        let array_tokens = self.array_tokens();

        if normal_tokens.is_some() || optional_tokens.is_some() || array_tokens.is_some() {
            let field_ident = self.field.ident();
            let field_tokens = self.field.ident_tokens();
            quote!(
                let #field_ident = &self.#field_tokens;
                #normal_tokens
                #optional_tokens
                #array_tokens
            )
        } else {
            quote!()
        }
    }

    fn normal_tokens(&self) -> Option<TokenStream> {
        if !self.validators.is_empty() {
            let validators = TokenStream::from_iter(self.validators.clone());
            Some(quote! (#validators))
        } else {
            None
        }
    }

    fn optional_tokens(&self) -> Option<TokenStream> {
        if let Some(optional_validators) = &self.optional_validators {
            let ident = self.field.ident();
            let option_ident = optional_validators.field.ident();
            let option_validators = optional_validators.to_tokens();
            Some(quote!(
                if let Some(#option_ident) = #ident {
                    #option_validators
                }
            ))
        } else {
            None
        }
    }

    fn array_tokens(&self) -> Option<TokenStream> {
        if let Some(array_validators) = &self.array_validators {
            let ident = self.field.ident();
            let array_ident = array_validators.field.ident();
            let array_validators = array_validators.to_tokens();
            Some(quote!(
                for #array_ident in #ident {
                    #array_validators
                }
            ))
        } else {
            None
        }
    }
}
