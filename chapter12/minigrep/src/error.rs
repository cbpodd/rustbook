//! # Error
//!
//! Error types for minigrep Command-Line Program.

use std::io;

use minigreplib::newtypes::{
    FileContentsValidationError, QueryValidationError,
};

/// Error types for the minigrep CLI Program..
#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    /// No command-line arguments provided.
    #[error("No command-line arguments provided. Expected 2")]
    NoArguments,

    /// Wrong number of command-line arguments provided.
    #[error("Wrong number of command-line arguments - expected 2, actual {0}")]
    WrongNumberOfArguments(u32),

    #[error("Failed to read file.")]
    FailedToReadFile(#[from] io::Error),

    #[error("Query validation failed.")]
    QueryValidationFailed(#[from] QueryValidationError),

    #[error("File contents validation failed.")]
    FileContentsValidationFailed(#[from] FileContentsValidationError),

    #[error("Query searching failed.")]
    QuerySearchFailed(#[from] minigreplib::error::Error),
}
