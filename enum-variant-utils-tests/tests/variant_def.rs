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
#[def(struct = TestEnumVariantDef, getter = get_def, all_getter = get_all,
      default_kind = VariantKind::UnknownKind, default_added_by = None)]
enum TestEnum {
    #[def(message = "Variant one details", kind = VariantKind::FirstKind)]
    VariantOne,
    #[def(message = "Variant two details", kind = VariantKind::SecondKind)]
    VariantTwo,
    #[def(message = "Variant 3", added_by = Some("steve"))]
    VariantThree,
    #[def(message = "Variant four")]
    VariantFour,
}

#[cfg(test)]
mod tests {
    use crate::{TestEnum, TestEnumVariantDef, VariantKind};

    #[test]
    fn get_def() {
        assert_eq!(
            &TestEnumVariantDef {
                message: "Variant one details",
                kind: VariantKind::FirstKind,
                added_by: None,
            },
            TestEnum::VariantOne.get_def()
        );
    }

    #[test]
    fn second_kind() {
        assert_eq!(VariantKind::SecondKind, TestEnum::VariantTwo.get_def().kind);
    }

    #[test]
    fn default_kind() {
        assert_eq!(
            VariantKind::UnknownKind,
            TestEnum::VariantFour.get_def().kind
        );
    }

    #[test]
    fn optional_field() {
        assert_eq!("steve", TestEnum::VariantThree.get_def().added_by.unwrap());
        assert_eq!(None, TestEnum::VariantFour.get_def().added_by);
    }

    #[test]
    fn get_all() {
        let all = TestEnum::get_all();
        assert_eq!(4, all.len());
        assert_eq!("Variant one details", all[0].message);
        assert_eq!(VariantKind::SecondKind, all[1].kind);
        assert_eq!(Some("steve"), all[2].added_by);
        assert_eq!("Variant four", all[3].message);
    }
}
