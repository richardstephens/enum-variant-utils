use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum};


pub(crate) fn derive_variant_name_impl(item: TokenStream) -> TokenStream {

    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    match &input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let match_arms = variants.iter().map(|v| {
                let v_ident = &v.ident;
                quote! {
                    Self::#v_ident { .. } => stringify!(#v_ident),
                }
            });
            quote! {
                #[automatically_derived]
                impl #ident {
                    pub fn varient_name(&self) -> &'static str {
                        match self {
                            #(#match_arms)*
                        }
                    }
                }
            }.into()
        }
        _ => panic!("VariantName only supports enums"),
    }
}