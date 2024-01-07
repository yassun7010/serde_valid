use crate::validate::{
    MetaListValidation, MetaNameValueCustomMessage, MetaNameValueValidation, MetaPathCustomMessage,
    MetaPathValidation,
};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;

use crate::validate::MetaListCustomMessage;

pub fn object_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::Object(
        ::serde_valid::validation::ObjectErrors::new(
            __rule_vec_errors,
            __property_vec_errors_map
                .into_iter()
                .map(|(field, errors)| {
                    let mut __field_items_errors = vec![];
                    let mut __field_properties_errors = None;
                    let mut __field_errors: ::serde_valid::validation::VecErrors = errors
                        .into_iter()
                        .filter_map(|error| match error {
                            ::serde_valid::validation::Error::Items(__array_errors) => {
                                __field_items_errors.push(__array_errors);
                                None
                            }
                            ::serde_valid::validation::Error::Properties(__object_errors) => {
                                __field_properties_errors = Some(__object_errors);
                                None
                            }
                            _ => Some(error),
                        })
                        .collect();

                    if let Some(__object_errors) = __field_properties_errors {
                        __field_errors.extend(__object_errors.errors);

                        (
                            field,
                            ::serde_valid::validation::Errors::Object(
                                ::serde_valid::validation::ObjectErrors::new(
                                    __field_errors,
                                    __object_errors.properties,
                                ),
                            ),
                        )
                    } else if !__field_items_errors.is_empty() {
                        let __array_errors = __field_items_errors
                            .into_iter()
                            .reduce(|a, b| a.merge(b))
                            .unwrap();
                        __field_errors.extend(__array_errors.errors);

                        (
                            field,
                            ::serde_valid::validation::Errors::Array(
                                ::serde_valid::validation::ArrayErrors::new(
                                    __field_errors,
                                    __array_errors.items,
                                ),
                            ),
                        )
                    } else {
                        (
                            field,
                            ::serde_valid::validation::Errors::NewType(__field_errors),
                        )
                    }
                })
                .collect()
        )
    ))
}

pub fn array_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::Array(
        ::serde_valid::validation::ArrayErrors::new(
            __rule_vec_errors,
            __item_vec_errors_map
                .into_iter()
                .map(|(index, errors)| {
                    let mut __field_items_errors = vec![];
                    let mut __field_properties_errors = None;
                    let mut __field_errors: ::serde_valid::validation::VecErrors = errors
                        .into_iter()
                        .filter_map(|error| match error {
                            ::serde_valid::validation::Error::Items(__array_errors) => {
                                __field_items_errors.push(__array_errors);
                                None
                            }
                            ::serde_valid::validation::Error::Properties(__object_errors) => {
                                __field_properties_errors = Some(__object_errors);
                                None
                            }
                            _ => Some(error),
                        })
                        .collect();

                    if let Some(__object_errors) = __field_properties_errors {
                        __field_errors.extend(__object_errors.errors);

                        (
                            index,
                            ::serde_valid::validation::Errors::Object(
                                ::serde_valid::validation::ObjectErrors::new(
                                    __field_errors,
                                    __object_errors.properties,
                                ),
                            ),
                        )
                    } else if !__field_items_errors.is_empty() {
                        let __array_errors = __field_items_errors
                            .into_iter()
                            .reduce(|a, b| a.merge(b))
                            .unwrap();
                        __field_errors.extend(__array_errors.errors);

                        (
                            index,
                            ::serde_valid::validation::Errors::Array(
                                ::serde_valid::validation::ArrayErrors::new(
                                    __field_errors,
                                    __array_errors.items,
                                ),
                            ),
                        )
                    } else {
                        (
                            index,
                            ::serde_valid::validation::Errors::NewType(__field_errors),
                        )
                    }
                })
                .collect()
        )
    ))
}

pub fn new_type_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::NewType(
        __rule_vec_errors
            .into_iter()
            .chain(__item_vec_errors_map.remove(&0).unwrap().into_iter())
            .collect()
    ))
}

#[derive(Debug)]
pub struct Error(syn::Error);

impl Error {
    fn new<Message: Into<String>>(span: proc_macro2::Span, message: Message) -> Self {
        Self(syn::Error::new(span, message.into()))
    }

    #[allow(dead_code)]
    pub fn macro_debug<Message: Into<String>>(span: proc_macro2::Span, message: Message) -> Self {
        Error::new(span, message)
    }

    pub fn unit_struct_not_support(input: &syn::DeriveInput) -> Self {
        Self::new(
            input.span(),
            "#[derive(Validate)] does not support Unit Struct.",
        )
    }

    pub fn union_not_support(input: &syn::DeriveInput) -> Self {
        Self::new(input.span(), "#[derive(Validate)] does not support Union.")
    }

    pub fn rule_allow_function_call_or_closure(span: impl Spanned) -> Self {
        Self::new(span.span(), "#[rule(???)] allows function call or closure.")
    }

