//! # Not Empty String
//!
//! This module contains the `NotEmptyString` struct, a read-only
//! string that will not be empty or whitespace.

use std::fmt::Display;
use std::hash::{Hash, Hasher};

use derive_getters::{Dissolve, Getters};

/// A struct containing a read-only string that will not be empty.
/// Constructing this string in any way with an empty or whitespace
/// string will panic.
///
/// # Examples
///
/// `NotEmptyString`s can be created from other strings to assert they
/// are not empty.
/// ```
/// # use common::not_empty_string::NotEmptyString;
/// let not_empty_string = NotEmptyString::from("Not Empty String");
/// ```
///
/// `NotEmptyString`s will panic if created from an empty string.
/// ```should_panic
/// # use common::not_empty_string::NotEmptyString;
/// let not_empty_string = NotEmptyString::from("");
/// ```
///
/// The value in a `NotEmptyString` cannot be modified.
/// ```compile_fail
/// # use common::not_empty_string::NotEmptyString;
/// let not_empty_string = NotEmptyString::from("Not Empty String");
///
/// not_empty_string.value().push_str("Cannot push!");
/// ```
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Getters, Clone, Dissolve)]
pub struct NotEmptyString {
    value: String,
}

impl NotEmptyString {
    /// Construct a new `NotEmptyString` from another string.
    ///
    /// # Panics
    /// Panics if the string provided is empty or whtiespace.
    ///
    /// # Examples
    ///
    /// `NotEmptyString`s can be created from other strings to assert they
    /// are not empty.
    /// ```
    /// # use common::not_empty_string::NotEmptyString;
    /// let not_empty_string = NotEmptyString::new(String::from("Not Empty String"));
    /// ```
    ///
    /// `NotEmptyString`s will panic if created from an empty string.
    /// ```should_panic
    /// # use common::not_empty_string::NotEmptyString;
    /// let not_empty_string = NotEmptyString::new(String::new());
    /// ```
    pub fn new(raw_string: String) -> Self {
        assert!(
            !raw_string.trim().is_empty(),
            "NotEmptyString must not contain an empty or whitespace-only string."
        );

        Self { value: raw_string }
    }
}

impl From<&str> for NotEmptyString {
    fn from(raw_string: &str) -> Self {
        Self::from(String::from(raw_string))
    }
}

impl From<String> for NotEmptyString {
    fn from(raw_string: String) -> Self {
        Self::new(raw_string)
    }
}

impl From<NotEmptyString> for String {
    fn from(not_empty_string: NotEmptyString) -> Self {
        not_empty_string.value
    }
}

impl Display for NotEmptyString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl Hash for NotEmptyString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value().hash(state);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::NotEmptyString;

    use tests_common::implements_behaviors;

    #[test]
    fn notemptystring_implements_required_behaviors() {
        implements_behaviors::is_thread_safe::<NotEmptyString>();
        implements_behaviors::is_equatable::<NotEmptyString>();
        implements_behaviors::is_comparable::<NotEmptyString>();
        implements_behaviors::is_debuggable::<NotEmptyString>();
        implements_behaviors::is_clonable::<NotEmptyString>();
        implements_behaviors::is_hashable::<NotEmptyString>();
        implements_behaviors::is_displayable::<NotEmptyString>();
    }

    #[test]
    fn can_be_created_from_nonempty_string() {
        let underlying_string = "An underlying string";
        let nes = NotEmptyString::new(String::from(underlying_string));

        assert_eq!(underlying_string, nes.value());
    }

    #[test]
    #[should_panic]
    fn panics_with_empty_string() {
        let _panics = NotEmptyString::new(String::new());
    }

    #[test]
    #[should_panic]
    fn panics_with_whitespace_string() {
        let _panics = NotEmptyString::new(String::from(" \t \n "));
    }
}
