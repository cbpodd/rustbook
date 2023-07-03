//! # Minigreplib
//!
//! A minigrep program. Library for the `minigrep` CLI app.

pub mod error;

mod prelude;

use newtypes::not_whitespace_string::NotWhitespaceString;

pub use crate::error::Error;

/// Searches for a pattern in a file's contents.
pub fn search_for_pattern(
    pattern: &NotWhitespaceString,
    file_contents: &NotWhitespaceString,
) {
    println!("Searching for {pattern} in {file_contents}");
}
