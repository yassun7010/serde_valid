mod field;
mod single_ident_path;

pub use field::{Field, NamedField, UnnamedField};
use proc_macro2::TokenStream;
pub use single_ident_path::SingleIdentPath;

pub type CommaSeparatedTokenStreams = syn::punctuated::Punctuated<TokenStream, syn::token::Comma>;
pub type CommaSeparatedNestedMetas =
    syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>;
