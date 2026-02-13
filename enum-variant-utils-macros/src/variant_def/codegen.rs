use crate::variant_def::parser::VariantDefInput;
use quote::quote;

pub fn generate(input: &VariantDefInput) -> proc_macro2::TokenStream {
    let enum_ident = &input.enum_ident;
    let def_struct = &input.def_struct;
    let getter = &input.getter;

    let default_field_names: Vec<&syn::Ident> = input.defaults.iter().map(|(k, _)| k).collect();
    let default_field_values: Vec<&proc_macro2::TokenStream> =
        input.defaults.iter().map(|(_, v)| v).collect();

    let match_arms: Vec<_> = input
        .variants
        .iter()
        .map(|v| {
            let v_ident = &v.ident;
            let field_names: Vec<&syn::Ident> = v.fields.iter().map(|(k, _)| k).collect();
            let field_values: Vec<&proc_macro2::TokenStream> =
                v.fields.iter().map(|(_, v)| v).collect();

            quote! {
                Self::#v_ident { .. } => #def_struct {
                    #(#field_names: #field_values,)*
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
        impl #enum_ident {
            pub fn #getter(&self) -> #def_struct {
                match self {
                    #(#match_arms)*
                }
            }
        }
    }
}
