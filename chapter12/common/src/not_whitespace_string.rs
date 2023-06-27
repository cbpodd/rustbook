//! # Not Whitespace String
//!
//! This module contains the `NotWhitespaceString` struct, a read-only
//! string that will not be whitespace or whitespace.

use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::ops::Deref;

use serde::{Deserialize, Serialize};

/// A struct containing a read-only string that will not be whitespace.
///
/// # Examples
///
/// `NotWhitespaceString`s can be created from other strings to ensure they
/// are not whitespace.
/// ```
/// # use common::not_whitespace_string::NotWhitespaceString;
/// let not_whitespace_string = NotWhitespaceString::try_from("Not Whitespace String".to_owned())
///     .expect("Construction should succeed");
/// ```
///
/// `NotWhitespaceString`s will error if created from an whitespace string.
/// ```
/// # use common::not_whitespace_string::NotWhitespaceString;
/// let not_whitespace_string = NotWhitespaceString::try_from("".to_owned())
///     .expect_err("Construction should error");
/// ```
///
/// The value in a `NotWhitespaceString` cannot be modified.
/// ```compile_fail
/// # use common::not_whitespace_string::NotWhitespaceString;
/// let not_whitespace_string = NotWhitespaceString::try_from("Not Whitespace String".to_owned())
///     .expect("Construction should succeed");
///
/// not_whitespace_string.value().push_str("Cannot push!");
/// ```
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Deserialize, Serialize)]
// TODO: Perhaps Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug, Deserialize, Serialize can all be conditionally compiled in macro?
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct NotWhitespaceString(String);
// TODO: Try to make this a macro over trait Accept<String> for NotWhitespaceString

impl NotWhitespaceString {
    /// Construct a new `NotWhitespaceString` from another string.
    ///
    /// # Errors
    ///
    /// Returns an error if a `NotWhitespaceString` is constructed
    /// with an empty or whitespace string.
    ///
    /// # Examples
    ///
    /// `NotWhitespaceString`s can be created from other strings to ensure they
    /// are not whitespace.
    /// ```
    /// # use common::not_whitespace_string::NotWhitespaceString;
    /// let not_whitespace_string = NotWhitespaceString::new(String::from("Not Whitespace String"))
    ///     .expect("String should pass construction.");
    /// ```
    ///
    /// `NotWhitespaceString`s will fail if created from an whitespace string,
    /// returning the original string as the error.
    /// ```
    /// # use common::not_whitespace_string::NotWhitespaceString;
    ///
    /// let err_string = NotWhitespaceString::new(String::new())
    ///     .expect_err("String should fail construction.");
    ///
    /// assert_eq!(String::new(), err_string);
    /// ```
    pub fn new(raw_string: String) -> Result<Self, String> {
        if raw_string.trim().is_empty() {
            return Err(raw_string);
        }

        Ok(Self(raw_string))
    }
}

impl TryFrom<String> for NotWhitespaceString {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NotWhitespaceString::new(value)
    }
}

impl From<NotWhitespaceString> for String {
    fn from(val: NotWhitespaceString) -> Self {
        val.0
    }
}

impl Display for NotWhitespaceString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Hash for NotWhitespaceString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Deref for NotWhitespaceString {
    type Target = String;

    #[allow(clippy::explicit_deref_methods)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod unit_tests {
    use super::NotWhitespaceString;

    use serde::{Deserialize, Serialize};
    use tests_common::implements_behaviors;

    #[test]
    fn notwhitespacestring_implements_required_behaviors() {
        implements_behaviors::is_thread_safe::<NotWhitespaceString>();
        implements_behaviors::is_equatable::<NotWhitespaceString>();
        implements_behaviors::is_comparable::<NotWhitespaceString>();
        implements_behaviors::is_debuggable::<NotWhitespaceString>();
        implements_behaviors::is_clonable::<NotWhitespaceString>();
        implements_behaviors::is_hashable::<NotWhitespaceString>();
        implements_behaviors::is_displayable::<NotWhitespaceString>();
        implements_behaviors::is_serializable::<NotWhitespaceString>();
    }

    #[test]
    fn can_be_created_from_nonwhitespace_string() {
        let underlying_string = "An underlying string";
        let nws = NotWhitespaceString::new(String::from(underlying_string))
            .expect("String construction should not fail");

        assert_eq!(underlying_string, *nws);
    }

    #[test]
    fn returns_err_with_empty_string() {
        let err_string =
            NotWhitespaceString::new(String::new()).expect_err("Expect construction to error");

        assert_eq!(String::new(), err_string);
    }

    #[test]
    fn returns_err_with_whitespace_string() {
        let whitespace_string = String::from(" \t \n ");
        let err_string = NotWhitespaceString::new(whitespace_string.clone())
            .expect_err("Expect construction to error");

        assert_eq!(whitespace_string, err_string);
    }

    #[test]
    fn deserializes_from_json_string() {
        let test_str = "Not Whitespace String";
        let json_str = format!("{{\"nws\":\"{test_str}\"}}");

        let deserialized_struct: TestStruct =
            serde_json::from_str(&json_str).expect("Deserialization should not fail");

        assert_eq!(test_str, *deserialized_struct.nws);
    }

    #[test]
    fn deserialization_fails_if_string_is_whitespace() {
        let test_str = "  ";
        let json_str = format!("{{\"nws\":\"{test_str}\"}}");

        let err =
            serde_json::from_str::<TestStruct>(&json_str).expect_err("Deserialization should fail");

        #[allow(clippy::string_slice)]
        let err_string = err.to_string();

        let test_str_subsection = err_string
            .get(..test_str.len())
            .expect("Slicing off extra error end for testing");

        assert_eq!(test_str, test_str_subsection);
    }

    #[test]
    fn serializes_to_json_string() {
        let test_str = "Not Whitespace String";
        let expected_json_str = format!("{{\"nws\":\"{test_str}\"}}");

        let test_struct = TestStruct {
            nws: NotWhitespaceString::try_from(test_str.to_owned())
                .expect("Construction should not fail"),
        };

        let actual_json_str =
            serde_json::to_string(&test_struct).expect("Serialization should not fail");

        assert_eq!(expected_json_str, actual_json_str);
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct TestStruct {
        nws: NotWhitespaceString,
    }
}
