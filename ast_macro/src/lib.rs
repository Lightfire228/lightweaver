use proc_macro::TokenStream;

mod ast_try_from;
mod obj_try_from;

#[proc_macro_derive(AstTryFrom)]
pub fn ast_try_from(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    ast_try_from::impl_ast_try_from(&ast)
}

#[proc_macro_derive(ObjTryFrom)]
pub fn obj_try_from(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    obj_try_from::impl_obj_try_from(&ast)
}
