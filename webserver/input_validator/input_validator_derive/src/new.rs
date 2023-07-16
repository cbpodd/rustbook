//! # New
//!
//! Derivable implementations of constructor functions for validated and/or
//! sanitized newtypes.

use crate::utils;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn implement_validated(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);

    let generated = quote! {
        #[automatically_derived]
        impl input_validator::NewValidated for #name {
            type Inner = #wrapped;

            #[inline]
            fn new(raw_input: Self::Inner) -> Option<Self> {
                if !#name::is_valid_input(&raw_input) {
                    return None
                }

                Some(#name(raw_input))
            }
        }
    };

    generated.into()
}

pub(crate) fn implement_sanitized(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);
    let generated = quote! {
        #[automatically_derived]
        impl input_validator::NewSanitized for #name {
            type Inner = #wrapped;

            #[inline]
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
    let generated = quote! {
        #[automatically_derived]
        impl input_validator::NewSanitizedValidated for #name {
            type Inner = #wrapped;

            #[inline]
            fn new(raw_input: Self::Inner) -> Option<Self> {
                let sanitized_input = #name::sanitize_input(raw_input);

                if !#name::is_valid_input(&sanitized_input) {
                    return None
                }

                Some(#name(sanitized_input))
            }
        }
    };

    generated.into()
}
