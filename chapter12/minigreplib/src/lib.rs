//! # Minigreplib
//!
//! A minigrep program. Library for the `minigrep` CLI app.

pub mod error;
pub mod newtypes;

mod prelude;

use std::{fs, path::Path};

use prelude::*;

/// Case for a search result.
#[derive(Debug, Copy, Clone)]
pub enum Case {
    /// Case-sensitive searching: Duct != duct
    Sensitive,

    /// Case-insensitive searching: Duct == duct
    Insensitive,
}

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
pub fn search(query: Query, contents: FileContents, case: Case) -> Vec<String> {
    let lines = contents.lines();
    let filtered = match case {
        Case::Sensitive => match_lines_case_sensitive(lines.collect(), query),
        Case::Insensitive => {
            match_lines_case_insensitive(lines.collect(), query)
        }
    };

    filtered
        .into_iter()
        .map(std::borrow::ToOwned::to_owned)
        .collect()
}

fn match_lines_case_insensitive(lines: Vec<&str>, query: Query) -> Vec<&str> {
    lines
        .into_iter()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

fn match_lines_case_sensitive(lines: Vec<&str>, query: Query) -> Vec<&str> {
    let pattern_str: &str = &query;
    lines
        .into_iter()
        .filter(|line| line.contains(pattern_str))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        const QUERY_STR: &str = "duct";
        const CONTENTS_STR: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let query: Query =
            QUERY_STR.try_into().expect("Construction should not fail");
        let contents: FileContents = CONTENTS_STR
            .try_into()
            .expect("Construciton should not fail");

        let search_result = search(query, contents, Case::Sensitive);

        assert_eq!(vec!["safe, fast, productive."], search_result);
    }

    #[test]
    fn case_insensitive() {
        const QUERY_STR: &str = "rUsT";
        const CONTENTS_STR: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let query: Query =
            QUERY_STR.try_into().expect("Construction should not fail");
        let contents: FileContents = CONTENTS_STR
            .try_into()
            .expect("Construciton should not fail");

        let search_result = search(query, contents, Case::Insensitive);

        assert_eq!(vec!["Rust:", "Trust me."], search_result);
    }
}
