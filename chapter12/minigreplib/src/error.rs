//! # Error
//!
//! Error types for minigrep library.

// Re-export query validation error.
pub use crate::newtypes::FileContentsValidationError;
pub use crate::newtypes::QueryValidationError;

/// Error types for the minigrep library.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// General error. Contains the string message for the error.
    /// TODO: Remove.
    #[error("General error: {0}")]
    General(String),
}
