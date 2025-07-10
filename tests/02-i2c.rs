use complex_enum_macros::ToCode;

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

fn main() {
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
