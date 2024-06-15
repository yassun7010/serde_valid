use crate::warning::Warning;
use proc_macro2::TokenStream;

#[derive(Debug, Clone)]
pub struct OutputStream {
    pub output: TokenStream,
    pub warnings: Vec<Warning>,
}

impl OutputStream {
    pub fn new() -> Self {
        Self {
            output: TokenStream::new(),
            warnings: vec![],
        }
    }

    #[allow(unused)]
    pub fn extend_warnings(&mut self, warnings: Vec<Warning>) -> &mut Self {
        self.warnings.extend(warnings);
        self
    }
}
