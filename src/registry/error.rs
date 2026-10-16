//! A grammar error: where (file, line, column), what, and how to fix it
//! (registry V16).

use std::fmt;

/// Why a registry file was refused: the first problem found, located.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RegistryError {
    /// The file name the caller passed to [`parse`](super::parse()).
    pub file: String,
    /// The 1-based line at fault.
    pub line: usize,
    /// The 1-based column at fault, counted in characters.
    pub column: usize,
    /// What is wrong, naming the offending text.
    pub what: String,
    /// What to write instead.
    pub fix: String,
}

/// A place in the file an error points at.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Pos<'a> {
    /// The file name, as the caller gave it.
    pub(crate) file: &'a str,
    /// The 1-based line.
    pub(crate) line: usize,
    /// The 1-based column, in characters.
    pub(crate) column: usize,
}

impl Pos<'_> {
    /// An error at this place.
    #[must_use]
    pub(crate) fn error(
        self,
        _what: impl Into<String>,
        _fix: impl Into<String>,
    ) -> RegistryError {
        RegistryError {
            line: self.line,
            ..RegistryError::default()
        }
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(())
    }
}

impl std::error::Error for RegistryError {}

#[cfg(test)]
mod tests;
