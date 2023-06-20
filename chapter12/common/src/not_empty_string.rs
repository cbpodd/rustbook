// not_empty_string.rs

use std::fmt::Display;
use std::hash::{Hash, Hasher};

use derive_getters::Getters;


#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Getters, Clone)]
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

impl Into<String> for NotEmptyString {
    fn into(self) -> String {
        self.value
    }
}

impl Into<NotEmptyString> for String {
    fn into(self) -> NotEmptyString {
        NotEmptyString::new(self)
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
    fn notemptystring_implemtnts_required_behaviors() {
        implements_behaviors::is_thread_safe::<NotEmptyString>();
        implements_behaviors::is_equatable::<NotEmptyString>();
        implements_behaviors::is_comparable::<NotEmptyString>();
        implements_behaviors::is_debuggable::<NotEmptyString>();
        implements_behaviors::is_clonable::<NotEmptyString>();
        implements_behaviors::is_hashable::<NotEmptyString>();
        implements_behaviors::is_displayable::<NotEmptyString>();
    }
}