mod desyn;

use std::fmt::Formatter;

use proc_macro::TokenStream;

use syn::Type;

use crate::desyn::DeSyntax;

struct WType(Type);

impl std::fmt::Display for WType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.de_syntax(f)
    }
}

#[proc_macro]
pub fn test_type(input: TokenStream) -> TokenStream {
    let ty = syn::parse::<Type>(input).unwrap();
    eprintln!("{}", WType(ty));
    TokenStream::new()
}
