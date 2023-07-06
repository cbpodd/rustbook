//! # Minigrep
//!
//! Minigrep CLI Program.

mod error;

mod prelude;

use std::{env, path::Path};

use crate::prelude::*;
use minigreplib::{newtypes::Query, SearchConfig};

fn main() -> Result<()> {
    let (query, path_str) = parse_args()?;

    let path = Path::new(&path_str);
    let contents = minigreplib::read_file(path)?;

    let config = SearchConfig::new(query, contents);
    let results = minigreplib::search(config)?;
    for result in results {
        println!("Result: {result}");
    }
    Ok(())
}

fn parse_args() -> Result<(Query, String)> {
    let mut args = env::args();

    drop(
        args.next()
            .expect("Program arguments should always contain program name"),
    );

    let query: Query = args.next().ok_or(Error::NoArguments)?.try_into()?;
    let path_str = args.next().ok_or(Error::WrongNumberOfArguments(1))?;

    if Option::is_some(&args.next()) {
        return Err(Error::WrongNumberOfArguments(3));
    };

    Ok((query, path_str))
}
