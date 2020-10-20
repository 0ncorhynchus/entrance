use crate::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Token};

pub struct ArgumentsInput {
    _struct_token: Token![struct],
    ident: syn::Ident,
    _brace_token: syn::token::Brace,
    fields: ArgumentFields,
}

impl ArgumentsInput {
    pub fn gen(&self) -> TokenStream {
        let parse_arms = self.fields.arguments.iter().map(|argument| {
            let ident = &argument.ident;
            quote! {
                #ident:
                    entrance::parse_argument(
                        args
                            .next()
                            .ok_or(entrance::Error::InvalidNumberOfArguments)?
                    )?,
            }
        });

        let parse_var_arg = self.fields.variable_argument.as_ref().map(|argument| {
            let ident = &argument.ident;
            quote! {
                #ident: entrance::parse_variable_argument(args)?,
            }
        });

        let names = self.fields.arguments.iter().map(|argument| &argument.ident);
        let descriptions = self
            .fields
            .arguments
            .iter()
            .map(|argument| &argument.description);

        let var_spec_impl = if let Some(argument) = &self.fields.variable_argument {
            let ident = &argument.ident;
            let description = &argument.description;
            quote! {
                Some(entrance::Arg {
                    name: stringify!(#ident),
                    description: #description,
                })
            }
        } else {
            quote! { None }
        };

        let ident = &self.ident;
        let num_arguments = self.fields.arguments.len();
        (quote! {
            impl entrance::Arguments for #ident {
                fn parse<I: std::iter::Iterator<Item = std::string::String>>(
                    args: &mut I
                ) -> entrance::Result<Self> {
                    Ok(Self {
                        #(
                            #parse_arms
                        )*
                        #parse_var_arg
                    })
                }

                fn spec() -> &'static [entrance::Arg] {
                    const ARGS: [entrance::Arg; #num_arguments] = [
                        #(
                            entrance::Arg{
                                name: stringify!(#names),
                                description: #descriptions,
                            },
                        )*
                    ];
                    &ARGS
                }

                fn var_spec() -> std::option::Option<entrance::Arg> {
                    #var_spec_impl
                }
            }
        })
        .into()
    }
}

impl Parse for ArgumentsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![struct]) {
            let content;
            let struct_token = input.parse()?;
            let ident = input.parse()?;
            let brace_token = braced!(content in input);
            let fields = content.parse()?;
            Ok(ArgumentsInput {
                _struct_token: struct_token,
                ident,
                _brace_token: brace_token,
                fields,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

struct Field {
    ident: syn::Ident,
    description: String,
}

struct ArgumentFields {
    arguments: Vec<Field>,
    variable_argument: Option<Field>,
}

fn extract_arguments_attrs(
    attrs: &[syn::Attribute],
) -> (Option<(syn::Meta, String)>, Option<syn::Meta>) {
    let mut description = None;
    let mut variadic = None;

    let attrs = extract_attributes(attrs);
    for (meta, attr) in attrs {
        match attr {
            Attribute::Description(desc) => {
                if description.is_some() {
                    panic!("description attributes are duplicated");
                }
                description = Some((meta, desc));
            }
            Attribute::Variadic => {
                variadic = Some(meta);
            }
            _ => {
                panic!("Invalid argument is given");
            }
        }
    }

    (description, variadic)
}

impl Parse for ArgumentFields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut arguments = Vec::new();
        let mut prev_variadic: Option<syn::Meta> = None;

        let fields: Punctuated<_, Token![,]> = input.parse_terminated(syn::Field::parse_named)?;
        for field in fields {
            if let Some(meta) = prev_variadic {
                return Err(syn::Error::new_spanned(
                    meta,
                    "The \"variable_argument\" attribute is allowed only for the last field",
                ));
            }

            let (description, variadic) = extract_arguments_attrs(&field.attrs);
            if let Some(meta) = variadic {
                prev_variadic = Some(meta);
            }

            let description = if let Some((_, description)) = description {
                description
            } else {
                String::new()
            };

            arguments.push(Field {
                ident: field.ident.unwrap(),
                description,
            });
        }

        let variable_argument = if prev_variadic.is_some() {
            arguments.pop()
        } else {
            None
        };

        Ok(Self {
            arguments,
            variable_argument,
        })
    }
}
