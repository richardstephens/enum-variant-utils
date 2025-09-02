use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Data, DataEnum, Variant};

fn variant_name_match_arms(variants: &Punctuated<Variant, Comma>) -> Vec<proc_macro2::TokenStream> {
    variants
        .iter()
        .map(|v| {
            let v_ident = &v.ident;
            quote! {
                Self::#v_ident { .. } => stringify!(#v_ident),
            }
        })
        .collect()
}

pub(crate) fn derive_variant_name_impl(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    match &input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let match_arms = variant_name_match_arms(variants);
            quote! {
                #[automatically_derived]
                impl #ident {
                    pub fn varient_name(&self) -> &'static str {
                        match self {
                            #(#match_arms)*
                        }
                    }
                }
            }
            .into()
        }
        _ => panic!("VariantName only supports enums"),
    }
}

pub(crate) fn derive_display_variant_name_impl(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    match &input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let match_arms = variant_name_match_arms(variants);

            quote! {
                #[automatically_derived]
                impl std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        let name = match self {
                            #(#match_arms)*
                        };
                        write!(f, "{}", name)
                    }
                }
            }
            .into()
        }
        _ => panic!("VariantName only supports enums"),
    }
}
