//! # Error
//!
//! Error types for minigrep Command-Line Program.

use std::io;

/// Error types for the minigrep CLI Program..
#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    /// Wrong number of command-line arguments provided.
    #[error("Wrong number of command-line arguments - expected 2, actual {0}")]
    WrongNumberOfArguments(u32),

    #[error("Whitespace or empty string input")]
    WhitespaceOrEmptyInputString(
        #[from] newtypes::not_whitespace_string::Error,
    ),

    #[error("Failed to read file.")]
    FailedToReadFile(#[from] io::Error),
}