    pub fn rule_allow_single_function(meta: &crate::types::NestedMeta) -> Self {
        Self::new(meta.span(), "#[rule(???)] allows single function.")
    }

    pub fn rule_need_arguments(path: &syn::Path) -> Self {
        Self::new(path.span(), "`rule` function needs arguments.")
    }

    pub fn rule_args_parse_error(metalist: &syn::MetaList, error: &syn::Error) -> Self {
        Self::new(
            metalist.span(),
            format!("#[rule(???)] parse error: {error}"),
        )
    }

    pub fn rule_args_allow_field_name(
        rule_fn_name_path: &syn::Path,
        meta: &crate::types::NestedMeta,
    ) -> Self {
        let rule_fn_name = quote!(#rule_fn_name_path).to_string();
        Self::new(
            meta.span(),
            format!("#[rule({rule_fn_name}(???, ...))] allows field name only."),
        )
    }

    pub fn rule_args_allow_field_index(
        rule_fn_name_path: &syn::Path,
        meta: &crate::types::NestedMeta,
    ) -> Self {
        let rule_fn_name = quote!(#rule_fn_name_path).to_string();
        Self::new(
            meta.span(),
            format!("#[rule({rule_fn_name}(???, ...))] allows field index only."),
        )
    }

    pub fn rule_named_clousure_input(meta: &syn::Pat) -> Self {
        Self::new(meta.span(), "Inputs of closure allows filed name only.")
    }

    pub fn rule_unnamed_clousure_input(meta: &syn::Pat) -> Self {
        Self::new(
            meta.span(),
            "Inputs of closure allows field index (like _0, _1, etc...) only.",
        )
    }

    pub fn validate_meta_name_value_not_support(name_value: &syn::MetaNameValue) -> Self {
        Self::new(
            name_value.span(),
            "#[validate = ???] format does not support.",
        )
    }

    pub fn meta_path_validation_need_value(path: &syn::Path, validation_type: &str) -> Self {
        Self::new(
            path.span(),
            format!("#[validate({validation_type}(???))] needs validation path."),
        )
    }

    pub fn meta_path_custom_message_need_value(
        path: &syn::Path,
        custom_message_type: &str,
    ) -> Self {
        Self::new(
            path.span(),
            format!("#[validate(..., {custom_message_type}(???))] needs custom message path."),
        )
    }

    pub fn meta_list_validation_need_value(path: &syn::Path, validation_type: &str) -> Self {
        Self::new(
            path.span(),
            format!("#[validate({validation_type}(???, ...))] needs validation list."),
        )
    }

    pub fn meta_list_custom_message_need_value(
        path: &syn::Path,
        custom_message_type: &str,
    ) -> Self {
        Self::new(
            path.span(),
            format!("#[validate(..., {custom_message_type}(???, ...))] needs custom message list."),
        )
    }

    pub fn meta_name_value_validation_need_value(path: &syn::Path, validation_type: &str) -> Self {
        Self::new(
            path.span(),
            format!("#[validate({validation_type} = ???)] needs validation value."),
        )
    }

    pub fn meta_name_value_custom_message_need_value(
        path: &syn::Path,
        validation_type: &str,
    ) -> Self {
        Self::new(
            path.span(),
            format!("#[validate(..., {validation_type} = ???)] needs custom message value."),
        )
    }

    pub fn validate_attribute_parse_error(attribute: &syn::Attribute, error: &syn::Error) -> Self {
        Self::new(
            attribute.span(),
            format!("#[validate] parse error: {error}"),
        )
    }

    pub fn validate_type_required_error(attribute: &syn::Attribute) -> Self {
        let filterd_candidates: Vec<&str> = (MetaPathValidation::iter().map(|x| x.name()))
            .chain(MetaNameValueValidation::iter().map(|x| x.name()))
            .chain(MetaListValidation::iter().map(|x| x.name()))
            .collect::<Vec<_>>();

        Self::new(
            attribute.meta.span(),
            format!("#[validate(???)] needs validation type. Is it one of the following?\n{filterd_candidates:#?}"),
        )
    }

    pub fn unknown_validation_type(path: &syn::Path, unknown: &str) -> Self {
        let candidates = &(MetaPathValidation::iter().map(|x| x.name()))
            .chain(MetaListValidation::iter().map(|x| x.name()))
            .chain(MetaNameValueValidation::iter().map(|x| x.name()))
            .collect::<Vec<_>>();

        let filterd_candidates =
            did_you_mean(unknown, candidates).unwrap_or_else(|| candidates.to_vec());

        Self::new(
            path.span(),
            format!("`{unknown}` is unknown validation type. Is it one of the following?\n{filterd_candidates:#?}"),
        )
    }

    pub fn unknown_custom_message_type(path: &syn::Path, unknown: &str) -> Self {
        let candidates = &(MetaPathCustomMessage::iter().map(|x| x.name()))
            .chain(MetaListCustomMessage::iter().map(|x| x.name()))
            .chain(MetaNameValueCustomMessage::iter().map(|x| x.name()))
            .collect::<Vec<_>>();

        let filterd_candidates =
            did_you_mean(unknown, candidates).unwrap_or_else(|| candidates.to_vec());

        Self::new(
            path.span(),
            format!("`{unknown}` is unkown error message type. Is it one of the following?\n{filterd_candidates:#?}"),
        )
    }

    pub fn validate_enumerate_parse_error(metalist: &syn::MetaList, error: &syn::Error) -> Self {
        Self::new(
            metalist.span(),
            format!("#[validate(enumerate(???))] parse error: {error}"),
        )
    }

    pub fn validate_enumerate_need_item(path: &syn::Path) -> Self {
        Self::new(path.span(), "#[validate(enumerate(???))] needs items.")
    }

    pub fn validate_custom_need_function(span: impl Spanned) -> Self {
        Self::new(span.span(), "#[validate(custom(???))] needs function.")
    }

    pub fn validate_custom_tail_error(nested: &crate::types::NestedMeta) -> Self {
        Self::new(
            nested.span(),
            "#[validate(custom(???)] supports only 1 item.",
        )
    }

    pub fn custom_message_parse_error(ident: &syn::Ident, error: &syn::Error) -> Self {
        Self::new(
            ident.span(),
            format!("#[validate(..., {ident})] parse error: {error}"),
        )
    }

    pub fn message_fn_need_item(path: &syn::Path) -> Self {
        Self::new(
            path.span(),
            "#[validate(..., message_fn(???))] needs function.",
        )
    }

    pub fn message_fn_allow_name_path(nested_meta: &crate::types::NestedMeta) -> Self {
        Self::new(
            nested_meta.span(),
            "#[validate(..., message_fn(???))] allows only function name path.",
        )
    }

    pub fn message_fn_tail_error(nested_meta: &crate::types::NestedMeta) -> Self {
        Self::new(
            nested_meta.span(),
            "#[validate(..., message_fn(???))] allows only 1 item.",
        )
    }

    #[cfg(feature = "fluent")]
    pub fn fluent_need_item(message_type: &MetaListCustomMessage, path: &syn::Path) -> Self {
        Self::new(
            path.span(),
            format!("`{}` needs items.", message_type.name()),
        )
    }

    #[cfg(feature = "fluent")]
    pub fn fluent_allow_key(
        message_type: &MetaListCustomMessage,
        nested_meta: &crate::types::NestedMeta,
    ) -> Self {
        Self::new(
            nested_meta.span(),
            format!(
                "#[validate(..., {}(???, ...))] allows only fluent key str",
                message_type.name()
            ),
        )
    }

    #[cfg(feature = "fluent")]
    pub fn fluent_allow_args(
        message_type: &MetaListCustomMessage,
        nested_meta: &crate::types::NestedMeta,
    ) -> Self {
        Self::new(
            nested_meta.span(),
            format!(
                "#[validate(..., {}(..., ???))] allows only fluent args key value.",
                message_type.name()
            ),
        )
    }

    pub fn literal_only(span: impl Spanned) -> Self {
        Self::new(span.span(), "Allow literal only.")
    }

    pub fn numeric_literal_only(lit: &syn::Lit) -> Self {
        Self::new(lit.span(), "Allow numeric literal only.")
    }

    pub fn str_literal_only(lit: &syn::Lit) -> Self {
        Self::new(lit.span(), "Allow str literal only.")
    }

    pub fn literal_not_support(lit: &syn::Lit) -> Self {
        Self::new(lit.span(), "Literal not support.")
    }

    pub fn closure_not_support(closure: &syn::ExprClosure) -> Self {
        Self::new(
            closure.or1_token.span(),
            format!("Closure not support. {}", closure.to_token_stream()),
        )
    }

    pub fn too_many_list_items(nested_meta: &syn::Meta) -> Self {
        Self::new(nested_meta.span(), "Too many list items.")
    }

    pub fn to_compile_error(&self) -> TokenStream {
        self.0.to_compile_error()
    }
}

fn did_you_mean<'a, T, I>(unknown: &'a str, candidates: I) -> Option<Vec<&'a str>>
where
    T: AsRef<str> + 'a,
    I: IntoIterator<Item = &'a T>,
{
    let mut filterd = candidates
        .into_iter()
        .map(|candidate| {
            (
                ::strsim::jaro_winkler(unknown, candidate.as_ref()),
                candidate.as_ref(),
            )
        })
        .filter(|(confidence, _)| *confidence > 0.8)
        .collect::<Vec<_>>();

    if filterd.is_empty() {
        None
    } else {
        filterd.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        Some(
            filterd
                .into_iter()
                .map(|(_, candidate)| candidate)
                .collect(),
        )
    }
}

pub type Errors = Vec<Error>;

pub fn to_compile_errors(errors: Errors) -> TokenStream {
    let compile_errors = errors.iter().map(Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
