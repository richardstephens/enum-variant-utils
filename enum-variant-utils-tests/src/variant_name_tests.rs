use enum_variant_utils_macros::VariantName;

#[derive(VariantName)]
enum MyEnum {
    VariantOne,
    VariantTwo,
    VariantThree,
}

#[cfg(test)]
mod tests {
    use crate::variant_name_tests::MyEnum;

    #[test]
    fn it_works() {
        assert_eq!("VariantOne", MyEnum::VariantOne.varient_name());
        assert_eq!("VariantTwo", MyEnum::VariantTwo.varient_name());
        assert_eq!("VariantThree", MyEnum::VariantThree.varient_name());
    }
}
