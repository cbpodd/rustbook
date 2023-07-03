//! Not empty string

use derive_more::{AsRef, Deref, Display, From, Into};
use input_validator::{
    InputSanitizer, InputValidator, IntoInner, NewSanitized, NewSanitizedValidated, NewValidated,
};

/// A test function
pub fn test() {
    let nws = NotWhitespaceString::new("test".to_owned()).expect("Construction should not fail");
    let mut inner = nws.into_inner();
    inner.push_str("test");
    println!("{inner}");
}

/// Not whitespace string
#[derive(
    Clone, Debug, Hash, Display, PartialEq, Eq, PartialOrd, Ord, Into, AsRef, Deref, IntoInner,
)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct NotWhitespaceString(String);

impl NewValidated for NotWhitespaceString {
    type Inner = String;

    fn new(raw_input: Self::Inner) -> Result<Self, Self::Inner> {
        if !NotWhitespaceString::is_valid_input(&raw_input) {
            return Err(raw_input);
        }

        Ok(NotWhitespaceString(raw_input))
    }
}

impl InputValidator for NotWhitespaceString {
    type Input = String;

    fn is_valid_input(raw_input: &Self::Input) -> bool {
        !raw_input.trim().is_empty()
    }
}

impl TryFrom<String> for NotWhitespaceString {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NotWhitespaceString::new(value)
    }
}

/// Trimmed not empty string
#[derive(
    Clone, Debug, Hash, Display, PartialEq, Eq, PartialOrd, Ord, Into, AsRef, Deref, IntoInner,
)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct TrimmedNotEmptyString(String);

impl NewSanitizedValidated for TrimmedNotEmptyString {
    type Inner = String;

    fn new(raw_input: Self::Inner) -> Result<Self, Self::Inner> {
        let sanitized_input = TrimmedNotEmptyString::sanitize_input(raw_input);

        if !TrimmedNotEmptyString::is_valid_input(&sanitized_input) {
            return Err(sanitized_input);
        }

        Ok(TrimmedNotEmptyString(sanitized_input))
    }
}

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

impl TryFrom<String> for TrimmedNotEmptyString {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TrimmedNotEmptyString::new(value)
    }
}

/// Trimmed string
#[derive(Clone, Debug, Hash, From, Display, PartialEq, Eq, PartialOrd, Ord, Into, AsRef, Deref)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct TrimmedString(String);

impl NewSanitized for TrimmedString {
    type Inner = String;

    fn new(raw_input: Self::Inner) -> Self {
        let sanitized_input = TrimmedString::sanitize_input(raw_input);
        TrimmedString(sanitized_input)
    }
}

impl IntoInner for TrimmedString {
    type Inner = String;

    fn into_inner(self) -> Self::Inner {
        self.0
    }
}

impl InputSanitizer for TrimmedString {
    type Input = String;

    fn sanitize_input(raw_input: Self::Input) -> Self::Input {
        raw_input.trim().to_owned()
    }
}
