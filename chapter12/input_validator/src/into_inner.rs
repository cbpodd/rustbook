//! # Into Inner
//!
//! Trait for newtypes to be converted into their wrapper type.
//!
/// Trait for a type that can be converted to an inner type.
pub trait IntoInner {
    /// Inner type.
    type Inner;

    /// Convert the type into its inner version, consuming the outer type.
    fn into_inner(self) -> Self::Inner;
}
