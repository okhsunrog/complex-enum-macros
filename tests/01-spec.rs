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
    let variant = TestEnum::Unit;
    assert_eq!(variant.to_code(), Some(0x01));

    let variant = TestEnum::Struct { value: Some(42) };
    assert_eq!(variant.to_code(), Some(0x02));

    let variant = TestEnum::Tuple("test".to_string());
    assert_eq!(variant.to_code(), Some(0x03));

    let variant = TestEnum::NoCode;
    assert_eq!(variant.to_code(), None);

    let variant = TestEnum::NoCodeStruct { value: 42 };
    assert_eq!(variant.to_code(), None);
}
