use proc_macro::TokenStream;

mod variant_name;

#[proc_macro_derive(VariantName)]
pub fn derive_variant_name(item: TokenStream) -> TokenStream {
    variant_name::derive_variant_name_impl(item)
}
