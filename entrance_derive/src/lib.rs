extern crate proc_macro;

mod options;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Arguments)]
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
                                .ok_or(entrance::ArgumentError::InvalidNumberOfArguments)?
                                .parse()?,
                    )*
                })
            }
        }
        _ => panic!("Not supported for any Struct without named fields"),
    };

    let spec_body = match fields {
        syn::Fields::Named(fields) => {
            let named = fields.named.iter().map(|f| f.ident.as_ref().unwrap());
            let num_variables = fields.named.len();
            quote! {
                const ARGS: [entrance::Arg; #num_variables] = [
                    #(
                        entrance::Arg::new(stringify!(#named), ""),
                    )*
                ];
                &ARGS
            }
        }
        _ => panic!("Not supported for any Struct without named fields"),
    };

    let gen = quote! {
        impl Arguments for #name {
            fn parse_from<I: std::iter::Iterator<Item = std::string::String>>(
                mut args: I
            ) -> entrance::Result<Self> {
                #body
            }

            fn spec() -> &'static [entrance::Arg] {
                #spec_body
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Options)]
pub fn optionss_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    options::impl_options(&ast)
}
