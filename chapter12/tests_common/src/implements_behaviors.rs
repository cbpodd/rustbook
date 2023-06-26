//! # Implements Behaviors
//!
//! A collection of generic test functions that ensures
//! a particular struct implements common behaviors.

use std::fmt::{Debug, Display};
use std::hash::Hash;

/// Generic test ensures that a struct
/// implements the required functions to be thread-safe.
/// This test doesn't actually have to run to verify,
/// compiling is enough.
pub fn is_thread_safe<T: Sized + Send + Sync + Unpin>() {}

/// Generic test ensuring that a struct implements
/// traits to be equatable.
/// This test doesn't actually have to run to verify,
/// compiling is enough.
pub fn is_equatable<T: PartialEq + Eq>() {}

/// Generic test ensuring that a struct implements
/// traits to be comparable.
/// This test doesn't actually have to run to verify,
/// compiling is enough.
pub fn is_comparable<T: PartialOrd + Ord>() {}

/// Generic test ensuring that a struct implements
/// the debug trait.
/// This test doesn't actually have to run to verify,
/// compiling is enough.
pub fn is_debuggable<T: Debug>() {}

/// Generic test ensuring that a struct is clonable.
/// This test doesn't actually have to run to verify,
/// compiling is enough.
pub fn is_clonable<T: Clone>() {}

/// Generic test ensuring that a struct is hashable.
/// This test doesn't actually have to run to verify,
/// compiling is enough.
pub fn is_hashable<T: Hash>() {}

/// Generic test ensuring that a struct can be displayed.
/// This test doesn't actually have to run to verify,
/// compiling is enough.
pub fn is_displayable<T: Display + Debug>() {}
