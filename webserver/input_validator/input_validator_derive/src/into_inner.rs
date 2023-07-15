// into_inner.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::utils;

pub(crate) fn implement(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = utils::get_struct_info(ast);
    let generated = quote! {
        #[automatically_derived]
        impl input_validator::IntoInner for #name {
            type Inner = #wrapped;

            #[inline]
            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }
    };

    generated.into()
}
