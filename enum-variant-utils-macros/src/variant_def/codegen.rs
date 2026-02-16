use crate::variant_def::parser::VariantDefInput;
use convert_case::{Case, Converter};
use quote::{format_ident, quote};

pub fn generate(input: &VariantDefInput) -> proc_macro2::TokenStream {
    let enum_ident = &input.enum_ident;
    let def_struct = &input.def_struct;

    let conv = Converter::new()
        .from_case(Case::Pascal)
        .to_case(Case::UpperSnake);

    let const_names: Vec<syn::Ident> = input
        .variants
        .iter()
        .map(|v| format_ident!("{}_DEF", conv.convert(v.ident.to_string())))
        .collect();

    let mut all_field_names: Vec<syn::Ident> = Vec::new();
    for (key, _) in &input.defaults {
        if !all_field_names.iter().any(|k| k == key) {
            all_field_names.push(key.clone());
        }
    }
    for v in &input.variants {
        for (key, _) in &v.fields {
            if !all_field_names.iter().any(|k| k == key) {
                all_field_names.push(key.clone());
            }
        }
    }

    let const_defs: Vec<_> = input
        .variants
        .iter()
        .zip(const_names.iter())
        .map(|(v, const_name)| {
            let fields: Vec<_> = all_field_names
                .iter()
                .filter_map(|field_name| {
                    let val = if let Some((_, val)) = v.fields.iter().find(|(k, _)| k == field_name)
                    {
                        val
                    } else if let Some((_, val)) =
                        input.defaults.iter().find(|(k, _)| k == field_name)
                    {
                        val
                    } else {
                        if input.builder.is_none() {
                            panic!(
                                "Field `{}` used by variant `{}` has no default value",
                                field_name, v.ident
                            );
                        } else {
                            return None;
                        }
                    };
                    if input.builder.is_some() {
                        Some(quote! { .#field_name(#val) })
                    } else {
                        Some(quote! { #field_name: #val })
                    }
                })
                .collect();

            if let Some(builder_fn) = &input.builder {
                quote! {
                    const #const_name: #def_struct = #builder_fn
                        #(#fields)*
                        .build();
                }
            } else {
                quote! {
                    const #const_name: #def_struct = #def_struct {
                        #(#fields,)*
                    };
                }
            }
        })
        .collect();

    let variant_idents: Vec<&syn::Ident> = input.variants.iter().map(|v| &v.ident).collect();

    let match_arms: Vec<_> = variant_idents
        .iter()
        .zip(const_names.iter())
        .map(|(v_ident, c_name)| {
            quote! {
                Self::#v_ident { .. } => &Self::#c_name,
            }
        })
        .collect();

    let getter_fn = input.getter.as_ref().map(|getter| {
        quote! {
            pub const fn #getter(&self) -> &'static #def_struct {
                match self {
                    #(#match_arms)*
                }
            }
        }
    });

    let all_getter_fn = input.all_getter.as_ref().map(|all_getter| {
        quote! {
            pub const fn #all_getter() -> &'static [#def_struct] {
                Self::ALL_DEFS
            }
        }
    });

    quote! {
        #[automatically_derived]
        impl #enum_ident {
            #(#const_defs)*

            const ALL_DEFS: &'static [#def_struct] = &[
                #(Self::#const_names,)*
            ];

            #getter_fn
            #all_getter_fn
        }

        #[automatically_derived]
        impl enum_variant_utils::variant_def::VariantDef for #enum_ident {
            type Def = #def_struct;

            fn variant_def(&self) -> &'static Self::Def {
                match self {
                    #(#match_arms)*
                }
            }

            fn all_variant_defs() -> &'static [Self::Def] {
                Self::ALL_DEFS
            }
        }
    }
}
