use std::{hash::Hash, str::FromStr};

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::{quote_spanned, ToTokens};

#[derive(Debug, Clone)]
pub enum Warning {
    Deprecated {
        ident: syn::Ident,
        note: String,
        span: Span,
    },
}

impl Hash for Warning {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::Deprecated { ident, note, .. } => {
                ident.hash(state);
                note.hash(state);
            }
        }
    }
}

impl std::cmp::PartialEq for Warning {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Deprecated {
                    ident: ident1,
                    note: note1,
                    ..
                },
                Self::Deprecated {
                    ident: ident2,
                    note: note2,
                    ..
                },
            ) => ident1 == ident2 && note1 == note2,
        }
    }
}

impl std::cmp::Eq for Warning {}

impl Warning {
    pub fn new_rule_deprecated(ident: &syn::Ident, span: Span) -> Self {
        Self::Deprecated {
            ident: ident.clone(),
            note: "#[rule(...)] is deprecated, use #[validate(custom(...)))] instead".to_string(),
            span,
        }
    }

    pub fn add_index(&self, index: usize) -> Self {
        match self {
            Self::Deprecated { ident, note, span } => Self::Deprecated {
                ident: syn::Ident::new(&format!("{}_{}", ident, index), ident.span()),
                note: note.clone(),
                span: *span,
            },
        }
    }
}

impl ToTokens for Warning {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Deprecated { ident, note, span } => {
                let func_name = TokenStream::from_str(&format!(
                    "__{}_warning",
                    ident.to_string().to_lowercase()
                ))
                .unwrap();

                quote_spanned!(*span =>
                    #[deprecated(note = #note)]
                    #[allow(clippy::let_unit_value)]
                    fn #func_name() {
                        #[deprecated(note = #note)]
                        #[allow(non_upper_case_globals)]
                        const _deprecated: () = ();
                        let _ = _deprecated;
                    }
                )
                .to_tokens(tokens)
            }
        }
    }
}
