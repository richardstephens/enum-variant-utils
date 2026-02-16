use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::{Data, DataEnum, Token};

pub(crate) struct VariantDefInput {
    pub enum_ident: syn::Ident,
    pub def_struct: proc_macro2::TokenStream,
    pub getter: Option<syn::Ident>,
    pub all_getter: Option<syn::Ident>,
    pub builder: Option<proc_macro2::TokenStream>,
    pub variant_name_field: Option<syn::Ident>,
    pub variant_name_case: Option<String>,
    pub defaults: Vec<(syn::Ident, proc_macro2::TokenStream)>,
    pub variants: Vec<VariantDef>,
}

impl VariantDefInput {
    pub(crate) fn parse(input: syn::DeriveInput) -> Self {
        let enum_ident = input.ident;

        let enum_def_attr = input
            .attrs
            .iter()
            .find(|a| a.path().is_ident("def"))
            .expect("VariantDef requires a #[def(...)] attribute on the enum");

        let enum_def: DefAttr = enum_def_attr
            .parse_args()
            .expect("Failed to parse enum-level #[def(...)] attribute");

        let mut def_struct: Option<proc_macro2::TokenStream> = None;
        let mut getter: Option<syn::Ident> = None;
        let mut all_getter: Option<syn::Ident> = None;
        let mut builder: Option<proc_macro2::TokenStream> = None;
        let mut variant_name_field: Option<syn::Ident> = None;
        let mut variant_name_case: Option<String> = None;
        let mut defaults: Vec<(syn::Ident, proc_macro2::TokenStream)> = Vec::new();

        for (key, value) in enum_def.pairs {
            let key_str = key.to_string();
            if key_str == "struct" {
                def_struct = Some(value);
            } else if key_str == "getter" {
                getter = Some(syn::parse2(value).expect("getter must be an identifier"));
            } else if key_str == "all_getter" {
                all_getter = Some(syn::parse2(value).expect("all_getter must be an identifier"));
            } else if key_str == "builder_fn" {
                builder = Some(value);
            } else if key_str == "variant_name_field" {
                variant_name_field =
                    Some(syn::parse2(value).expect("variant_name_field must be an identifier"));
            } else if key_str == "variant_name_case" {
                let ident: syn::Ident =
                    syn::parse2(value).expect("variant_name_case must be an identifier");
                variant_name_case = Some(ident.to_string());
            } else if let Some(field_name) = key_str.strip_prefix("default_") {
                defaults.push((syn::Ident::new(field_name, key.span()), value));
            } else {
                panic!("Unknown enum-level def attribute key: {}", key_str);
            }
        }

        let def_struct = def_struct.expect("VariantDef requires struct=<Type> in #[def(...)]");

        let variants = match &input.data {
            Data::Enum(DataEnum { variants, .. }) => variants
                .iter()
                .map(|v| {
                    let fields = v
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

                    VariantDef {
                        ident: v.ident.clone(),
                        fields,
                    }
                })
                .collect(),
            _ => panic!("VariantDef only supports enums"),
        };

        VariantDefInput {
            enum_ident,
            def_struct,
            getter,
            all_getter,
            builder,
            variant_name_field,
            variant_name_case,
            defaults,
            variants,
        }
    }
}

pub(crate) struct DefAttr {
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

pub(crate) struct VariantDef {
    pub ident: syn::Ident,
    pub fields: Vec<(syn::Ident, proc_macro2::TokenStream)>,
}
