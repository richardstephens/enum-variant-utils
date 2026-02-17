use evutils::variant_props::{VariantPropsArray, VariantPropsToJsonArray};
use serde::Serialize;
use serde_json::json;

#[derive(VariantPropsToJsonArray)]
enum MixedEnum {
    Empty,
    Single(u32),
    Multi(u32, String),
    Numbers(u32, i32, f64, i16),
}

#[derive(Serialize)]
struct Inner {
    x: u32,
    y: String,
}

#[derive(VariantPropsToJsonArray)]
enum WithSerializable {
    Thing(Inner),
}

#[test]
fn unit_variant_returns_empty_vec() {
    let result = MixedEnum::Empty.props().unwrap();
    assert!(result.is_empty());
}

#[test]
fn single_field_variant() {
    let result = MixedEnum::Single(42).props().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], serde_json::json!(42));
}

#[test]
fn multi_field_variant() {
    let result = MixedEnum::Multi(1, "hello".to_string()).props().unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], serde_json::json!(1));
    assert_eq!(result[1], serde_json::json!("hello"));
}

#[test]
fn serializable_struct_field() {
    let result = WithSerializable::Thing(Inner {
        x: 10,
        y: "world".to_string(),
    })
    .props()
    .unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], serde_json::json!({"x": 10, "y": "world"}));
}

#[test]
fn many_numbers() {
    assert_eq!(
        json!([1, -2, 3.5, 5555]),
        json!(MixedEnum::Numbers(1, -2, 3.5, 5555).props().unwrap())
    );
}
