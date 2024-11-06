//! A derive macro for automatically implementing code conversion for enums
//!
//! This crate provides a `ToCode` derive macro that generates a `to_code()` method
//! for converting enum variants to `Option<u8>` values.
//!
//! # Example
//! ```
//! use complex_enum_macros::ToCode;
//!
//! #[derive(ToCode)]
//! #[repr(u8)]
//! enum Command {
//!     Start = 0x01,
//!     SetConfig { value: Option<u32> } = 0x02,
//!     SendData(String) = 0x03,
//!     Stop = 0x04,
//!     Unknown,
//! }
//!
//! let cmd = Command::SetConfig { value: Some(42) };
//! assert_eq!(cmd.to_code(), Some(0x02));
//! let cmd = Command::SendData("Hello, world!".to_string());
//! assert_eq!(cmd.to_code(), Some(0x03));
//! let cmd = Command::Unknown;
//! assert_eq!(cmd.to_code(), None);
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ToCode)]
pub fn derive_to_code(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("ToCode can only be derived for enums"),
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        if let Some((_, expr)) = &variant.discriminant {
            match &variant.fields {
                Fields::Unit => quote! {
                    #name::#variant_name => Some(#expr),
                },
                Fields::Named(_) => quote! {
                    #name::#variant_name { .. } => Some(#expr),
                },
                Fields::Unnamed(_) => quote! {
                    #name::#variant_name(..) => Some(#expr),
                },
            }
        } else {
            match &variant.fields {
                Fields::Unit => quote! {
                    #name::#variant_name => None,
                },
                Fields::Named(_) => quote! {
                    #name::#variant_name { .. } => None,
                },
                Fields::Unnamed(_) => quote! {
                    #name::#variant_name(..) => None,
                },
            }
        }
    });

    let expanded = quote! {
        impl #name {
            pub fn to_code(&self) -> Option<u8> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
