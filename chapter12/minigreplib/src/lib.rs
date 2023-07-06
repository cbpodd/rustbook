//! # Minigreplib
//!
//! A minigrep program. Library for the `minigrep` CLI app.

pub mod error;
pub mod newtypes;

mod prelude;

use std::{fs, path::Path};

use prelude::*;

/// Read the file at a specified path into a string.
///
/// # Errors
///
/// Errors if reading to the file failed, or if the contents cannot
/// be turned into a `FileContents`.
pub fn read_file(path: &Path) -> Result<FileContents> {
    Ok(fs::read_to_string(path)?.try_into()?)
}

/// Searches for a pattern in a file's contents.
///
/// # Errors
///
/// None right now.
pub fn search(config: SearchConfig) -> Result<Vec<String>> {
    println!(
        "Searching for {} in {}",
        config.pattern, config.file_contents
    );

    Ok(Vec::new())
}

/// Configuration for this library.
#[derive(Debug, Clone)]
pub struct SearchConfig {
    /// Query to search.
    pattern: Query,

    /// File contents to search in.
    file_contents: FileContents,
}

impl SearchConfig {
    /// Create a new configuration object.
    pub fn new(pattern: Query, file_contents: FileContents) -> Self {
        Self {
            pattern,
            file_contents,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        const CONTENTS_STR: &str = "\
        Rust:
        safe, fast, productive.
        Pick three.";

        let query: Query = create_newtype("duct");
        let contents: FileContents = create_newtype(CONTENTS_STR);
    }

    fn create_newtype<T>(input: &str) -> T
    where
        T: TryFrom<String>,
        T::Error: std::fmt::Debug,
    {
        input
            .to_owned()
            .try_into()
            .expect("Construction should not fail")
    }
}
