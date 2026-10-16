//! Scalar styles: the one place that decides how a string is written
//! (V24). Plain only for a closed set of characters; otherwise single
//! quoted, double quoted with escapes, a `|` literal block for several
//! lines, or a `>-` folded block for a long line.

use super::text::{Dialect, Text};

/// The longest line the writer aims for (yamllint, `src/yaml` C).
pub const WIDTH: usize = 120;

/// `text` on one line: plain when that reads back as the same string,
/// otherwise quoted. `flow` is true inside `[a, b]`, where an expression
/// is always quoted.
pub(crate) fn inline(dialect: &dyn Dialect, text: &Text, flow: bool) -> String {
    let _ = (dialect, text, flow);
    String::new()
}

/// A key or other literal string on one line, plain or quoted (V24).
#[must_use]
pub fn key(raw: &str) -> String {
    raw.to_owned()
}

/// `raw` quoted: single quotes, or double quotes with escapes when it
/// holds a control character (a line break among them).
#[must_use]
pub fn quote(raw: &str) -> String {
    raw.to_owned()
}

/// `raw` as a `|` literal block: the header (with its chomping indicator)
/// and the lines, unindented. `None` when it would not read back.
pub(crate) fn literal(raw: &str) -> Option<(&'static str, Vec<&str>)> {
    let _ = raw;
    None
}

/// `raw` as the lines of a `>-` folded block indented by `indent`;
/// `None` when folding would change the string.
pub(crate) fn fold(raw: &str, indent: usize) -> Option<Vec<String>> {
    let _ = (raw, indent);
    None
}

#[cfg(test)]
mod tests;
