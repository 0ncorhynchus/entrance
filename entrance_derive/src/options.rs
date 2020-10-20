use crate::*;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Token};

pub struct OptionsInput {
    _struct_token: Token![enum],
    ident: syn::Ident,
    _brace_token: syn::token::Brace,
    variants: Punctuated<OptionVariant, Token![,]>,
}

impl OptionsInput {
    pub fn gen(&self) -> TokenStream {
        let ident = &self.ident;
        let options: Vec<_> = self.variants.iter().collect();
        let long_option_arms = options.iter().map(|option| {
            let option = &option.ident;
            let long = get_long_option(option);
            quote! {
                #long => Ok(#ident::#option),
            }
        });
        let short_option_arms = options.iter().filter_map(|option| {
            let short = option.short?;
            let option = &option.ident;
            Some(quote! {
                #short => Ok(#ident::#option),
            })
        });
        let parse_lines = quote! {
            match option {
                entrance::OptionItem::Long(option) => {
                    match option.as_str() {
                        #(
                            #long_option_arms
                        )*
                        _ => {
                            Err(entrance::Error::InvalidOption)
                        }
                    }
                }
                entrance::OptionItem::Short(o) => {
                    match o {
                        #(
                            #short_option_arms
                        )*
                        _ => {
                            Err(entrance::Error::InvalidOption)
                        }
                    }
                }
            }
        };

        let informative_arms = options.iter().map(|option| {
            let is_informative = option.is_informative;
            let option = &option.ident;
            quote! {
                Self::#option => #is_informative
            }
        });

        let idents = options.iter().map(|option| get_long_option(&option.ident));
        let num_options = options.len();
        let descriptions = options.iter().map(|option| &option.description);
        let shorts = options.iter().map(|option| option_to_tokens(option.short));
        (quote! {
            impl entrance::Options for #ident {
                fn parse(option: entrance::OptionItem) -> entrance::Result<Self> {
                    #parse_lines
                }

                fn is_informative(&self) -> bool {
                    match self {
                        #(
                            #informative_arms,
                        )*
                    }
                }

                fn spec() -> &'static [entrance::Opt] {
                    static OPTS: [entrance::Opt; #num_options] = [
                        #(
                            entrance::Opt {
                                long: #idents,
                                short: #shorts,
                                description: #descriptions,
                            },
                        )*
                    ];
                    &OPTS
                }
            }
        })
        .into()
    }
}

impl Parse for OptionsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![enum]) {
            let content;
            let struct_token = input.parse()?;
            let ident = input.parse()?;
            let brace_token = braced!(content in input);
            let variants = content.parse_terminated(OptionVariant::parse)?;
            Ok(Self {
                _struct_token: struct_token,
                ident,
                _brace_token: brace_token,
                variants,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

struct OptionAttribute {
    short: Option<char>,
    description: String,
    is_informative: bool,
}

fn extract_options_attrs(attrs: &[syn::Attribute]) -> OptionAttribute {
    let mut short = None;
    let mut description = None;
    let mut is_informative = false;

    let attrs = extract_attributes(attrs);
    for (_meta, attr) in attrs {
        match attr {
            Attribute::Description(desc) => {
                if description.is_some() {
                    panic!("description attributes are duplicated");
                }
                description = Some(desc);
            }
            Attribute::Short(c) => {
                if short.is_some() {
                    panic!("short attributes are duplicated");
                }
                short = Some(c);
            }
            Attribute::Informative => {
                if is_informative {
                    panic!("is_informative attributes are duplicated");
                }
                is_informative = true;
            }
            _ => {
                panic!("Invalid argument is given");
            }
        }
    }

    OptionAttribute {
        short,
        description: description.unwrap_or_else(String::new),
        is_informative,
    }
}

struct OptionVariant {
    ident: syn::Ident,
    short: Option<char>,
    description: String,
    is_informative: bool,
}
impl Parse for OptionVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variant = syn::Variant::parse(input)?;

        let ident = variant.ident;
        let option_attrs = extract_options_attrs(&variant.attrs);

        Ok(Self {
            ident,
            short: option_attrs.short,
            description: option_attrs.description,
            is_informative: option_attrs.is_informative,
        })
    }
}

fn get_long_option(ident: &syn::Ident) -> String {
    ident.to_string().to_lowercase()
}

fn option_to_tokens<T: quote::ToTokens>(x: Option<T>) -> impl quote::ToTokens {
    match x {
        Some(c) => quote! {
            Some(#c)
        },
        None => quote! {
            None
        },
    }
}
