use proc_macro::TokenStream;

mod is_variant;
mod variant_name;

#[proc_macro_derive(VariantName)]
pub fn derive_variant_name(item: TokenStream) -> TokenStream {
    variant_name::derive_variant_name_impl(item)
}

#[proc_macro_derive(IsVariant)]
pub fn derive_is_variant(item: TokenStream) -> TokenStream {
    is_variant::derive_is_variant_impl(item)
}
