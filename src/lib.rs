//! # My Crate
//! This is a crate aiming towards proving (only TS for now) types as text for
//! documentation purposes. You can think of this crate as a thin [`specta`]
//! wrapper that adds helper functions to the export types to manipulated
//! produced string

mod ts;
// NOTE: rust features in development
#[cfg(feature = "rust")]
mod rust;
pub use ts::generate_typedoc;
pub use ts::types::*;
