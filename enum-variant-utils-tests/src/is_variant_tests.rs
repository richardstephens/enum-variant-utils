use enum_variant_utils_macros::IsVariant;

pub struct InnerStruct {
    pub blah: String,
}
#[derive(IsVariant)]
pub enum MyEnum {
    VariantOne,
    VariantTwo,
    VariantThree(u32),
    VariantFour { x: u32, y: u32 },
    VariantFive { a: String },
    VariantSix(InnerStruct),
    VariantSeven(InnerStruct, u16),
}

#[cfg(test)]
mod tests {
    use crate::is_variant_tests::MyEnum;

    #[test]
    fn simple_case() {
        assert!(MyEnum::VariantOne.is_variant_one());
        assert!(!MyEnum::VariantOne.is_variant_two());
    }
    #[test]
    fn has_integer() {
        assert!(MyEnum::VariantThree(4).is_variant_three());
        assert!(!MyEnum::VariantThree(4).is_variant_two());
    }
}
