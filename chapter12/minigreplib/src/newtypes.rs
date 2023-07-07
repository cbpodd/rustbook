//! # Newtypes
//!
//! Newtypes used in this library.

use derive_more::{Deref, Display, Into};
use input_validator::{
    InputSanitizer, InputValidator, IntoInner, NewSanitizedValidated,
    NewValidated, TryFrom, TryFromStr,
};

/// Minigrep query. Must be a non-empty or whitespace string.
#[derive(
    Debug,
    Clone,
    Display,
    Into,
    Deref,
    IntoInner,
    NewSanitizedValidated,
    TryFrom,
    TryFromStr,
)]
#[error_type(QueryValidationError)]
pub struct Query(String);

impl InputSanitizer for Query {
    type Input = String;

    fn sanitize_input(raw_input: Self::Input) -> Self::Input {
        raw_input.trim().to_owned()
    }
}

impl InputValidator for Query {
    type Input = String;

    fn is_valid_input(raw_input: &Self::Input) -> bool {
        !raw_input.is_empty()
    }
}

/// Error type indicating query validation has failed.
#[derive(thiserror::Error, Debug)]
#[error("Query validation failed. Invalid query: {0}")]
pub struct QueryValidationError(String);

/// File contents. Must be a non-empty or whitespace string.
#[derive(
    Debug,
    Display,
    Clone,
    Into,
    Deref,
    IntoInner,
    NewValidated,
    TryFrom,
    TryFromStr,
)]
#[error_type(FileContentsValidationError)]
pub struct FileContents(String);

impl InputValidator for FileContents {
    type Input = String;

    fn is_valid_input(raw_input: &Self::Input) -> bool {
        !raw_input.trim().is_empty()
    }
}

/// Error type indicating that file contents validation has failed.
#[derive(thiserror::Error, Debug)]
#[error("File contents validation failed. Invalid file contents: {0}")]
pub struct FileContentsValidationError(String);
