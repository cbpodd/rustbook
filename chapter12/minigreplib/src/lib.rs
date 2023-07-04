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
pub fn search_for_pattern(
    pattern: Query,
    file_contents: FileContents,
) -> Result<()> {
    println!("Searching for {pattern} in {file_contents}");
    Ok(())
}
