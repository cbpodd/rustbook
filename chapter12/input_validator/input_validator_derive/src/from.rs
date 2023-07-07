//! #From
//!
//! Derivable macros for from-like traits. Assumes use of the new validated pattern.

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::utils;

pub(crate) fn implement_tryfrom(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);
    let attribute_error_type =
        utils::get_attribute_value(&ast.attrs, "error_type");
    let error_type = attribute_error_type.as_ref().unwrap_or(wrapped);
    let generated = quote! {
        #[automatically_derived]
        impl ::core::convert::TryFrom<#wrapped> for #name {
            type Error = #error_type;

            #[inline]
            fn try_from(value: #wrapped) -> Result<Self, Self::Error> {
                #name::new(value)
            }
        }
    };

    generated.into()
}

pub(crate) fn implement_tryfromstr(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);
    let attribute_error_type =
        utils::get_attribute_value(&ast.attrs, "error_type");
    let error_type = attribute_error_type.as_ref().unwrap_or(wrapped);
    let generated = quote! {
        #[automatically_derived]
        impl ::core::convert::TryFrom<&str> for #name {
            type Error = #error_type;

            #[inline]
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                #name::try_from(value.to_owned())
            }
        }
    };

    generated.into()
}
