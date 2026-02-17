pub use evutils_macros::VariantPropsToJsonArray;

pub trait VariantPropsArray {
    fn props(&self) -> Result<Vec<serde_json::Value>, serde_json::Error>;
}
