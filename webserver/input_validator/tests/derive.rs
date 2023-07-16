#![cfg(feature = "derive")]

use input_validator::{
    error::ValidationFailedError, InputValidator, IntoInner, NewValidated,
    TryFrom, TryFromStr,
};

#[derive(Clone, Debug, NewValidated, IntoInner, TryFrom, TryFromStr)]
struct NotEmptyStringStartsWithA(String);

impl InputValidator for NotEmptyStringStartsWithA {
    type Input = String;

    fn is_valid_input(raw_input: &Self::Input) -> bool {
        let first_char = raw_input
            .trim()
            .to_lowercase()
            .chars()
            .next()
            .unwrap_or('b'); // None variant indicates failure, and anything other than 'a' will fail

        first_char == 'a'
    }
}

#[test]
fn accept_starts_with_a() {
    let res = NotEmptyStringStartsWithA::new("A string".to_owned());
    assert!(res.is_some());
}

#[test]
fn reject_starts_with_b() {
    let res = NotEmptyStringStartsWithA::new("B string".to_owned());
    assert!(res.is_none());
}

#[test]
fn reject_empty() {
    let res = NotEmptyStringStartsWithA::new(String::new());
    assert!(res.is_none());
}

#[test]
fn into_inner_returns_value() {
    let string = String::from("A test string");
    let starts_with_a = NotEmptyStringStartsWithA::new(string.clone())
        .expect("Construction will succeed");
    let inner = starts_with_a.into_inner();
    assert_eq!(string, inner);
}

#[test]
fn try_from_returns_error_on_failure() {
    let _: ValidationFailedError =
        NotEmptyStringStartsWithA::try_from("B string".to_owned())
            .expect_err("Construction will error");
}

#[test]
fn try_from_str() {
    let _ = NotEmptyStringStartsWithA::try_from("A string")
        .expect("Construction will succeed");
}
