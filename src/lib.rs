#![warn(missing_docs)]
//! A derive macro for automatically implementing code conversion for enums
//!
//! This crate provides two derive macros:
//! - `ToCode` generates a `to_code()` method for converting enum variants to `Option<u8>` values
//! - `TryFromCode` generates a `try_from_code()` method for creating enum variants from `u8` values
//!
//! Both macros work with unit variants, named fields, and tuple variants. For using TryFromCode fields in variants must implement
//! the `Default` trait as they will be initialized with default values during conversion from codes.
//!
//! # Example
//! ```
//! use complex_enum_macros::{ToCode, TryFromCode};
//!
//! #[derive(ToCode, TryFromCode, Debug, PartialEq)]
//! #[repr(u8)]
//! enum Command {
//!     Unknown,
//!     Start = 0x01,
//!     SetConfig { value: Option<u32> } = 0x02,
//!     SendData(String) = 0x03,
//!     Stop = 0x04,
//! }
//!
//! // Convert enum to code
//! let cmd = Command::SetConfig { value: Some(42) };
//! assert_eq!(cmd.to_code(), Some(0x02));
//!
//! // Create enum from code and modify
//! let mut cmd = Command::try_from_code(0x02).unwrap();
//! match cmd {
//!     Command::SetConfig { ref mut value } => *value = Some(42),
//!     _ => unreachable!(),
//! }
//! assert_eq!(cmd.to_code(), Some(0x02));
//! match cmd {
//!     Command::SetConfig { value } => assert_eq!(value, Some(42)),
//!     _ => unreachable!(),
//! }
//!
//! // Unknown codes return None
//! assert_eq!(Command::try_from_code(0xFF), None);
//!
//! // Variants without codes return None
//! assert_eq!(Command::Unknown.to_code(), None);
//! ```
//!
//! # Features
//! - Automatic code conversion in both directions
//! - Support for all enum variant types
//! - Default initialization of fields
//!
//! # Requirements
//! - Enum must have `#[repr(u8)]`
//! - For TryFromCode field types must implement `Default`
//! - Variants with codes must have explicit discriminants

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/// Derives the `ToCode` trait for an enum.
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
            #[doc = "Converts the enum to its `u8` representation."]
            pub fn to_code(&self) -> Option<u8> {
                match self {
                    #(#match_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Derives the `TryFromCode` trait for an enum.
#[proc_macro_derive(TryFromCode)]
pub fn derive_try_from_code(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("TryFromCode can only be derived for enums"),
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        if let Some((_, expr)) = &variant.discriminant {
            match &variant.fields {
                Fields::Unit => quote! {
                    #expr => Some(Self::#variant_name),
                },
                Fields::Named(fields) => {
                    let field_names = fields.named.iter().map(|f| &f.ident);
                    quote! {
                        #expr => Some(Self::#variant_name {
                            #(#field_names: Default::default(),)*
                        }),
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_count = fields.unnamed.len();
                    let defaults = std::iter::repeat(quote!(Default::default())).take(field_count);
                    quote! {
                        #expr => Some(Self::#variant_name(#(#defaults),*)),
                    }
                }
            }
        } else {
            quote!()
        }
    });

    let expanded = quote! {
        impl #name {
            #[doc = "Tries to convert a `u8` code to an enum variant."]
            pub fn try_from_code(code: u8) -> Option<Self> {
                match code {
                    #(#match_arms)*
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(expanded)
}
