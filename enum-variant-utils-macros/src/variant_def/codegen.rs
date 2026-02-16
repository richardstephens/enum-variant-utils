use crate::variant_def::parser::{VariantDef, VariantDefInput};
use convert_case::{Case, Converter};
use quote::{format_ident, quote};

fn variant_name_converter(name: &str) -> Converter {
    let base = Converter::new().from_case(Case::Pascal);
    match name {
        "snake_case" => base.to_case(Case::Snake),
        "SCREAMING_SNAKE_CASE" => base.to_case(Case::UpperSnake),
        "camelCase" => base.to_case(Case::Camel),
        "PascalCase" => base.to_case(Case::Pascal),
        "kebab-case" => base.to_case(Case::Kebab),
        "SCREAMING-KEBAB-CASE" => base.to_case(Case::UpperKebab),
        "dotted.lower.case" => base
            .set_pattern(convert_case::pattern::lowercase)
            .set_delim("."),
        other => panic!("Unsupported variant_name_case: {}", other),
    }
}

/// Resolve field values for a variant: variant-level attrs, then enum-level
/// defaults, then variant_name_field injection. Returns (field_name, tokens)
/// pairs for all fields that have a value.
fn resolve_fields<'a>(
    variant: &'a VariantDef,
    input: &'a VariantDefInput,
    all_field_names: &'a [syn::Ident],
    variant_name: &'a str,
) -> Vec<(&'a syn::Ident, proc_macro2::TokenStream)> {
    all_field_names
        .iter()
        .filter_map(|field_name| {
            if let Some(name_field) = &input.variant_name_field
                && field_name == name_field
            {
                Some((field_name, quote! { #variant_name }))
            } else if let Some((_, val)) = variant.fields.iter().find(|(k, _)| k == field_name) {
                Some((field_name, val.clone()))
            } else if let Some((_, val)) = input.defaults.iter().find(|(k, _)| k == field_name) {
                Some((field_name, val.clone()))
            } else if input.builder.is_none() {
                panic!(
                    "Field `{}` used by variant `{}` has no default value",
                    field_name, variant.ident
                );
            } else {
                None
            }
        })
        .collect()
}

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

    // If injecting the variant name into the def, it needs to be added to the list of field names
    if let Some(name_field) = &input.variant_name_field
        && !all_field_names.iter().any(|k| k == name_field)
    {
        all_field_names.insert(0, name_field.clone());
    }

    let variant_name_conv = input
        .variant_name_case
        .as_ref()
        .map(|c| variant_name_converter(c));

    let variant_names: Vec<String> = input
        .variants
        .iter()
        .map(|v| {
            let name = v.ident.to_string();
            match &variant_name_conv {
                Some(conv) => conv.convert(&name),
                None => name,
            }
        })
        .collect();

    let const_defs: Vec<_> = itertools::izip!(&input.variants, &const_names, &variant_names)
        .map(|(v, const_name, vname)| {
            let fields = resolve_fields(v, input, &all_field_names, vname);

            if let Some(builder_fn) = &input.builder {
                let calls: Vec<_> = fields
                    .iter()
                    .map(|(name, val)| quote! { .#name(#val) })
                    .collect();
                quote! {
                    const #const_name: #def_struct = #builder_fn
                        #(#calls)*
                        .build();
                }
            } else {
                let assignments: Vec<_> = fields
                    .iter()
                    .map(|(name, val)| quote! { #name: #val })
                    .collect();
                quote! {
                    const #const_name: #def_struct = #def_struct {
                        #(#assignments,)*
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
