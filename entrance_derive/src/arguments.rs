use crate::{extract_name_values, get_description};
use proc_macro::TokenStream;
use quote::quote;
use std::convert::{TryFrom, TryInto};

pub fn impl_arguments(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("Not supported for any type except Struct"),
    };

    let body = match fields {
        syn::Fields::Named(fields) => impl_for_named_fields(fields),
        _ => panic!("Not supported for any Struct without named fields"),
    };

    let gen = quote! {
        impl entrance::Arguments for #name {
            #body
        }
    };
    gen.into()
}

struct ArgumentItem<'a> {
    name: &'a syn::Ident,
    description: String,
}

impl<'a> TryFrom<&'a syn::Field> for ArgumentItem<'a> {
    type Error = &'static str;

    fn try_from(field: &'a syn::Field) -> Result<Self, Self::Error> {
        let name = field
            .ident
            .as_ref()
            .ok_or("The tuple structure is not available.")?;
        let name_value_attrs = extract_name_values(&field.attrs);
        let description = get_description(&name_value_attrs);

        Ok(Self { name, description })
    }
}

fn impl_for_named_fields(fields: &syn::FieldsNamed) -> impl quote::ToTokens {
    let arguments: Vec<ArgumentItem> = fields
        .named
        .iter()
        .map(|field| field.try_into().unwrap())
        .collect();

    let parse_arms = arguments.iter().map(|argument| {
        let name = &argument.name;
        quote! {
            #name:
                entrance::parse_argument(
                    args
                        .next()
                        .ok_or(entrance::ErrorKind::InvalidNumberOfArguments)?
                )?,
        }
    });
    let parse_impl = quote! {
        fn parse<I: std::iter::Iterator<Item = std::string::String>>(
            args: &mut I
        ) -> entrance::Result<Self> {
            Ok(Self {
                #(
                    #parse_arms
                )*
            })
        }
    };

    let num_variables = arguments.len();
    let names = arguments.iter().map(|argument| &argument.name);
    let descriptions = arguments.iter().map(|argument| &argument.description);
    let spec_impl = quote! {
        fn spec() -> &'static [entrance::Arg] {
            const ARGS: [entrance::Arg; #num_variables] = [
                #(
                    entrance::Arg {
                        name: stringify!(#names),
                        description: #descriptions,
                    },
                )*
            ];
            &ARGS
        }
    };

    let var_spec_impl = quote! {
        fn var_spec() -> std::option::Option<entrance::Arg> {
            None
        }
    };

    quote! {
        #parse_impl

        #spec_impl

        #var_spec_impl
    }
}
