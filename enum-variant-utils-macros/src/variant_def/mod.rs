mod codegen;
mod parser;

use crate::variant_def::codegen::generate;
use crate::variant_def::parser::VariantDefInput;
use proc_macro::TokenStream;

pub(crate) fn derive_variant_def_impl(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    let parsed = VariantDefInput::parse(input);
    generate(&parsed).into()
}
