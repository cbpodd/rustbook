use proc_macro::TokenStream;
use syn::{Attribute, DeriveInput, Ident, Type};

pub(crate) fn parse_ast(input: TokenStream) -> DeriveInput {
    syn::parse(input).expect("Parsing syntax tree should not fail")
}

pub(crate) fn get_struct_info(ast: &DeriveInput) -> (&Ident, &Type) {
    let name = get_name(ast);
    let wrapped_type = get_single_wrapped_field(ast);
    (name, wrapped_type)
}

pub(crate) fn get_attribute_value<AttributeType>(
    attrs: &[Attribute],
    name: &str,
) -> Option<AttributeType>
where
    AttributeType: syn::parse::Parse,
{
    attrs
        .iter()
        .find(|a| a.path.is_ident(name))
        .and_then(|attr| attr.parse_args::<AttributeType>().ok())
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
