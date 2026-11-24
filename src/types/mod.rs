//! Serde types mirroring the Corvo NFS-e wire format.
//!
//! Field names follow the API's `camelCase` convention via
//! `#[serde(rename_all = "camelCase")]` — except the billing plan endpoint,
//! schedules and imports, whose objects use `snake_case` and are annotated as
//! such.

/// Defines a closed string enum of the contract with `as_str()`.
macro_rules! wire_enum {
    (
        $(#[$meta:meta])*
        $name:ident {
            $( $(#[$vmeta:meta])* $variant:ident => $wire:literal ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub enum $name {
            $( $(#[$vmeta])* #[serde(rename = $wire)] $variant ),+
        }

        impl $name {
            /// The exact string the API expects on the wire.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( $name::$variant => $wire ),+
                }
            }
        }
    };
}

pub mod accountant;
pub mod billing;
pub mod certificate;
pub mod cnpj;
pub mod common;
pub mod company;
pub mod credits;
pub mod customer;
pub mod fiscal;
pub mod import;
pub mod integration;
pub mod invoice;
pub mod schedule;
pub mod service_favorite;
pub mod webhook;

pub use accountant::*;
pub use billing::*;
pub use certificate::*;
pub use cnpj::*;
pub use common::*;
pub use company::*;
pub use credits::*;
pub use customer::*;
pub use fiscal::*;
pub use import::*;
pub use integration::*;
pub use invoice::*;
pub use schedule::*;
pub use service_favorite::*;
pub use webhook::*;
