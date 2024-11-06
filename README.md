# complex-enum-macros

A Rust procedural macro for deriving code/discriminant conversion methods for enums, supporting both unit and complex variants.

## Features
- Convert enum variants to their discriminant values
- Support for unit variants, struct-like variants, and tuple variants
- Works with explicit discriminants
- Zero dependencies in the generated code

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
complex-enum-macros = "0.1"
```

## Usage

```
use complex_enum_macros::ToCode;

#[derive(ToCode)]
#[repr(u8)]
pub enum I2cCommand {
    Uptime = 0x00,
    AdcStats = 0x01,
    SampleRate { rate: Option<u32> } = 0x02,
    GoertThreshold { threshold: Option<u16> } = 0x03,
    Reset = 0x06,
    // Variants without explicit discriminants are also supported
    Scan,
    WavStart,
}

fn main() {
    let cmd = I2cCommand::Uptime;
    assert_eq!(cmd.to_code(), Some(0x00));

    let cmd = I2cCommand::SampleRate { rate: Some(1000) };
    assert_eq!(cmd.to_code(), Some(0x02));

    // Variants without explicit discriminants return None
    let cmd = I2cCommand::Scan;
    assert_eq!(cmd.to_code(), None);
}
```
