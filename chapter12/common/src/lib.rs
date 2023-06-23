//! # Common
//!
//! `common` is a collection of common utilities.

/// A utility class containing read-only strings
/// that are not empty or whitespace.
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
/// ```
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
pub mod not_empty_string;

mod error;
mod prelude;
