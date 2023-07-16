//! #From
//!
//! Derivable macros for from-like traits. Assumes use of the new validated pattern.

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::utils;

pub(crate) fn implement_tryfrom(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);
    let generated = quote! {
        #[automatically_derived]
        impl ::core::convert::TryFrom<#wrapped> for #name {
            type Error = input_validator::error::ValidationFailedError;

            #[inline]
            fn try_from(value: #wrapped) -> Result<Self, Self::Error> {
                #name::new(value).ok_or(input_validator::error::ValidationFailedError)
            }
        }
    };

    generated.into()
}

pub(crate) fn implement_tryfromstr(ast: &DeriveInput) -> TokenStream {
    let (name, _) = utils::get_struct_info(ast);
    let generated = quote! {
        #[automatically_derived]
        impl ::core::convert::TryFrom<&str> for #name {
            type Error = input_validator::error::ValidationFailedError;

            #[inline]
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                #name::try_from(value.to_owned())
            }
        }
    };

    generated.into()
}
