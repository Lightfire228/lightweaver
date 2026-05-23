use proc_macro::TokenStream;
use quote::quote;
use syn::{Data};

pub fn impl_obj_try_from(ast: &syn::DeriveInput) -> TokenStream {
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

            impl<'gc> TryFrom<#enum_name<'gc>> for #member_type {
                type Error = ();

                fn try_from(value: #enum_name<'gc>) -> Result<Self, Self::Error> {
                    match value {
                        #enum_name::#member_name(value) => Ok(value),
                        _                               => Err(()),
                    }
                }
            }

            impl<'gc> TryFrom<Obj<'gc>> for #member_type {
                type Error = ();

                fn try_from(value: Obj<'gc>) -> Result<Self, Self::Error> {
                    value.type_.try_into()
                }
            }

            impl<'a, 'gc: 'a> TryFrom<&'a #enum_name<'gc>> for &'a #member_type {
                type Error = ();

                fn try_from(value: &'a #enum_name<'gc>) -> Result<Self, Self::Error> {
                    match value {
                        #enum_name::#member_name(value) => Ok(value),
                        _                               => Err(()),
                    }
                }
            }

            impl<'a, 'gc: 'a> TryFrom<&'a Obj<'gc>> for &'a #member_type {
                type Error = ();

                fn try_from(value: &'a Obj<'gc>) -> Result<Self, Self::Error> {
                    (&value.type_).try_into()
                }
            }

            impl<'a, 'gc: 'a> TryFrom<&'a mut #enum_name<'gc>> for &'a mut #member_type {
                type Error = ();

                fn try_from(value: &'a mut #enum_name<'gc>) -> Result<Self, Self::Error> {
                    match value {
                        #enum_name::#member_name(value) => Ok(value),
                        _                               => Err(()),
                    }
                }
            }

            impl<'a, 'gc: 'a> TryFrom<&'a mut Obj<'gc>> for &'a mut #member_type {
                type Error = ();

                fn try_from(value: &'a mut Obj<'gc>) -> Result<Self, Self::Error> {
                    (&mut value.type_).try_into()
                }
            }

            impl<'gc> From<#member_type> for #enum_name<'gc> {
                fn from(value: #member_type) -> Self {
                    #enum_name::#member_name(value)
                }
            }
        }
    });

    generated.flat_map(|g| TokenStream::from(g)).collect()

}
