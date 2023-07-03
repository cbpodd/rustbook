//! # New
//!
//! Derivable implementations of constructor functions for validated and/or
//! sanitized newtypes.

use crate::utils;
use proc_macro::{Ident, Span, TokenStream};
use quote::quote;
use syn::{DeriveInput, LitStr};

pub(crate) fn implement_validated(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);

    let attribute_error_type =
        utils::get_attribute_value(&ast.attrs, "error_type");
    let error_type = attribute_error_type.as_ref().unwrap_or(wrapped);

    let error_constructor =
        if let Some(error_type) = attribute_error_type.as_ref() {
            quote! { #error_type(raw_input) }
        } else {
            quote! { raw_input }
        };

    let generated = quote! {
        impl NewValidated for #name {
            type Inner = #wrapped;
            type Error = #error_type;

            fn new(raw_input: Self::Inner) -> Result<Self, Self::Error> {
                if !#name::is_valid_input(&raw_input) {
                    return Err(#error_constructor);
                }

                Ok(#name(raw_input))
            }
        }
    };

    generated.into()
}

pub(crate) fn implement_sanitized(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);
    let generated = quote! {
        impl NewSanitized for #name {
            type Inner = #wrapped;

            fn new(raw_input: Self::Inner) -> Self {
                let sanitized_input = #name::sanitize_input(raw_input);
                #name(sanitized_input)
            }
        }
    };

    generated.into()
}

pub(crate) fn implement_sanitized_validated(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);

    let attribute_error_type =
        utils::get_attribute_value(&ast.attrs, "error_type");
    let error_type = attribute_error_type.as_ref().unwrap_or(wrapped);

    let error_constructor =
        if let Some(error_type) = attribute_error_type.as_ref() {
            quote! { #error_type(sanitized_input) }
        } else {
            quote! { sanitized_input }
        };

    let generated = quote! {
        impl NewSanitizedValidated for #name {
            type Inner = #wrapped;
            type Error = #error_type;

            fn new(raw_input: Self::Inner) -> Result<Self, Self::Error> {
                let sanitized_input = #name::sanitize_input(raw_input);

                if !#name::is_valid_input(&sanitized_input) {
                    return Err(#error_constructor);
                }

                Ok(#name(sanitized_input))
            }
        }
    };

    generated.into()
}

pub(crate) fn implement_thiserror_wrapper(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);

    let attribute_error_name =
        utils::get_attribute_value(&ast.attrs, "error_name");
    let error_name = attribute_error_name
        .map_or("Error".to_owned(), |lit_str: LitStr| lit_str.value());

    let error_identifier =
        proc_macro2::Ident::new(&error_name, proc_macro2::Span::call_site());

    let generated = quote! {
        /// Error indicating the wrapper struct failed validation.
        #[derive(Debug, thiserror::Error)]
        #[error("Input for {} failed validation. Input: {0}", stringify!(#name))]
        pub struct #error_identifier(pub #wrapped);
    };

    generated.into()
}
