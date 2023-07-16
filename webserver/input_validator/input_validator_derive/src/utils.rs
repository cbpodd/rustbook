use proc_macro::TokenStream;
use syn::{DeriveInput, Ident, Type};

pub(crate) fn parse_ast(input: TokenStream) -> DeriveInput {
    syn::parse(input).expect("Parsing syntax tree should not fail")
}

pub(crate) fn get_struct_info(ast: &DeriveInput) -> (&Ident, &Type) {
    let name = get_name(ast);
    let wrapped_type = get_single_wrapped_field(ast);
    (name, wrapped_type)
}

fn get_name(ast: &DeriveInput) -> &Ident {
    &ast.ident
}

fn get_single_wrapped_field(ast: &DeriveInput) -> &Type {
    const ERROR_MESSAGE: &str = "Expected a single unnamed field";
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }),
        ..
    }) = &ast.data
    {
        unnamed.first().map(|f| &f.ty).expect(ERROR_MESSAGE)
    } else {
        panic!("{ERROR_MESSAGE}")
    }
}
