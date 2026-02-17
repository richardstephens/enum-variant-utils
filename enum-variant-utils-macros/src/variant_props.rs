use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DataEnum, Fields};

pub(crate) fn derive_variant_props_to_json_array_impl(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    match &input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let match_arms = variants.iter().map(|v| {
                let v_ident = &v.ident;

                match &v.fields {
                    Fields::Unit => {
                        quote! {
                            Self::#v_ident => Ok(vec![]),
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let bindings: Vec<_> = (0..fields.unnamed.len())
                            .map(|i| format_ident!("v{}", i))
                            .collect();
                        let to_values = bindings.iter().map(|b| {
                            quote! { serde_json::to_value(#b)? }
                        });
                        quote! {
                            Self::#v_ident(#(#bindings),*) => {
                                Ok(vec![#(#to_values),*])
                            }
                        }
                    }
                    Fields::Named(_) => {
                        panic!(
                            "VariantPropsToJsonArray does not support variants with named fields (found on variant `{v_ident}`)"
                        );
                    }
                }
            });

            quote! {
                #[automatically_derived]
                impl evutils::variant_props::VariantPropsArray for #ident {
                    fn props(&self) -> Result<Vec<serde_json::Value>, serde_json::Error> {
                        match self {
                            #(#match_arms)*
                        }
                    }
                }
            }
            .into()
        }
        _ => panic!("VariantPropsToJsonArray only supports enums"),
    }
}
