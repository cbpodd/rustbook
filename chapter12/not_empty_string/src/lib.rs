//! Not empty string

use derive_more::{AsRef, Deref, Display, Into};
use input_validator::{InputValidator, IntoInner};

/// Not empty string
#[derive(Clone, Debug, Hash, Display, PartialEq, Eq, PartialOrd, Ord, Into, AsRef, Deref)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(try_from = "String"))]
#[cfg_attr(feature = "serde", serde(into = "String"))]
pub struct NotEmptyString(String);

impl NotEmptyString {
    /// Creates a new `NotEmptyString`.
    ///
    /// # Errors
    ///
    /// Returns the input as an error type if it is not valid.
    pub fn new(raw_input: String) -> Result<NotEmptyString, String> {
        if !NotEmptyString::is_valid_input(&raw_input) {
            return Err(raw_input);
        }

        Ok(NotEmptyString(raw_input))
    }
}

impl IntoInner for NotEmptyString {
    type Inner = String;

    fn into_inner(self) -> Self::Inner {
        self.0
    }
}

impl InputValidator for NotEmptyString {
    type Input = String;

    fn is_valid_input(raw_input: &Self::Input) -> bool {
        !raw_input.trim().is_empty()
    }
}

impl TryFrom<String> for NotEmptyString {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NotEmptyString::new(value)
    }
}
