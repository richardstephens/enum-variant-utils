use enum_variant_utils_macros::IsVariant;

#[derive(IsVariant)]
enum MyEnum {
    VariantOne,
    VariantTwo,
    VariantThree,
}

#[cfg(test)]
mod tests {
    use crate::is_variant_tests::MyEnum;

    #[test]
    fn it_works() {
        assert!(MyEnum::VariantOne.is_variantone());
        assert!(!MyEnum::VariantOne.is_varianttwo());
    }
}
