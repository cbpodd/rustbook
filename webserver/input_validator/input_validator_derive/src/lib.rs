//! # Input Validator Derivations
//!
//! Derivable macros for the input validator library.

// TODO: Add an error type as an attribute.
// TODO: Move these implementations into functional macros

use proc_macro::TokenStream;

mod from;
mod into_inner;
mod new;
mod utils;

/// Derive the `IntoInner`trait on the specified wrapper struct.
#[proc_macro_derive(IntoInner)]
pub fn into_inner_derive(input: TokenStream) -> TokenStream {
    let ast = utils::parse_ast(input);
    into_inner::implement(&ast)
}

/// Derive the `NewValidated`trait on the specified wrapper struct.
#[proc_macro_derive(NewValidated)]
pub fn new_validated_derive(input: TokenStream) -> TokenStream {
    let ast = utils::parse_ast(input);
    new::implement_validated(&ast)
}

/// Derive the `NewSanitized`trait on the specified wrapper struct.
#[proc_macro_derive(NewSanitized)]
pub fn new_sanitized_derive(input: TokenStream) -> TokenStream {
    let ast = utils::parse_ast(input);
    new::implement_sanitized(&ast)
}

/// Derive the `NewSanitizedValidated`trait on the specified wrapper struct.
#[proc_macro_derive(NewSanitizedValidated)]
pub fn new_sanitized_validated_derive(input: TokenStream) -> TokenStream {
    let ast = utils::parse_ast(input);
    new::implement_sanitized_validated(&ast)
}

/// Derive the `TryFrom`trait on the specified newtype struct.
#[proc_macro_derive(TryFrom)]
pub fn new_validated_tryfrom(input: TokenStream) -> TokenStream {
    let ast = utils::parse_ast(input);
    from::implement_tryfrom(&ast)
}

/// This macro implements `TryFrom<&str>` on a newtype struct that
/// already has `TryFrom<String>` implemented.
#[proc_macro_derive(TryFromStr)]
pub fn new_validated_tryfromstr(input: TokenStream) -> TokenStream {
    let ast = utils::parse_ast(input);
    from::implement_tryfromstr(&ast)
}
