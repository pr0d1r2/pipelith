//! One table row into cells, and the readers that type a cell: `|`
//! splits, `\|` is a literal pipe, `-` is an empty cell, `,` splits a
//! list (registry C, V18, V19).

use super::document::Row;
use super::error::{Pos, RegistryError};
use super::schema::SectionSpec;

/// One `key=value` item of an args cell.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Arg {
    /// The text before the first `=`, trimmed.
    pub key: String,
    /// The text after the first `=`, trimmed.
    pub value: String,
}

/// The cells of a row as `(offset, text)`: the 0-based character offset
/// of the cell's text in `row`, and the text trimmed, `\|` unescaped and
/// `-` made empty.
#[must_use]
pub(crate) fn split(_row: &str) -> Vec<(usize, String)> {
    Vec::new()
}

/// The header row is the section's own, cell by cell.
///
/// # Errors
///
/// Any other header.
pub(crate) fn header(
    at: Pos<'_>,
    _spec: &SectionSpec<'_>,
    _row: &str,
) -> Result<(), RegistryError> {
    Err(at.error("", ""))
}

/// A data row with exactly one cell per column; `at` is its first
/// character.
///
/// # Errors
///
/// The row has more or fewer cells than the section's header.
pub(crate) fn row(
    at: Pos<'_>,
    _spec: &SectionSpec<'_>,
    _text: &str,
) -> Result<Row, RegistryError> {
    Err(at.error("", ""))
}

/// The text of cell `index` (column `name`), which may not be empty.
///
/// # Errors
///
/// The cell is empty (`-`), naming the column.
pub fn required(
    _file: &str,
    _row: &Row,
    _index: usize,
    _name: &str,
) -> Result<String, RegistryError> {
    Ok(String::new())
}

/// The text of cell `index`, or `None` when it is empty (`-`).
#[must_use]
pub fn optional(_row: &Row, _index: usize) -> Option<String> {
    None
}

/// Cell `index` (column `name`) as a `,`-separated list, items trimmed;
/// an empty cell (`-`) is an empty list.
///
/// # Errors
///
/// An empty item (`a,,b`).
pub fn list(
    _file: &str,
    _row: &Row,
    _index: usize,
    _name: &str,
) -> Result<Vec<String>, RegistryError> {
    Ok(Vec::new())
}

/// Cell `index` (column `name`) as `,`-separated `key=value` pairs, in
/// order, each key once.
///
/// # Errors
///
/// An item that is not `key=value`, or a key given twice.
pub fn args(
    _file: &str,
    _row: &Row,
    _index: usize,
    _name: &str,
) -> Result<Vec<Arg>, RegistryError> {
    Ok(Vec::new())
}

#[cfg(test)]
mod tests;
