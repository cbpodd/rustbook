//! Procedural macro for deriving the IntoInner trait.
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn implement(ast: &DeriveInput) -> TokenStream {
    let type_name = crate::utils::get_name(ast);
    let wrapped_type = crate::utils::get_single_wrapped_field(ast);
    let generated = quote! {
        impl IntoInner for #type_name {
            type Inner = #wrapped_type;

            fn into_inner(self) -> Self::Inner {
                self.0
            }
        }
    };

    generated.into()
}
