use crate::{extract_name_values, get_description};
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
                            .ok_or(entrance::ErrorKind::InvalidNumberOfArguments)?
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

impl Parse for ArgumentFields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fields: Punctuated<_, Token![,]> = input.parse_terminated(syn::Field::parse_named)?;
        let mut arguments = Vec::new();
        let mut prev_variable: Option<syn::Attribute> = None;
        for field in fields {
            if let Some(attr) = prev_variable {
                return Err(syn::Error::new_spanned(
                    attr,
                    "The \"variable_argument\" attribute is allowed only for the last field",
                ));
            }

            prev_variable = field
                .attrs
                .iter()
                .find(|attr| {
                    if let Some(meta) = attr.interpret_meta() {
                        if let syn::Meta::Word(ident) = meta {
                            return ident == "variable_argument";
                        }
                    }
                    false
                })
                .cloned();

            arguments.push(Field {
                ident: field.ident.unwrap(),
                description: get_description(&extract_name_values(&field.attrs)),
            });
        }

        let variable_argument = if prev_variable.is_some() {
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
