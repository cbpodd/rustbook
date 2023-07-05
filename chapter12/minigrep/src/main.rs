//! # Minigrep
//!
//! Minigrep CLI Program.

mod error;

mod prelude;

use std::{env, fs, path::Path};

use crate::prelude::*;
use minigreplib::{
    newtypes::{FileContents, Query},
    ConfigBuilder,
};

fn main() -> Result<()> {
    let (query, path_string) = parse_args()?;
    let file_path = Path::new(&path_string);
    let contents = read_file_contents(file_path)?;
    let config = ConfigBuilder::default()
        .pattern(query)
        .file_contents(contents)
        .build()
        .expect("Creating config should not fail");

    minigreplib::run(config)?;
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

fn read_file_contents(path: &Path) -> Result<FileContents> {
    let contents_string = fs::read_to_string(path)?;
    Ok(FileContents::try_from(contents_string)?)
}
