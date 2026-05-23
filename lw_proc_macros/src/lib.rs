use proc_macro ::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Meta, parse_macro_input, punctuated::Punctuated};

mod ast_try_from;
mod obj_try_from;
mod parser_logger;

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
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Collect)]
        #[collect(no_drop)]
        #input
    }.into()
}

/// This is highly coupled to the implementation details of Parser
/// and should only be used in its impl block with access to its private members
#[proc_macro_attribute]
pub fn parser_logger(args: TokenStream, input: TokenStream) -> TokenStream {

    let args = parse_macro_input!(args with Punctuated::<Meta, syn::Token![,]>::parse_terminated);

    let func = syn::parse(input).unwrap();

    let no_children = args
        .first()
        .map  (|a| a
            .path       ()
            .get_ident  ()
            .map_or_else(|| String::new(), |i| i.to_string())
        )
        .is_some_and(|a| a == "no_children")
    ;

    if no_children {
        parser_logger::parser_logger_no_children(&func).into()
    }
    else {
        parser_logger::parser_logger(&func).into()
    }

}
