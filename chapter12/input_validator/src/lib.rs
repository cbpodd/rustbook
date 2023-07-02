//! Input Validator

/// Trait for a type that newtypes that validate their input.
pub trait InputValidator {
    /// Input type
    type Input;

    /// Determines if an input type is valid input to a type.
    fn is_valid_input(raw_input: &Self::Input) -> bool;
}

/// Trait for a type that can be converted to an inner type.
pub trait IntoInner {
    /// Inner type.
    type Inner;

    /// Convert the type into its inner version, consuming the outer type.
    fn into_inner(self) -> Self::Inner;
}
