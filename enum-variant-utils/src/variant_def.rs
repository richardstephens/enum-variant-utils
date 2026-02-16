pub use evutils_macros::VariantDef;

pub trait VariantDef {
    type Def;
    fn variant_def(&self) -> &'static Self::Def;
    fn all_variant_defs() -> &'static [Self::Def];
}
