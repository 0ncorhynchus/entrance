use crate::{extract_name_values, get_description, get_single_attribute};
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
                            Err(entrance::ErrorKind::InvalidOption.into())
                        }
                    }
                }
                entrance::OptionItem::Short(o) => {
                    match o {
                        #(
                            #short_option_arms
                        )*
                        _ => {
                            Err(entrance::ErrorKind::InvalidOption.into())
                        }
                    }
                }
            }
        };
        let idents = options.iter().map(|option| get_long_option(&option.ident));
        let num_options = options.len();
        let descriptions = options.iter().map(|option| &option.description);
        let shorts = options.iter().map(|option| option_to_tokens(option.short));
        (quote! {
            impl entrance::Options for #ident {
                fn parse(option: entrance::OptionItem) -> entrance::Result<Self> {
                    #parse_lines
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

struct OptionVariant {
    ident: syn::Ident,
    short: Option<char>,
    description: String,
}
impl Parse for OptionVariant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variant = syn::Variant::parse(input)?;

        let ident = variant.ident;
        let name_value_attrs = extract_name_values(&variant.attrs);

        let short = get_short_attribute(&name_value_attrs)?;
        let description = get_description(&name_value_attrs);

        Ok(Self {
            ident,
            short,
            description,
        })
    }
}

fn get_long_option(ident: &syn::Ident) -> String {
    ident.to_string().to_lowercase()
}

fn get_short_attribute(name_value_attrs: &[syn::MetaNameValue]) -> syn::Result<Option<char>> {
    match get_single_attribute("short", name_value_attrs) {
        Some(lit) => {
            if let syn::Lit::Char(c) = lit {
                Ok(Some(c.value()))
            } else {
                Err(syn::Error::new_spanned(
                    lit,
                    "Invalid usage of `short` attribute: expected a char",
                ))
            }
        }
        None => Ok(None),
    }
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
