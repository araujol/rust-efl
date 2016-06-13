//! eina library: Data types and low-level/basic abstractions
//! Hash and List functionality is enabled by the "eina" feature
//! Otherwise, only eina_minimal is loaded, which provides EinaBool
pub use self::eina_minimal::*;
#[cfg(feature = "eina")]
pub use self::eina_full::*;

mod eina_minimal;
#[cfg(feature = "eina")]
mod eina_full;
