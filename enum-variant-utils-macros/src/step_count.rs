use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DataEnum};

pub(crate) fn derive_step_count_impl(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let ident = &input.ident;

    match &input.data {
        Data::Enum(DataEnum { variants, .. }) => {
            if variants.is_empty() {
                panic!("StepCount requires an enum with at least one variant");
            }
            let total = variants.len();
            let match_arms = variants.iter().enumerate().map(|(i, v)| {
                let v_ident = &v.ident;
                let step = i + 1;
                quote! {
                    Self::#v_ident { .. } => evutils::step_count::Step::new(#step, #total),
                }
            });

            quote! {
                #[automatically_derived]
                impl evutils::step_count::StepCount for #ident {
                    fn step(&self) -> evutils::step_count::Step {
                        match self {
                            #(#match_arms)*
                        }
                    }
                }
            }
            .into()
        }
        _ => panic!("StepCount only supports enums"),
    }
}
