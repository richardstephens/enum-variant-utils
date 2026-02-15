pub use enum_variant_utils_macros::VariantDef;
pub trait VariantDef {
    type Def;
    fn variant_def(&self) -> &'static Self::Def;
    fn all_variant_defs() -> &'static [Self::Def];
}
