//! # Error
//!
//! Error types for minigrep library.

use std::io;

// Re-export query validation error.
pub use crate::newtypes::FileContentsValidationError;
pub use crate::newtypes::QueryValidationError;

/// Error types for the minigrep library.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Failed to read file.
    #[error("Failed to read file: {0}")]
    FailedToReadFile(#[from] io::Error),

    /// File contents validation failed.
    #[error("File contents validation failed: {0}")]
    FileContentsValidationFailed(#[from] FileContentsValidationError),
}
