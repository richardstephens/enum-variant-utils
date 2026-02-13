use proc_macro::TokenStream;
use quote::quote;
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Data, DataEnum, Token};

struct DefAttr {
    pairs: Vec<(syn::Ident, proc_macro2::TokenStream)>,
}

impl Parse for DefAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut pairs = Vec::new();
        while !input.is_empty() {
            let key = syn::Ident::parse_any(input)?;
            input.parse::<Token![=]>()?;

            let mut tokens = proc_macro2::TokenStream::new();
            while !input.is_empty() && !input.peek(Token![,]) {
                let tt: proc_macro2::TokenTree = input.parse()?;
                tokens.extend(std::iter::once(tt));
            }
            pairs.push((key, tokens));

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(DefAttr { pairs })
    }
}

pub(crate) fn derive_variant_def_impl(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    // Parse enum-level #[def(...)] attribute
    let enum_def_attr = input
        .attrs
        .iter()
        .find(|a| a.path().is_ident("def"))
        .expect("VariantDef requires a #[def(...)] attribute on the enum");

    let enum_def: DefAttr = enum_def_attr
        .parse_args()
        .expect("Failed to parse enum-level #[def(...)] attribute");

    // Extract struct path, getter name, and field defaults
    let mut def_struct: Option<proc_macro2::TokenStream> = None;
    let mut getter: Option<syn::Ident> = None;
    let mut defaults: Vec<(syn::Ident, proc_macro2::TokenStream)> = Vec::new();

    for (key, value) in enum_def.pairs {
        let key_str = key.to_string();
        if key_str == "struct" {
            def_struct = Some(value);
        } else if key_str == "getter" {
            getter = Some(syn::parse2(value).expect("getter must be an identifier"));
        } else if let Some(field_name) = key_str.strip_prefix("default_") {
            defaults.push((syn::Ident::new(field_name, key.span()), value));
        } else {
            panic!("Unknown enum-level def attribute key: {}", key_str);
        }
    }

    let def_struct = def_struct.expect("VariantDef requires struct=<Type> in #[def(...)]");
    let getter = getter.expect("VariantDef requires getter=<name> in #[def(...)]");

    match &input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let match_arms: Vec<_> = variants
                .iter()
                .map(|v| {
                    let v_ident = &v.ident;

                    // Parse variant-level #[def(...)] if present
                    let variant_fields: Vec<(syn::Ident, proc_macro2::TokenStream)> = v
                        .attrs
                        .iter()
                        .find(|a| a.path().is_ident("def"))
                        .map(|attr| {
                            let def: DefAttr = attr
                                .parse_args()
                                .expect("Failed to parse variant #[def(...)] attribute");
                            def.pairs
                        })
                        .unwrap_or_default();

                    let variant_field_names: Vec<&syn::Ident> =
                        variant_fields.iter().map(|(k, _)| k).collect();
                    let variant_field_values: Vec<&proc_macro2::TokenStream> =
                        variant_fields.iter().map(|(_, v)| v).collect();

                    let default_field_names: Vec<&syn::Ident> =
                        defaults.iter().map(|(k, _)| k).collect();
                    let default_field_values: Vec<&proc_macro2::TokenStream> =
                        defaults.iter().map(|(_, v)| v).collect();

                    quote! {
                        Self::#v_ident { .. } => #def_struct {
                            #(#variant_field_names: #variant_field_values,)*
                            ..#def_struct {
                                #(#default_field_names: #default_field_values,)*
                                ..Default::default()
                            }
                        },
                    }
                })
                .collect();

            quote! {
                #[automatically_derived]
                impl #ident {
                    pub fn #getter(&self) -> #def_struct {
                        match self {
                            #(#match_arms)*
                        }
                    }
                }
            }
            .into()
        }
        _ => panic!("VariantDef only supports enums"),
    }
}
