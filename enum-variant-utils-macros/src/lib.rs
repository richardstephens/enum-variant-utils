use proc_macro::TokenStream;

mod is_variant;
mod variant_def;
mod variant_name;

#[proc_macro_derive(VariantName)]
pub fn derive_variant_name(item: TokenStream) -> TokenStream {
    variant_name::derive_variant_name_impl(item)
}

#[proc_macro_derive(DisplayVariantName)]
pub fn derive_display_variant_name(item: TokenStream) -> TokenStream {
    variant_name::derive_display_variant_name_impl(item)
}

#[proc_macro_derive(IsVariant)]
pub fn derive_is_variant(item: TokenStream) -> TokenStream {
    is_variant::derive_is_variant_impl(item)
}

#[proc_macro_derive(VariantDef, attributes(def))]
pub fn derive_variant_def(item: TokenStream) -> TokenStream {
    variant_def::derive_variant_def_impl(item)
}
