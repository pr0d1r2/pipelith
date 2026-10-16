//! Proposed rows as registry text: the `format` line and the
//! `## workflows` table (V37, `.:I` file).

use crate::kind::{RawRow, SECTION, WORKFLOWS};
use crate::registry::SUPPORTED_FORMAT;

/// The 1-based line the first row lands on under [`header`].
pub const FIRST_ROW: usize = 5;

/// Everything above the first row: `format <SUPPORTED_FORMAT>`, a blank
/// line, `## workflows` and the column header.
#[must_use]
pub fn header() -> String {
    let columns = WORKFLOWS.header();
    format!("format {SUPPORTED_FORMAT}\n\n## {SECTION}\n{columns}\n")
}

/// A whole config file: [`header`], then one line per row.
#[must_use]
pub fn text(rows: &[RawRow]) -> String {
    rows.iter().fold(header(), |mut out, row| {
        out.push_str(&row_line(row));
        out.push('\n');
        out
    })
}

/// One `id|kind|systems|cache|args` row; `-` for an empty cell, `\|` for
/// a pipe.
#[must_use]
pub fn row_line(row: &RawRow) -> String {
    let systems = list(row.systems.iter().map(String::as_str));
    let cache = list(row.cache.as_deref());
    let args = row.args.iter().map(|a| format!("{}={}", a.key, a.value));
    let cells = [cell(&row.id), cell(&row.kind), systems, cache, list(args)];
    cells.join("|")
}

/// Items joined by `, `, each escaped; `-` when there are none.
fn list<T: AsRef<str>>(items: impl IntoIterator<Item = T>) -> String {
    let cells: Vec<String> =
        items.into_iter().map(|item| cell(item.as_ref())).collect();
    if cells.is_empty() {
        return "-".to_owned();
    }
    cells.join(", ")
}

/// A literal pipe written as `\|`.
fn cell(text: &str) -> String {
    text.replace('|', "\\|")
}

#[cfg(test)]
mod tests;
