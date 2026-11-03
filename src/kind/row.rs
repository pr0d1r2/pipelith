//! A `## workflows` row checked for its kind: systems resolved to known
//! nix systems (V52), the cache to a `## caches` row or a built-in one
//! (V53), the args to
//! the schema (V31). A forge builds a pipeline only from a checked row.

use super::args::{Args, parse_args};
use super::cache::{Cache, resolve};
use super::catalogue::KindSpec;
use super::error::KindError;
use super::portable::Extras;
use super::raw::RawRow;
use super::rules;
use crate::cache::{BinaryCache, Caches};
use crate::system::{System, parse_system};

/// A row a forge can build from.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    /// The 1-based line of the row.
    pub line: usize,
    /// The pipeline's name and file stem.
    pub id: String,
    /// The systems, in row order; the kind's default for `-`.
    pub systems: Vec<System>,
    /// The cache to trust, if the row names one.
    pub cache: Option<Cache>,
    /// The checked args.
    pub args: Args,
}

/// A row checked with the file's `## caches` rows: the row as
/// [`Row::check`] gives it, and the cache it trusts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checked {
    /// The row. Its `cache` is the 0.2.0 [`Cache`]: set only when the
    /// row names a built-in cachix cache no `## caches` row shadows, so
    /// a caller reading it sees exactly what 0.2.0 gave.
    pub row: Row,
    /// The cache the row trusts, whichever table named it; `None` for
    /// `-`. A forge on `## caches` renders from this one.
    pub cache: Option<BinaryCache>,
}

impl Row {
    /// `row` checked against `kind` and the forge's `extras`, with the
    /// built-in caches only, as in 0.2.0.
    ///
    /// # Errors
    ///
    /// An unknown or repeated system; an unknown cache;
    /// a bad arg or `timeout` (V31, V52, V53).
    pub fn check(
        kind: &KindSpec,
        row: &RawRow,
        extras: &Extras<'_>,
    ) -> Result<Self, KindError> {
        Self::check_with(kind, row, extras, &Caches::default()).map(|c| c.row)
    }

    /// `row` checked against `kind`, the forge's `extras` and the file's
    /// `## caches` rows (`src/cache:V82`).
    ///
    /// # Errors
    ///
    /// As [`Row::check`], and a cache in neither `caches` nor the
    /// built-in table, or a `build` row whose cache is read-only.
    pub fn check_with(
        kind: &KindSpec,
        row: &RawRow,
        extras: &Extras<'_>,
        caches: &Caches,
    ) -> Result<Checked, KindError> {
        let systems = systems(kind.default_systems, row)?;
        let (old, cache) = caches_of(row, caches)?;
        let checked = Self {
            line: row.line,
            id: row.id.clone(),
            systems,
            cache: old,
            args: parse_args(kind, row, extras)?,
        };
        checked.args.timeout()?;
        rules::check(kind, &checked, cache.as_ref())?;
        Ok(Checked {
            row: checked,
            cache,
        })
    }
}

/// The row's caches as `(row.cache, checked.cache)`; none for `-`.
fn caches_of(
    row: &RawRow,
    caches: &Caches,
) -> Result<(Option<Cache>, Option<BinaryCache>), KindError> {
    let name = row.cache.as_deref();
    name.map_or(Ok((None, None)), |n| resolve(row.line, n, caches))
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
        let system = known(row.line, name)?;
        if out.contains(&system) {
            return Err(twice(row.line, name));
        }
        out.push(system);
    }
    Ok(out)
}

fn known(line: usize, name: &str) -> Result<System, KindError> {
    let fix = "write nix systems from the list, or `-`";
    let refuse = |e: crate::system::SystemError| {
        KindError::new(line, e.to_string(), fix)
    };
    parse_system(name).map_err(refuse)
}

fn twice(line: usize, name: &str) -> KindError {
    KindError::new(
        line,
        format!("system `{name}` is listed twice"),
        format!("keep one `{name}`"),
    )
}

#[cfg(test)]
mod tests;
