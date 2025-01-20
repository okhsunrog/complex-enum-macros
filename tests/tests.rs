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

#[test]
fn test_unit_variant() {
    let variant = TestEnum::Unit;
    assert_eq!(variant.to_code(), Some(0x01));
}

#[test]
fn test_struct_variant() {
    let variant = TestEnum::Struct { value: Some(42) };
    assert_eq!(variant.to_code(), Some(0x02));
}

#[test]
fn test_tuple_variant() {
    let variant = TestEnum::Tuple("test".to_string());
    assert_eq!(variant.to_code(), Some(0x03));
}

#[test]
fn test_no_code_variants() {
    let variant = TestEnum::NoCode;
    assert_eq!(variant.to_code(), None);

    let variant = TestEnum::NoCodeStruct { value: 42 };
    assert_eq!(variant.to_code(), None);
}
#[derive(ToCode)]
#[repr(u8)]
#[allow(dead_code)]
enum I2cCommand {
    Uptime = 0x00,
    AdcStats = 0x01,
    SampleRate { rate: Option<u32> } = 0x02,
    GoertThreshold { threshold: Option<u16> } = 0x03,
    Reset = 0x06,
    Scan,
}
#[test]
fn test_i2c_command() {
    assert_eq!(I2cCommand::Uptime.to_code(), Some(0x00));
    assert_eq!(I2cCommand::AdcStats.to_code(), Some(0x01));
    assert_eq!(
        I2cCommand::SampleRate { rate: Some(1000) }.to_code(),
        Some(0x02)
    );
    assert_eq!(
        I2cCommand::GoertThreshold { threshold: None }.to_code(),
        Some(0x03)
    );
    assert_eq!(I2cCommand::Reset.to_code(), Some(0x06));
    assert_eq!(I2cCommand::Scan.to_code(), None);
}

#[test]
fn test_try_from_unit_variant() {
    let variant = TestEnum::try_from_code(0x01);
    assert_eq!(variant, Some(TestEnum::Unit));
}

#[test]
fn test_try_from_struct_variant() {
    let variant = TestEnum::try_from_code(0x02);
    assert_eq!(variant, Some(TestEnum::Struct { value: None }));
}

#[test]
fn test_try_from_tuple_variant() {
    let variant = TestEnum::try_from_code(0x03);
    assert_eq!(variant, Some(TestEnum::Tuple(String::default())));
}

#[test]
fn test_try_from_invalid_code() {
    let variant = TestEnum::try_from_code(0xFF);
    assert_eq!(variant, None);
}

#[test]
fn test_full_conversion_cycle() {
    let original = TestEnum::Struct { value: None };
    let code = original.to_code().unwrap();
    let converted = TestEnum::try_from_code(code);
    assert_eq!(converted, Some(original));
}
