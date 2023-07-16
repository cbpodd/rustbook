//! # Input Validator
//!
//! Various derivable traits for validated newtypes.

#[cfg(feature = "derive")]
pub use input_validator_derive::*;

pub use input::*;
pub use into_inner::*;
pub use new::*;

pub mod error;

mod input;
mod into_inner;
mod new;
