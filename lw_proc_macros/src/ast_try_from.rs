use proc_macro::TokenStream;
use quote::quote;
use syn::{Data};

pub fn impl_ast_try_from(ast: &syn::DeriveInput) -> TokenStream {
    let enum_name = &ast.ident;

    let Data::Enum(data) = &ast.data else {
        panic!("Not an enum");
    };


    let variants: Vec<_> = data.variants.iter()
        .filter(|v| v.fields.iter().next().is_some())
        .map(|v| (
            &v.ident,
            v.fields
                .iter()
                .map(|f| &f.ty)
                .next()
                .unwrap()
        ))
        .collect()
    ;


    let generated = variants.iter().map(|v| {
        let member_name = v.0;
        let member_type = v.1;

        quote! {

            impl TryFrom<#enum_name> for #member_type {
                type Error = ();

                fn try_from(value: #enum_name) -> Result<Self, Self::Error> {
                    match value {
                        #enum_name::#member_name(value) => Ok(value),
                        _                               => Err(()),
                    }
                }
            }

            impl<'a> TryFrom<&'a #enum_name> for &'a #member_type {
                type Error = ();

                fn try_from(value: &'a #enum_name) -> Result<Self, Self::Error> {
                    match value {
                        #enum_name::#member_name(value) => Ok(value),
                        _                               => Err(()),
                    }
                }
            }

            impl<'a> TryFrom<&'a mut #enum_name> for &'a mut #member_type {
                type Error = ();

                fn try_from(value: &'a mut #enum_name) -> Result<Self, Self::Error> {
                    match value {
                        #enum_name::#member_name(value) => Ok(value),
                        _                               => Err(()),
                    }
                }
            }
        }
    });

    generated.flat_map(|g| TokenStream::from(g)).collect()

}
