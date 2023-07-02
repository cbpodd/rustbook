//! # Minigrep
//!
//! Minigrep CLI Program.

mod error;

mod prelude;

use std::{env, fs, path::Path};

use crate::prelude::*;
use newtypes::not_whitespace_string::NotWhitespaceString;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let query_str = args.get(1).expect("Must have query");
    let query = NotWhitespaceString::try_from(query_str.clone())?;

    let file_path_string = args.get(2).expect("Must have file path");
    let file_path = Path::new(file_path_string);

    println!("Looking for {query} in {}", file_path.display());

    let contents_string = fs::read_to_string(file_path)?;
    let contents = NotWhitespaceString::try_from(contents_string)?;

    minigreplib::search_for_pattern(&query, &contents);

    Ok(())
}
