use evutils_macros::{DisplayVariantName, VariantName};

#[derive(VariantName)]
pub enum MyEnum {
    VariantOne,
    VariantTwo,
    VariantThree(u16),
}

#[derive(DisplayVariantName)]
pub enum MyEnumDisplayTest {
    VariantOne,
    VariantTwo,
    VariantThree(u16),
}
#[cfg(test)]
mod tests {
    use crate::{MyEnum, MyEnumDisplayTest};

    #[test]
    fn get_variant_name() {
        assert_eq!("VariantOne", MyEnum::VariantOne.varient_name());
        assert_eq!("VariantTwo", MyEnum::VariantTwo.varient_name());
        assert_eq!("VariantThree", MyEnum::VariantThree(1).varient_name());
    }

    #[test]
    fn display_variant_name() {
        assert_eq!("VariantOne", MyEnumDisplayTest::VariantOne.to_string());
        assert_eq!("VariantTwo", format!("{}", MyEnumDisplayTest::VariantTwo));
        assert_eq!(
            "VariantThree",
            format!("{}", MyEnumDisplayTest::VariantThree(1))
        );
    }
}
