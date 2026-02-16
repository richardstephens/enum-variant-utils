use bon::Builder;
use enum_variant_utils_macros::VariantDef;

#[derive(Builder)]
#[builder(const)]
struct FruitProps {
    colour: &'static str,
    #[builder(default = false)]
    sour: bool,
    price: Option<f32>,
}

#[derive(VariantDef)]
#[def(struct = FruitProps, builder)]
enum Fruit {
    #[def(colour = "red")]
    Tomato,
    #[def(colour = "green", price = 2.99)]
    Apple,
    #[def(colour = "yellow", sour = true)]
    Lemon,
}

#[cfg(test)]
mod tests {
    use crate::Fruit;
    use enum_variant_utils::variant_def::VariantDef;

    #[test]
    fn explicit_field() {
        assert_eq!("red", Fruit::Tomato.variant_def().colour);
        assert_eq!("yellow", Fruit::Lemon.variant_def().colour);
    }

    #[test]
    fn builder_default_field() {
        assert!(!Fruit::Tomato.variant_def().sour);
    }

    #[test]
    fn builder_default_overridden() {
        assert!(Fruit::Lemon.variant_def().sour);
    }

    #[test]
    fn option_field_unset() {
        assert_eq!(None, Fruit::Tomato.variant_def().price);
    }

    #[test]
    fn option_field_set() {
        assert_eq!(Some(2.99), Fruit::Apple.variant_def().price);
    }

    #[test]
    fn all_variant_defs() {
        let all = Fruit::all_variant_defs();
        assert_eq!(3, all.len());
        assert_eq!("red", all[0].colour);
        assert_eq!("green", all[1].colour);
        assert_eq!("yellow", all[2].colour);
    }
}
