//! # Minigreplib
//!
//! A minigrep program. Library for the `minigrep` CLI app.

pub mod error;
pub mod newtypes;

mod prelude;

use prelude::*;

/// Searches for a pattern in a file's contents.
///
/// # Errors
///
/// None right now.
pub fn run(config: Config) -> Result<()> {
    println!(
        "Searching for {} in {}",
        config.pattern, config.file_contents
    );

    Ok(())
}

/// Configuration for this library.
#[derive(Debug, Clone)]
pub struct Config {
    /// Query to search.
    pattern: Query,

    /// File contents to search in.
    file_contents: FileContents,
}

impl Config {
    /// Create a new configuration object.
    pub fn new(pattern: Query, file_contents: FileContents) -> Self {
        Self {
            pattern,
            file_contents,
        }
    }
}
