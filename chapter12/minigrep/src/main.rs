//! # Minigrep
//!
//! Minigrep CLI Program.

mod error;

mod prelude;

use std::{env, path::Path, process::ExitCode};

use crate::prelude::*;
use minigreplib::{newtypes::Query, Case};

fn main() -> ExitCode {
    let err = match run() {
        Ok(_) => return ExitCode::SUCCESS,
        Err(err) => err,
    };

    handle_err(err)
}

fn run() -> Result<()> {
    let (query, path_str) = parse_args()?;

    let path = Path::new(&path_str);
    let contents = minigreplib::read_file(path)?;
    let case = get_case();

    let results = minigreplib::search(query, contents, case);
    for result in results {
        println!("{result}");
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

fn get_case() -> Case {
    match env::var("IGNORE_CASE") {
        Ok(_) => Case::Insensitive,
        Err(_) => Case::Sensitive,
    }
}

fn handle_err(err: Error) -> ExitCode {
    eprintln!("{err}");
    ExitCode::FAILURE
}
