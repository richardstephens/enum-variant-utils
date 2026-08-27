use proc_macro::TokenStream;

mod is_variant;
mod step_count;
mod variant_def;
mod variant_name;
mod variant_props;

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

#[proc_macro_derive(StepCount)]
pub fn derive_step_count(item: TokenStream) -> TokenStream {
    step_count::derive_step_count_impl(item)
}

#[proc_macro_derive(VariantDef, attributes(def))]
pub fn derive_variant_def(item: TokenStream) -> TokenStream {
    variant_def::derive_variant_def_impl(item)
}

#[proc_macro_derive(VariantPropsToJsonArray)]
pub fn derive_variant_props_to_json_array(item: TokenStream) -> TokenStream {
    variant_props::derive_variant_props_to_json_array_impl(item)
}
