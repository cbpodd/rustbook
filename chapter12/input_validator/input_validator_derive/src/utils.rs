//! Utilities for the input validator deriving functions.

use proc_macro::TokenStream;
use syn::{DeriveInput, Ident, Type};

pub(crate) fn get_name(ast: &DeriveInput) -> &Ident {
    &ast.ident
}

pub(crate) fn get_single_wrapped_field(ast: &DeriveInput) -> &Type {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
        ..
    }) = &ast.data
    {
        unnamed
            .first()
            .map(|f| &f.ty)
            .expect("Expected a single unnamed field")
    } else {
        panic!("Expected a single unnamed field")
    }
}

pub(crate) fn parse_ast(input: TokenStream) -> DeriveInput {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    syn::parse(input).expect("Parsing syntax tree should not fail")
}
