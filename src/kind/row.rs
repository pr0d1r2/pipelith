//! A `## workflows` row checked for its kind: systems resolved to ones
//! some forge hosts (V52), the cache to a known key (V53), the args to
//! the schema (V31). A forge builds a pipeline only from a checked row.

use super::args::Args;
use super::cache::Cache;
use super::catalogue::KindSpec;
use super::error::KindError;
use super::portable::Extras;
use super::raw::RawRow;
use crate::system::System;

/// A row a forge can build from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    /// The 1-based line of the row.
    pub line: usize,
    /// The pipeline's name and file stem.
    pub id: String,
    /// The systems, each with a hosted runner; the kind's default for `-`.
    pub systems: Vec<System>,
    /// The cache to trust, if the row names one.
    pub cache: Option<Cache>,
    /// The checked args.
    pub args: Args,
}

impl Row {
    /// `row` checked against `kind` and the forge's `extras`.
    ///
    /// # Errors
    ///
    /// An unknown, repeated or unhosted system; an unknown cache;
    /// a bad arg or `timeout` (V31, V52, V53).
    pub fn check(
        kind: &KindSpec,
        row: &RawRow,
        extras: &Extras<'_>,
    ) -> Result<Self, KindError> {
        let _ = (kind, row, extras);
        Err(KindError::default())
    }
}

#[cfg(test)]
mod tests;
