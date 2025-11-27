use proc_macro ::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

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

#[proc_macro_attribute]
pub fn derive_all(_attr: TokenStream, input: TokenStream) -> TokenStream {

    let input: TokenStream2 = input.into();

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        #input
    }.into()
}
