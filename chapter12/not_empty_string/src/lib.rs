//! # Not empty string
//!
//! Some various test newtypes to try out my derivable macros.

use derive_more::{AsRef, Deref, Display, From, Into};
use input_validator::{
    InputSanitizer, InputValidator, IntoInner, NewSanitized,
    NewSanitizedValidated, NewValidated, TryFrom,
};

fn printer(test: &str) {
    println!("{test}");
}

/// A test function
pub fn test() {
    let nws = NotWhitespaceString::try_from("test".to_owned())
        .expect("Construction should not fail");
    printer(&nws);

    let inner = nws.into_inner();
    printer(&inner);
}

/// Not whitespace string
#[derive(
    Clone,
    Debug,
    Hash,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Into,
    AsRef,
    Deref,
    IntoInner,
    NewValidated,
    TryFrom,
)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct NotWhitespaceString(String);

impl InputValidator for NotWhitespaceString {
    type Input = String;

    fn is_valid_input(raw_input: &Self::Input) -> bool {
        !raw_input.trim().is_empty()
    }
}

/// Trimmed not empty string
#[derive(
    Clone,
    Debug,
    Hash,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Into,
    AsRef,
    Deref,
    IntoInner,
    TryFrom,
    NewSanitizedValidated,
)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct TrimmedNotEmptyString(String);

impl InputValidator for TrimmedNotEmptyString {
    type Input = String;

    fn is_valid_input(raw_input: &Self::Input) -> bool {
        !raw_input.is_empty()
    }
}

impl InputSanitizer for TrimmedNotEmptyString {
    type Input = String;

    fn sanitize_input(raw_input: Self::Input) -> Self::Input {
        raw_input.trim().to_owned()
    }
}

/// Trimmed string
#[derive(
    Clone,
    Debug,
    Hash,
    From,
    Display,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Into,
    AsRef,
    Deref,
    IntoInner,
    NewSanitized,
)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct TrimmedString(String);

impl InputSanitizer for TrimmedString {
    type Input = String;

    fn sanitize_input(raw_input: Self::Input) -> Self::Input {
        raw_input.trim().to_owned()
    }
}
