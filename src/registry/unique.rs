//! A section's key column holds each value once (registry V51; actlith
//! `rows::unique`).

use std::collections::BTreeMap;

use super::document::{Cell, Row};
use super::error::{Pos, RegistryError};
use super::schema::SectionSpec;

/// A repeated key: its line, its cell, and the line it was first on.
type Repeat<'a> = (usize, &'a Cell, usize);

/// The rows of `spec`'s section repeat no key; a section without a key
/// column passes.
///
/// # Errors
///
/// The second row with a key, naming the first one's line.
pub(crate) fn unique(
    file: &str,
    spec: &SectionSpec<'_>,
    rows: &[Row],
) -> Result<(), RegistryError> {
    let Some(key) = spec.key else {
        return Ok(());
    };
    let name = spec.columns.get(key).copied().unwrap_or_default();
    let cells = rows
        .iter()
        .filter_map(|r| Some((r.line, r.cells.get(key)?)));
    once(cells).map_or(Ok(()), |repeat| Err(twice(file, name, repeat)))
}

/// The first `(line, cell)` whose text was seen on an earlier line.
fn once<'a>(
    cells: impl Iterator<Item = (usize, &'a Cell)>,
) -> Option<Repeat<'a>> {
    let mut seen: BTreeMap<&str, usize> = BTreeMap::new();
    for (line, cell) in cells {
        if let Some(first) = seen.insert(&cell.text, line) {
            return Some((line, cell, first));
        }
    }
    None
}

/// The repeated key of column `name`, at its cell.
fn twice(file: &str, name: &str, repeat: Repeat<'_>) -> RegistryError {
    let (line, cell, first) = repeat;
    let at = Pos {
        file,
        line,
        column: cell.column,
    };
    let what =
        format!("{name} `{}` given twice, first on line {first}", cell.text);
    at.error(what, "keep one row per key")
}

#[cfg(test)]
mod tests;
