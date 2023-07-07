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
pub fn search(query: Query, contents: FileContents) -> Vec<String> {
    let pattern_str: &str = &query;

    contents
        .lines()
        .filter(|line| line.contains(pattern_str))
        .map(std::borrow::ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        const QUERY_STR: &str = "duct";
        const CONTENTS_STR: &str = "\
Rust:
safe, fast, productive.
Pick three.";

        let query: Query =
            QUERY_STR.try_into().expect("Construction should not fail");
        let contents: FileContents = CONTENTS_STR
            .try_into()
            .expect("Construciton should not fail");

        let search_result = search(query, contents);

        assert_eq!(vec!["safe, fast, productive."], search_result);
    }
}
