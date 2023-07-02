//! Input Validator

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

/// Trait for a type that can be converted to an inner type.
pub trait IntoInner {
    /// Inner type.
    type Inner;

    /// Convert the type into its inner version, consuming the outer type.
    fn into_inner(self) -> Self::Inner;
}

/// Trait for a type which can be created from an inner type.
/// This type must be validated. If so, construction will succeed.
/// If not, construction will fail.
pub trait NewValidated
where
    Self: Sized + InputValidator,
{
    /// Inner type.
    type Inner;

    /// Create a new self.
    ///
    /// # Errors
    ///
    /// Returns the failing input if the input does not pass validation.
    fn new(raw_input: Self::Inner) -> Result<Self, Self::Inner>;
}

/// Trait for a newtype which can be created from sanitizing its input.
pub trait NewSanitized
where
    Self: Sized + InputSanitizer,
{
    /// Inner Type.
    type Inner;

    /// Create a new item.
    fn new(raw_input: Self::Inner) -> Self;
}

/// Trait for a newtype which can be created from sanitizing and validating its input.
pub trait NewSanitizedValidated
where
    Self: Sized + InputSanitizer + InputValidator,
{
    /// Inner type.
    type Inner;

    /// Create a new item by sanitizing and validating the input.
    ///
    /// # Errors
    ///
    /// Returns the failing input if the input does not pass validation.
    fn new(raw_input: Self::Inner) -> Result<Self, Self::Inner>;
}
