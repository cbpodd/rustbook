//! # Input
//!
//! Module containing traits for validating and sanitizing input for newtypes.

/// Trait for a type that newtypes that validate their input.
pub trait InputValidator {
    /// Input type
    type Input;

    /// Determines if an input type is valid input to a type.
    fn is_valid_input(raw_input: &Self::Input) -> bool;
}

/// Trait for a newtype that sanitizes its input.
pub trait InputSanitizer {
    /// Input type.
    type Input;

    /// Sanitize the input to the newtype.
    fn sanitize_input(raw_input: Self::Input) -> Self::Input;
}
