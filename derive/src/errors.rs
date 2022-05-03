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
        Self::new(span, "literal meta item does not support.")
    }

    pub fn new_name_value_meta_item_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "name value meta does not support.")
    }

    pub fn new_attribute_parse_error(span: proc_macro2::Span, error: &syn::Error) -> Self {
        Self::new(span, format!("attribute parse error: {error}"))
    }

    pub fn new_attribute_required_error(span: proc_macro2::Span) -> Self {
        Self::new(span, "it needs at least one validator")
    }

    pub fn new_path_meta_name_error(
        span: proc_macro2::Span,
        target: &str,
        candidates: &[&str],
    ) -> Self {
        if let Some(candidate) = did_you_mean(target, candidates) {
            Self::new_do_you_mean_name_error(span, target, &candidate)
        } else {
            Self::new(
                span,
                format!("path meta must be selected from {candidates:?}"),
            )
        }
    }

    fn new_do_you_mean_name_error(span: proc_macro2::Span, target: &str, candidate: &str) -> Self {
        Self::new(
            span,
            format!("Unknown name: `{target}`. Do you mean `{candidate}`?"),
        )
    }

    pub fn to_compile_error(&self) -> TokenStream {
        self.0.to_compile_error()
    }
}

fn did_you_mean<'a, T, I>(field: &str, alternates: I) -> Option<String>
where
    T: AsRef<str> + 'a,
    I: IntoIterator<Item = &'a T>,
{
    let mut candidate: Option<(f64, &str)> = None;
    for pv in alternates {
        let confidence = ::strsim::jaro_winkler(field, pv.as_ref());
        if confidence > 0.8 && (candidate.is_none() || (candidate.as_ref().unwrap().0 < confidence))
        {
            candidate = Some((confidence, pv.as_ref()));
        }
    }
    candidate.map(|(_, candidate)| candidate.into())
}

pub type Errors = Vec<Error>;

pub fn to_compile_errors(errors: Errors) -> TokenStream {
    let compile_errors = errors.iter().map(Error::to_compile_error);
    quote!(#(#compile_errors)*)
}
