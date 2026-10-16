//! A row's `args` checked against its kind's schema plus the forge's
//! extra keys (V31): an unknown, repeated or missing key is an error
//! naming the row's line and the key, never ignored or silently
//! defaulted.

use std::collections::BTreeMap;

use super::catalogue::{DEFAULT_TIMEOUT, KindSpec, MAX_TIMEOUT};
use super::error::KindError;
use super::portable::Extras;
use super::raw::RawRow;

/// A row's args after the schema check: every given key is known, every
/// required key is there, every [`ArgDefault::Value`](super::ArgDefault::Value) is filled in.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Args {
    line: usize,
    values: BTreeMap<String, String>,
}

impl Args {
    /// The line of the row the args came from.
    #[must_use]
    pub fn line(&self) -> usize {
        let _ = self;
        0
    }

    /// The value of `key`, given or defaulted; `None` when absent, and
    /// for an [`ArgDefault::Forge`](super::ArgDefault::Forge) key the row left out.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&str> {
        let _ = (self, key);
        None
    }

    /// The value of `key` as a whole number, `None` when absent.
    ///
    /// # Errors
    ///
    /// The value is not a whole number.
    pub fn number(&self, key: &str) -> Result<Option<u32>, KindError> {
        let _ = (self, key);
        Ok(None)
    }

    /// The `timeout` in minutes, [`DEFAULT_TIMEOUT`] when absent.
    ///
    /// # Errors
    ///
    /// Not a whole number, or outside 1 to [`MAX_TIMEOUT`].
    pub fn timeout(&self) -> Result<u32, KindError> {
        let _ = (self, DEFAULT_TIMEOUT, MAX_TIMEOUT);
        Ok(0)
    }
}

/// The args of `row` checked against `kind`'s schema and the forge's
/// extra keys.
///
/// # Errors
///
/// An unknown or repeated key, or a missing required one (V31).
pub fn parse_args(
    kind: &KindSpec,
    row: &RawRow,
    extras: &Extras<'_>,
) -> Result<Args, KindError> {
    let _ = (kind, row, extras);
    Ok(Args {
        line: 0,
        values: BTreeMap::new(),
    })
}

#[cfg(test)]
mod tests;
