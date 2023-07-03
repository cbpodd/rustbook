//! # New
//!
//! Derivable implementations of constructor functions for validated and/or
//! sanitized newtypes.

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn implement_validated(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = crate::utils::get_struct_info(ast);
    let generated = quote! {
        impl NewValidated for #name {
            type Inner = #wrapped;

            fn new(raw_input: Self::Inner) -> Result<Self, Self::Inner> {
                if !#name::is_valid_input(&raw_input) {
                    return Err(raw_input);
                }

                Ok(#name(raw_input))
            }
        }
    };

    generated.into()
}

pub(crate) fn implement_sanitized(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = crate::utils::get_struct_info(ast);
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
    let (name, wrapped) = crate::utils::get_struct_info(ast);
    let generated = quote! {
        impl NewSanitizedValidated for #name {
            type Inner = #wrapped;

            fn new(raw_input: Self::Inner) -> Result<Self, Self::Inner> {
                let sanitized_input = #name::sanitize_input(raw_input);

                if !#name::is_valid_input(&sanitized_input) {
                    return Err(sanitized_input);
                }

                Ok(#name(sanitized_input))
            }
        }
    };

    generated.into()
}
