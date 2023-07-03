// from.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn implement_tryfrom(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = crate::utils::get_struct_info(ast);
    let generated = quote! {
        impl TryFrom<#wrapped> for #name {
            type Error = #wrapped;

            fn try_from(value: #wrapped) -> Result<Self, Self::Error> {
                #name::new(value)
            }
        }
    };

    generated.into()
}
