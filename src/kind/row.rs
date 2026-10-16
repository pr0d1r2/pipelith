//! A `## workflows` row checked for its kind: systems resolved to ones
//! some forge hosts (V52), the cache to a known key (V53), the args to
//! the schema (V31). A forge builds a pipeline only from a checked row.

use super::args::{Args, parse_args};
use super::cache::Cache;
use super::catalogue::KindSpec;
use super::error::KindError;
use super::portable::Extras;
use super::raw::RawRow;
use crate::system::{System, ci_system, parse_system};

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
        let checked = Self {
            line: row.line,
            id: row.id.clone(),
            systems: systems(kind.default_systems, row)?,
            cache: cache(row)?,
            args: parse_args(kind, row, extras)?,
        };
        checked.args.timeout()?;
        Ok(checked)
    }
}

/// The row's systems in its order, or the kind's default for `-`.
fn systems(
    defaults: &[System],
    row: &RawRow,
) -> Result<Vec<System>, KindError> {
    if row.systems.is_empty() {
        return Ok(defaults.to_vec());
    }
    let mut out = Vec::new();
    for name in &row.systems {
        let system = hosted(row.line, name)?;
        if out.contains(&system) {
            return Err(twice(row.line, name));
        }
        out.push(system);
    }
    Ok(out)
}

fn hosted(line: usize, name: &str) -> Result<System, KindError> {
    let fix = "write systems with a hosted runner, or `-`";
    let refuse = |e: crate::system::SystemError| {
        KindError::new(line, e.to_string(), fix)
    };
    parse_system(name).and_then(ci_system).map_err(refuse)
}

fn twice(line: usize, name: &str) -> KindError {
    KindError::new(
        line,
        format!("system `{name}` is listed twice"),
        format!("keep one `{name}`"),
    )
}

/// The row's cache, `None` for `-` (V53).
fn cache(row: &RawRow) -> Result<Option<Cache>, KindError> {
    let name = row.cache.as_deref();
    name.map(|c| Cache::parse(row.line, c)).transpose()
}

#[cfg(test)]
mod tests;
