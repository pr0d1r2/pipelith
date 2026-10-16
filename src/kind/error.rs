//! Why a row was refused: the row's line, what is wrong (naming the key,
//! system or cache at fault) and the fix (V31). The row knows its line,
//! not its file: [`KindError::located`] adds the file name.

use std::fmt;

use crate::registry::RegistryError;

/// A refused `## workflows` row.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KindError {
    /// The 1-based line of the row.
    pub line: usize,
    /// What is wrong, naming the offending key or value.
    pub what: String,
    /// What to write instead.
    pub fix: String,
}

impl KindError {
    /// An error for the row at `line`.
    #[must_use]
    pub fn new(
        line: usize,
        what: impl Into<String>,
        fix: impl Into<String>,
    ) -> Self {
        Self {
            line,
            what: what.into(),
            fix: fix.into(),
        }
    }

    /// The same error as a registry error in `file`, at the row's first
    /// column, so a forge crate reports every refusal one way.
    #[must_use]
    pub fn located(&self, file: &str) -> RegistryError {
        RegistryError {
            file: file.to_owned(),
            line: self.line,
            column: 1,
            what: self.what.clone(),
            fix: self.fix.clone(),
        }
    }
}

impl fmt::Display for KindError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { line, what, fix } = self;
        write!(f, "line {line}: {what} ({fix})")
    }
}

impl std::error::Error for KindError {}

#[cfg(test)]
mod tests;
