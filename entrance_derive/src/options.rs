use crate::{extract_name_values, get_description, get_single_attribute};
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Token};

pub struct OptionsInput {
    _struct_token: Token![struct],
    ident: syn::Ident,
    _brace_token: syn::token::Brace,
    fields: Punctuated<OptionField, Token![,]>,
}

impl OptionsInput {
    pub fn gen(&self) -> TokenStream {
        let ident = &self.ident;
        let options: Vec<_> = self.fields.iter().collect();
        let declare_lines = options.iter().map(|option| {
            let ident = &option.ident;
            quote! {
                let mut #ident = false;
            }
        });
        let long_option_arms = options.iter().map(|option| long_option_arm(&option.ident));
        let short_option_arms = options.iter().filter_map(|option| {
            let ident = &option.ident;
            let short = option.short?;
            Some(quote! {
                #short => #ident = true,
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
                            return Err(entrance::ErrorKind::InvalidOption.into());
                        }
                    }
                }
                entrance::OptionItem::Short(o) => {
                    match o {
                        #(
                            #short_option_arms
                        )*
                        _ => {
                            return Err(entrance::ErrorKind::InvalidOption.into());
                        }
                    }
                }
            }
        };
        let idents = options.iter().map(|option| &option.ident);
        let idents_for_spec = options.iter().map(|option| &option.ident);
        let num_options = options.len();
        let descriptions = options.iter().map(|option| &option.description);
        let shorts = options.iter().map(|option| option_to_tokens(option.short));
        (quote! {
            impl entrance::Options for #ident {
                fn parse<I: std::iter::Iterator<Item = entrance::OptionItem>>(
                    mut options: I,
                ) -> std::result::Result<Self, entrance::Error> {
                    #(
                        #declare_lines
                    )*

                    for option in options {
                        #parse_lines
                    }

                    Ok(Self {
                        #(
                            #idents,
                        )*
                    })
                }

                fn spec() -> &'static [entrance::Opt] {
                    static OPTS: [entrance::Opt; #num_options] = [
                        #(
                            entrance::Opt {
                                long: stringify!(#idents_for_spec),
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
        if lookahead.peek(Token![struct]) {
            let content;
            Ok(Self {
                _struct_token: input.parse()?,
                ident: input.parse()?,
                _brace_token: braced!(content in input),
                fields: content.parse_terminated(OptionField::parse)?,
            })
        } else {
            Err(lookahead.error())
        }
    }
}

struct OptionField {
    ident: syn::Ident,
    short: Option<char>,
    description: String,
}

impl Parse for OptionField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let field = syn::Field::parse_named(input)?;

        let ident = field.ident.unwrap();
        let name_value_attrs = extract_name_values(&field.attrs);

        let short = get_short_attribute(&name_value_attrs)?;
        let description = get_description(&name_value_attrs);

        Ok(Self {
            ident,
            short,
            description,
        })
    }
}

fn long_option_arm(option: &syn::Ident) -> impl quote::ToTokens {
    quote!(
        stringify!(#option) => #option = true,
    )
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
