// not_empty_string.rs

use std::fmt::Display;
use std::hash::{Hash, Hasher};

use derive_getters::{Getters, Dissolve};


#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Getters, Dissolve, Clone)]
pub struct NotEmptyString {
    value: String,
}

impl NotEmptyString {
    pub fn new(raw_string: String) -> Self {
        if raw_string.trim().is_empty() {
            panic!("NotEmptyString must not contain an empty or whitespace-only string.");
        }

        Self {
            value: raw_string,
        }
    }
}

impl From<&str> for NotEmptyString {
    fn from(raw_string: &str) -> Self {
        Self::new(String::from(raw_string))
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
    use crate::tests_common;

    use super::NotEmptyString;

    #[test]
    fn notemptystring_implemtnts_required_behaviors() {
        tests_common::is_thread_safe::<NotEmptyString>();
        tests_common::is_equatable::<NotEmptyString>();
        tests_common::is_comparable::<NotEmptyString>();
        tests_common::is_debuggable::<NotEmptyString>();
        tests_common::is_clonable::<NotEmptyString>();
        tests_common::is_hashable::<NotEmptyString>()
    }
}