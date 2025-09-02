use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum};

pub(crate) fn derive_is_variant_impl(item: TokenStream) -> TokenStream {
    let conv = convert_case::Converter::new()
        .from_case(convert_case::Case::Pascal)
        .to_case(convert_case::Case::Snake);
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    match &input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let is_variant_fns = variants.iter().map(|v| {
                let v_ident = &v.ident;
                let fn_name = syn::Ident::new(
                    &format!("is_{}", conv.convert(v_ident.to_string())),
                    v_ident.span(),
                );

                quote! {
                    pub fn #fn_name(&self) -> bool {
                        matches!(self, Self::#v_ident { .. })
                    }
                }
            });
            quote! {
                #[automatically_derived]
                impl #ident {
                    #(#is_variant_fns)*
                }
            }
            .into()
        }
        _ => panic!("IsVariant only supports enums"),
    }
}
