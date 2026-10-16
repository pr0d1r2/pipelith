//! Proposed rows as registry text: the `format` line and the
//! `## workflows` table (V37, `.:I` file).

use crate::kind::RawRow;

/// The 1-based line the first row lands on under [`header`].
pub const FIRST_ROW: usize = 0;

/// Everything above the first row: `format <SUPPORTED_FORMAT>`, a blank
/// line, `## workflows` and the column header.
#[must_use]
pub fn header() -> String {
    String::new()
}

/// A whole config file: [`header`], then one line per row.
#[must_use]
pub fn text(rows: &[RawRow]) -> String {
    let _ = rows;
    String::new()
}

/// One `id|kind|systems|cache|args` row; `-` for an empty cell, `\|` for
/// a pipe.
#[must_use]
pub fn row_line(row: &RawRow) -> String {
    let _ = row;
    String::new()
}

#[cfg(test)]
mod tests;
