use enum_variant_utils_macros::VariantDef;

#[derive(Default, Debug, PartialEq)]
enum VariantKind {
    FirstKind,
    SecondKind,
    #[default]
    DefaultKind,
    UnknownKind,
}

#[derive(Default, Debug, PartialEq)]
struct TestEnumVariantDef {
    message: &'static str,
    kind: VariantKind,
    added_by: Option<&'static str>,
}

#[derive(VariantDef)]
#[def(struct = TestEnumVariantDef, default_kind = VariantKind::UnknownKind, getter = get_def)]
enum TestEnum {
    #[def(message = "Variant one details", kind = VariantKind::FirstKind)]
    VariantOne,
    #[def(message = "Variant two details", kind = VariantKind::SecondKind)]
    VariantTwo,
    #[def(message = "Variant three")]
    VariantThree,
    #[def(message = "Variant four", added_by = Some("steve"))]
    VariantFour,
    #[def(message = "Variant five")]
    VariantFive,
    #[def(message = "Variant six")]
    VariantSix,
}

#[cfg(test)]
mod tests {
    use crate::{TestEnum, TestEnumVariantDef, VariantKind};

    #[test]
    fn get_def() {
        assert_eq!(
            TestEnumVariantDef {
                message: "Variant one details",
                kind: VariantKind::FirstKind,
                added_by: None,
            },
            TestEnum::VariantOne.get_def()
        );
    }

    #[test]
    fn default_kind() {
        assert_eq!(
            VariantKind::UnknownKind,
            TestEnum::VariantSix.get_def().kind
        );
    }

    #[test]
    fn optional_field() {
        assert_eq!("steve", TestEnum::VariantFour.get_def().added_by.unwrap());
    }
}
