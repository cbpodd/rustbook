//! Derivable macros for the input validator library.

use proc_macro::TokenStream;

mod into_inner;
mod utils;

/// Derive a hello macro
#[proc_macro_derive(IntoInner)]
pub fn into_inner_derive(input: TokenStream) -> TokenStream {
    let ast = utils::parse_ast(input);
    into_inner::implement(&ast)
}
