//! # Not Whitespace String
//!
//! This module contains the `NotWhitespaceString` struct, a read-only
//! string that will not be whitespace or whitespace.

use std::fmt::Display;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

/// A struct containing a read-only string that will not be whitespace.
///
/// # Examples
///
/// `NotWhitespaceString`s can be created from other strings to ensure they
/// are not whitespace.
/// ```
/// # use common::not_whitespace_string::NotWhitespaceString;
/// let not_whitespace_string = NotWhitespaceString::try_from("Not Whitespace String")
///     .expect("Construction shoudl succeed");
/// ```
///
/// `NotWhitespaceString`s will error if created from an whitespace string.
/// ```
/// # use common::not_whitespace_string::NotWhitespaceString;
/// let not_whitespace_string = NotWhitespaceString::try_from("")
///     .expect_err("Construction should error");
/// ```
///
/// The value in a `NotWhitespaceString` cannot be modified.
/// ```compile_fail
/// # use common::not_whitespace_string::NotWhitespaceString;
/// let not_whitespace_string = NotWhitespaceString::new(String::from("Not Whitespace String"))
///     .expect("Construction should succeed");
///
/// not_whitespace_string.value().push_str("Cannot push!");
/// ```
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Deserialize, Serialize)]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct NotWhitespaceString(String);

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

    /// Get a reference to the inner string.
    ///
    /// # Examples
    ///
    /// This function gets a reference to the inner string.
    /// ```
    /// # use common::not_whitespace_string::NotWhitespaceString;
    /// let input_str = "Not Whitespace String";
    /// let not_whitespace_string = NotWhitespaceString::new(String::from(input_str))
    ///     .expect("String should pass construction.");
    ///
    /// assert_eq!(input_str, not_whitespace_string.get());
    /// ```
    ///
    /// The reference is not mutable.
    /// ```compile_fail
    /// # use common::not_whitespace_string::NotWhitespaceString;
    /// let input_str = "Not Whitespace String";
    /// let not_whitespace_string = NotWhitespaceString::new(String::from(input_str))
    ///     .expect("String should pass construction.");
    ///
    /// not_whitespace_string.get().push_str("Another string");
    /// ```
    pub fn get(&self) -> &str {
        &self.0
    }

    /// Consume this `NotWhitespaceString`, returning the wrapped string.
    ///
    /// # Examples
    ///
    /// This function takes ownership of the inner string.
    /// ```
    /// # use common::not_whitespace_string::NotWhitespaceString;
    /// let input_string = String::from("Not Whitespace String");
    /// let not_whitespace_string = NotWhitespaceString::new(input_string.clone())
    ///     .expect("String should pass construction.");
    ///
    /// let output_string = not_whitespace_string.into_inner();
    ///
    /// assert_eq!(input_string, output_string);
    /// ```
    ///
    /// This function consumes the `NotWhitespaceString`.
    /// ```compile_fail
    /// # use common::not_whitespace_string::NotWhitespaceString;
    /// let input_string = String::from("Not Whitespace String");
    /// let not_whitespace_string = NotWhitespaceString::new(input_string.clone())
    ///     .expect("String should pass construction.");
    ///
    /// let output_string = not_whitespace_string.into_inner();
    ///
    /// not_whitespace_string.get(); // This will not compile, as into_inner() consumes the string.
    /// ```
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl TryFrom<String> for NotWhitespaceString {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        NotWhitespaceString::new(value)
    }
}

impl<'a> TryFrom<&'a str> for NotWhitespaceString {
    type Error = &'a str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match NotWhitespaceString::try_from(value.to_owned()) {
            Ok(nws) => Ok(nws),
            Err(_) => Err(value),
        }
    }
}

impl From<NotWhitespaceString> for String {
    fn from(val: NotWhitespaceString) -> Self {
        val.0
    }
}

impl Display for NotWhitespaceString {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.get().fmt(f)
    }
}

impl Hash for NotWhitespaceString {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state);
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
    }

    #[test]
    fn can_be_created_from_nonwhitespace_string() {
        let underlying_string = "An underlying string";
        let nes = NotWhitespaceString::new(String::from(underlying_string))
            .expect("String construction should not fail");

        assert_eq!(underlying_string, nes.get());
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
            .expect_err("Expet construction to error");

        assert_eq!(whitespace_string, err_string);
    }

    #[test]
    fn deserializes_from_json_string() {
        let test_str = "Not Whitespace String";
        let json_str = format!("{{\"nws\":\"{test_str}\"}}");

        let deserialized_struct: TestStruct =
            serde_json::from_str(&json_str).expect("Deserialization should not fail");

        assert_eq!(test_str, deserialized_struct.nws.get());
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
            nws: NotWhitespaceString::try_from(test_str).expect("Construction should not fail"),
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
