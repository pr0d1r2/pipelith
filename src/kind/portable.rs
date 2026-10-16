//! What a forge adds beyond the catalogue, and the portability check
//! (V33, `.:V4`, `.:V5`): a row using a forge-only kind or arg key is
//! valid on that forge only, and `portable` reports each such use.

use std::fmt;

use super::catalogue::{ArgSpec, KindSpec};
use super::error::KindError;
use super::raw::RawRow;

/// The kinds and arg keys a forge crate declares on top of the
/// catalogue. A forge-only key is accepted on every kind of that forge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Extras<'a> {
    /// Forge-only kinds.
    pub kinds: &'a [KindSpec],
    /// Forge-only arg keys.
    pub keys: &'a [ArgSpec],
}

/// One reason a row renders on its own forge only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Finding {
    /// The row's kind is forge-only.
    Kind {
        /// The 1-based line of the row.
        line: usize,
        /// The kind name.
        kind: String,
    },
    /// The row gives a forge-only arg key.
    Key {
        /// The 1-based line of the row.
        line: usize,
        /// The key.
        key: String,
    },
}

impl<'a> Extras<'a> {
    /// The forge-only kind `name`, if the forge declares one.
    #[must_use]
    pub fn kind(&self, name: &str) -> Option<&'a KindSpec> {
        let _ = (self, name);
        None
    }
}

/// The kind `row` names: a catalogue kind, else a forge-only one.
///
/// # Errors
///
/// A kind neither knows, listing the kinds a row may name.
pub fn resolve<'a>(
    row: &RawRow,
    extras: &Extras<'a>,
) -> Result<&'a KindSpec, KindError> {
    let _ = (row, extras);
    Err(KindError::default())
}

/// One finding per forge-only kind and per forge-only arg key of `row`;
/// empty when the row renders on every forge (V33). A key the row's
/// kind takes itself is never forge-only; an unknown kind or key is
/// not a finding but a [`Row::check`](super::Row::check) error.
#[must_use]
pub fn portable(row: &RawRow, extras: &Extras<'_>) -> Vec<Finding> {
    let _ = (row, extras);
    Vec::new()
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = self;
        f.write_str("")
    }
}

#[cfg(test)]
mod tests;
