//! One table row into cells, and the readers that type a cell: `|`
//! splits, `\|` is a literal pipe, `-` is an empty cell, `,` splits a
//! list (registry C, V18, V19).

use super::document::{Cell, Row};
use super::error::{Pos, RegistryError};
use super::schema::SectionSpec;

/// The fix hint for a malformed row.
const ROW_FIX: &str = "one cell per column, `-` if empty, `\\|` for a pipe";

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
pub(crate) fn split(row: &str) -> Vec<(usize, String)> {
    let mut cells = vec![(0, String::new())];
    let mut chars = row.chars().enumerate().peekable();
    while let Some((i, c)) = chars.next() {
        if c == '|' {
            cells.push((i.saturating_add(1), String::new()));
        } else if c == '\\' && chars.next_if(|&(_, n)| n == '|').is_some() {
            push(&mut cells, '|');
        } else {
            push(&mut cells, c);
        }
    }
    cells.into_iter().map(|(at, raw)| tidy(at, &raw)).collect()
}

/// Append `c` to the last cell.
fn push(cells: &mut [(usize, String)], c: char) {
    if let Some((_, cell)) = cells.last_mut() {
        cell.push(c);
    }
}

/// A raw cell trimmed, its offset moved past leading blanks, `-` emptied.
fn tidy(start: usize, raw: &str) -> (usize, String) {
    let lead = raw.chars().take_while(|c| c.is_whitespace()).count();
    let text = match raw.trim() {
        "-" => String::new(),
        t => t.to_owned(),
    };
    (start.saturating_add(lead), text)
}

/// The header row is the section's own, cell by cell.
///
/// # Errors
///
/// Any other header.
pub(crate) fn header(
    at: Pos<'_>,
    spec: &SectionSpec<'_>,
    row: &str,
) -> Result<(), RegistryError> {
    let cells: Vec<String> = split(row).into_iter().map(|(_, t)| t).collect();
    if cells == spec.columns {
        return Ok(());
    }
    let what = format!("`## {}` header is `{row}`", spec.name);
    let fix = format!("start the section with `{}`", spec.header());
    Err(at.error(what, fix))
}

/// A data row with exactly one cell per column; `at` is its first
/// character.
///
/// # Errors
///
/// The row has more or fewer cells than the section's header.
pub(crate) fn row(
    at: Pos<'_>,
    spec: &SectionSpec<'_>,
    text: &str,
) -> Result<Row, RegistryError> {
    let cells: Vec<Cell> = split(text)
        .into_iter()
        .map(|(o, text)| Cell {
            column: at.column.saturating_add(o),
            text,
        })
        .collect();
    if cells.len() == spec.columns.len() {
        return Ok(Row {
            line: at.line,
            cells,
        });
    }
    Err(count(at, spec, cells.len()))
}

/// A row with `n` cells in a section whose rows have another count.
fn count(at: Pos<'_>, spec: &SectionSpec<'_>, n: usize) -> RegistryError {
    let s = if n == 1 { "" } else { "s" };
    let what = format!(
        "{n} cell{s} where `## {}` rows have {}",
        spec.name,
        spec.columns.len()
    );
    at.error(what, format!("{}: {ROW_FIX}", spec.header()))
}

/// Where a reader's error points: the cell, or the row's start.
fn spot<'a>(file: &'a str, row: &Row, cell: Option<&Cell>) -> Pos<'a> {
    let column = cell.map_or(1, |c| c.column);
    Pos {
        file,
        line: row.line,
        column,
    }
}

/// The text of cell `index` (column `name`), which may not be empty.
///
/// # Errors
///
/// The cell is empty (`-`), naming the column.
pub fn required(
    file: &str,
    row: &Row,
    index: usize,
    name: &str,
) -> Result<String, RegistryError> {
    match row.cells.get(index) {
        Some(c) if !c.text.is_empty() => Ok(c.text.clone()),
        cell => {
            let fix = format!("fill `{name}`; `-` is only for optional cells");
            Err(spot(file, row, cell).error(format!("`{name}` is empty"), fix))
        }
    }
}

/// The text of cell `index`, or `None` when it is empty (`-`).
#[must_use]
pub fn optional(row: &Row, index: usize) -> Option<String> {
    let text = row.cells.get(index).map(|c| c.text.clone());
    text.filter(|t| !t.is_empty())
}

/// Cell `index` (column `name`) as a `,`-separated list, items trimmed;
/// an empty cell (`-`) is an empty list.
///
/// # Errors
///
/// An empty item (`a,,b`).
pub fn list(
    file: &str,
    row: &Row,
    index: usize,
    name: &str,
) -> Result<Vec<String>, RegistryError> {
    let cell = row.cells.get(index);
    let text = cell.map_or("", |c| c.text.as_str());
    if text.is_empty() {
        return Ok(Vec::new());
    }
    let items: Vec<String> =
        text.split(',').map(|i| i.trim().to_owned()).collect();
    if items.iter().any(String::is_empty) {
        let what = format!("`{name}` has an empty item");
        let fix = "separate items with one `,`; `-` for none";
        return Err(spot(file, row, cell).error(what, fix));
    }
    Ok(items)
}

/// Cell `index` (column `name`) as `,`-separated `key=value` pairs, in
/// order, each key once.
///
/// # Errors
///
/// An item that is not `key=value`, or a key given twice.
pub fn args(
    file: &str,
    row: &Row,
    index: usize,
    name: &str,
) -> Result<Vec<Arg>, RegistryError> {
    let at = spot(file, row, row.cells.get(index));
    let mut out: Vec<Arg> = Vec::new();
    for item in list(file, row, index, name)? {
        let arg = pair(at, &item)?;
        if out.iter().any(|a| a.key == arg.key) {
            let what = format!("arg `{}` given twice", arg.key);
            return Err(at.error(what, "give each key once"));
        }
        out.push(arg);
    }
    Ok(out)
}

/// One `key=value` item, both sides non-empty.
fn pair(at: Pos<'_>, item: &str) -> Result<Arg, RegistryError> {
    let kv = item.split_once('=').map(|(k, v)| (k.trim(), v.trim()));
    match kv {
        Some((key, value)) if !key.is_empty() && !value.is_empty() => Ok(Arg {
            key: key.to_owned(),
            value: value.to_owned(),
        }),
        _ => Err(at.error(
            format!("arg `{item}` is not `key=value`"),
            "write `key=value`; leave the key out instead of an empty value",
        )),
    }
}

#[cfg(test)]
mod tests;
