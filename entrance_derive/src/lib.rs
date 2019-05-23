extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Args)]
pub fn args_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_args(&ast)
}

fn impl_args(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Not supported for any type except Struct"),
    };

    let body = match fields {
        syn::Fields::Named(fields) => {
            let named = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            quote! {
                Ok(Self {
                    #(
                        #named:
                            args.next()
                                .ok_or(entrance::Error::InvalidNumberOfArguments)?
                                .parse()?,
                    )*
                })
            }
        }
        _ => panic!("Not supported for any Struct without named fields"),
    };

    let gen = quote! {
        impl Args for #name {
            fn parse_from<I: std::iter::Iterator<Item = std::string::String>>(
                mut args: I
            ) -> entrance::Result<Self> {
                args.next(); // Drop the first element;
                #body
            }
        }
    };
    gen.into()
}
