//! # New
//!
//! Traits for creating validated new types.

use crate::input::*;

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
