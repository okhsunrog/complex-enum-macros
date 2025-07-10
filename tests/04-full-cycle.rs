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
    let original = TestEnum::Struct { value: None };
    let code = original.to_code().unwrap();
    let converted = TestEnum::try_from_code(code);
    assert_eq!(converted, Some(original));
}
