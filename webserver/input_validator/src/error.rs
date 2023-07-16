//! # Error
//!
//! Provides errors for the crate.

/// Error indicating that validation failed for a validated newtype.
#[derive(thiserror::Error, Debug, Copy, Clone)]
#[error("The provided input failed validation")]
pub struct ValidationFailedError;
