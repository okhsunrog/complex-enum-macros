# complex-enum-macros

A Rust procedural macro for deriving code/discriminant conversion methods for enums, supporting both unit and complex variants.

## Features
- Convert enum variants to their discriminant values
- Convert u8 codes back to enum variants with `try_from_code()`
- Support for unit variants, struct-like variants, and tuple variants
- Works with explicit discriminants
- Default initialization of variant fields during code conversion
- Zero dependencies in the generated code

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
complex-enum-macros = "0.2"
```

## Usage

```rust
use complex_enum_macros::{ToCode, TryFromCode};

#[derive(ToCode, TryFromCode, Debug, PartialEq)]
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
    // Convert enum to code
    let cmd = I2cCommand::Uptime;
    assert_eq!(cmd.to_code(), Some(0x00));

    let cmd = I2cCommand::SampleRate { rate: Some(1000) };
    assert_eq!(cmd.to_code(), Some(0x02));

    // Create enum from code
    let cmd = I2cCommand::try_from_code(0x03).unwrap();
    assert!(matches!(cmd, I2cCommand::GoertThreshold { .. }));

    // Invalid codes return None
    assert_eq!(I2cCommand::try_from_code(0xFF), None);

    // Variants without explicit discriminants return None for to_code()
    let cmd = I2cCommand::Scan;
    assert_eq!(cmd.to_code(), None);
}
```
