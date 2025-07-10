use complex_enum_macros::{ToCode, TryFromCode};

#[derive(ToCode, Debug, PartialEq, TryFromCode)]
#[repr(u8)]
enum TestEnum {
    // Unit variant with discriminant
    Unit = 0x01,

    // Struct variant with discriminant
    Struct { value: Option<u32> } = 0x02,

    // Tuple variant with discriminant
    Tuple(String) = 0x03,

    // Unit variant without discriminant
    NoCode,

    // Struct variant without discriminant
    NoCodeStruct { value: u32 },
}

fn main() {
    let variant = TestEnum::try_from_code(0x01);
    assert_eq!(variant, Some(TestEnum::Unit));

    let variant = TestEnum::try_from_code(0x02);
    assert_eq!(variant, Some(TestEnum::Struct { value: None }));

    let variant = TestEnum::try_from_code(0x03);
    assert_eq!(variant, Some(TestEnum::Tuple(String::default())));

    let variant = TestEnum::try_from_code(0xFF);
    assert_eq!(variant, None);
}
