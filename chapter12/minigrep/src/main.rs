//! # Minigrep
//!
//! Minigrep CLI Program.

mod error;

mod prelude;

use std::{env, fs, path::Path};

use crate::prelude::*;
use minigreplib::{newtypes::Query, Config};

fn main() -> Result<()> {
    minigreplib::run(generate_config()?)?;
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

fn generate_config() -> Result<Config> {
    let (query, path_string) = parse_args()?;
    let contents = fs::read_to_string(Path::new(&path_string))?.try_into()?;
    Ok(Config::new(query, contents))
}
