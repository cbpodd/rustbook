// into_inner.rs

use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn implement(ast: &DeriveInput) -> TokenStream {
    let (name, wrapped) = crate::utils::get_struct_info(ast);
    let generated = quote! {
        impl IntoInner for #name {
            type Inner = #wrapped;

            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }
    };

    generated.into()
}