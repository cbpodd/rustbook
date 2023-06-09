// tests_common.rs

#[cfg(test)]

use std::fmt::Debug;
use std::hash::Hash;

pub fn is_thread_safe<T: Sized + Send + Sync + Unpin>() {}

pub fn is_equatable<T: PartialEq + Eq>() {}

pub fn is_comparable<T: PartialOrd + Ord>() {}

pub fn is_debuggable<T: Debug>() {}

pub fn is_clonable<T: Clone>() {}

pub fn is_hashable<T: Hash>() {}
