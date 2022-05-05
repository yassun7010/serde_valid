use proc_macro2::TokenStream;
use quote::quote;

pub fn fields_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::Fields(__errors))
}

pub fn new_type_errors_tokens() -> TokenStream {
    quote!(::serde_valid::validation::Errors::NewType(
        __errors.remove("0").unwrap()
    ))
}

#[derive(Debug)]
pub struct Error(syn::Error);

impl Error {
    pub fn new<Message: Into<String>>(span: proc_macro2::Span, message: Message) -> Self {
        Self(syn::Error::new(span, message.into()))
    }

    pub fn new_literal_meta_item_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "#[validate(`literal`)] does not support.")
    }

    pub fn new_meta_name_value_item_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "#[validate = something...] does not support.")
    }

    pub fn new_meta_name_value_need_value_error(
        span: proc_macro2::Span,
        validation_type: &str,
    ) -> Self {
        Self::new(
            span,
            format!("#[validate({validation_type} = ???)] needs validation value."),
        )
    }

    pub fn new_meta_path_need_value_error(span: proc_macro2::Span, validation_type: &str) -> Self {
        Self::new(
            span,
            format!("#[validate({validation_type}(???))] needs validation path."),
        )
    }

    pub fn new_meta_list_need_value_error(span: proc_macro2::Span, validation_type: &str) -> Self {
        Self::new(
            span,
            format!("#[validate({validation_type}(???, ???, ...))] needs validation list."),
        )
    }

    pub fn new_attribute_parse_error(span: proc_macro2::Span, error: &syn::Error) -> Self {
        Self::new(span, format!("#[validate] parse error: {error}"))
    }

    pub fn new_attribute_required_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "#[validate(validation...)] needs validation.")
    }

    pub fn new_numeric_literal_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "Allow only numeric literal.")
    }

    pub fn new_str_literal_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "Allow only str literal.")
    }

    pub fn new_unknown_meta_error(
        span: proc_macro2::Span,
        unknown: &str,
        candidates: &[&str],
    ) -> Self {
        let filterd_candidates = did_you_mean(unknown, candidates).unwrap_or(candidates.to_vec());

        Self::new(
            span,
            format!("Unknown: `{unknown}`. Is it one of the following?\n{filterd_candidates:#?}"),
        )
    }

    pub fn new_message_fn_name_error(span: proc_macro2::Span) -> Self {
        Self::new(
            span,
            format!("#[validate(..., message_fn(???))] allow only function path."),
        )
    }

    pub fn new_message_fn_name_tail_error(span: proc_macro2::Span) -> Self {
        Self::new(span, format!("message_fn support only 1 item."))
    }

    pub fn new_literal_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "literal does not support.")
    }

    pub fn new_too_manyitems_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "Too many items.")
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

    if filterd.len() == 0 {
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
