//! # ch1402_documentation_comments
//!
//! `///` style comment adds document for the following element.
//! `//!` style comment adds document not for the following element but itself.
//! `//!` style comments are typically used in the crate root file (src/lib.rs) or module to document them as a whole.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, ch1402_documentation_comments::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
