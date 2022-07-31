use crate::types::Field;
use proc_macro2::TokenStream;
use quote::quote;
use std::borrow::Cow;
use std::iter::FromIterator;

pub type Validator = TokenStream;

pub struct FieldValidators<'a, F: Field + Clone + 'a> {
    field: Cow<'a, F>,
    validators: Vec<Validator>,
}

impl<'a, F: Field + Clone> FieldValidators<'a, F> {
    pub fn new(field: Cow<'a, F>) -> Self {
        Self {
            field,
            validators: vec![],
        }
    }

    pub fn ident(&self) -> &syn::Ident {
        self.field.ident()
    }

    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }

    pub fn push(&mut self, validator: Validator) {
        self.validators.push(validator)
    }

    pub fn get_tokens(&self) -> Option<TokenStream> {
        if !self.validators.is_empty() {
            let validators = TokenStream::from_iter(self.validators.clone());
            Some(quote! (#validators))
        } else {
            None
        }
    }

    pub fn get_field_variable_token(&self) -> TokenStream {
        let field_ident = self.field.ident();
        let field_getter = self.field.getter_token();
        quote!(
            let #field_ident = &self.#field_getter;
        )
    }

    pub fn generate_tokens(&self) -> TokenStream {
        let normal_tokens = self.get_tokens();

        if normal_tokens.is_some() {
            let field_variable_token = self.get_field_variable_token();
            quote!(
                #field_variable_token
                #normal_tokens
            )
        } else {
            quote!()
        }
    }
}
