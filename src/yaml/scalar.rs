//! Scalar styles: the one place that decides how a string is written
//! (V24). Plain only for a closed set of characters; otherwise single
//! quoted, double quoted with escapes, a `|` literal block for several
//! lines, or a `>-` folded block for a long line.

use super::text::{Dialect, Part, Text};

/// The longest line the writer aims for (yamllint, `src/yaml` C).
pub const WIDTH: usize = 120;

/// Words a YAML 1.1 reader takes for a boolean or null.
const RESERVED: [&str; 10] = [
    "true", "false", "yes", "no", "on", "off", "y", "n", "null", "~",
];

/// Characters that give a plain scalar another meaning at its start.
const INDICATORS: &str = "-?:,[]{}#&*!|>'\"%@`";

/// `text` on one line: plain when that reads back as the same string,
/// otherwise quoted. `flow` is true inside `[a, b]`, where an expression
/// is always quoted.
pub(crate) fn inline(dialect: &dyn Dialect, text: &Text, flow: bool) -> String {
    let raw = text.render(dialect);
    if plain(dialect, text, &raw, flow) {
        raw
    } else {
        quote(&raw)
    }
}

/// A key or other literal string on one line, plain or quoted (V24).
#[must_use]
pub fn key(raw: &str) -> String {
    if lit_safe(raw) && starts_plain(raw) && !loose(raw) {
        raw.to_owned()
    } else {
        quote(raw)
    }
}

fn plain(dialect: &dyn Dialect, text: &Text, raw: &str, flow: bool) -> bool {
    let starts = text.parts().first().is_some_and(|p| match p {
        Part::Expr(_) => expr_starts(&p.render(dialect)),
        Part::Lit(_) => starts_plain(&p.render(dialect)),
    });
    let parts = text.parts().iter().all(|p| match p {
        Part::Lit(_) => lit_safe(&p.render(dialect)),
        Part::Expr(_) => !flow,
    });
    starts && parts && !loose(raw) && !breaks(raw)
}

/// Text a plain scalar would read as a mapping or a comment (V56): what
/// an expression renders to is judged in place, not its inner text.
fn breaks(raw: &str) -> bool {
    raw.contains(": ") || raw.contains(" #") || raw.ends_with(':')
}

fn expr_starts(text: &str) -> bool {
    text.chars().next().is_some_and(|c| !INDICATORS.contains(c))
}

fn lit_safe(s: &str) -> bool {
    s.chars()
        .all(|c| c.is_ascii_alphanumeric() || " _./@+=-".contains(c))
}

fn starts_plain(s: &str) -> bool {
    s.chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '/' || c == '_')
}

/// A trailing space (plain would drop it) or a reserved word.
fn loose(raw: &str) -> bool {
    raw.ends_with(' ') || RESERVED.iter().any(|r| raw.eq_ignore_ascii_case(r))
}

/// `raw` quoted: single quotes, or double quotes with escapes when it
/// holds a control character (a line break among them).
#[must_use]
pub fn quote(raw: &str) -> String {
    if raw.chars().any(char::is_control) {
        format!("\"{}\"", raw.chars().map(escape).collect::<String>())
    } else {
        format!("'{}'", raw.replace('\'', "''"))
    }
}

fn escape(c: char) -> String {
    match c {
        '\\' => "\\\\".to_owned(),
        '"' => "\\\"".to_owned(),
        '\n' => "\\n".to_owned(),
        '\t' => "\\t".to_owned(),
        '\r' => "\\r".to_owned(),
        c if c.is_control() => format!("\\u{:04X}", u32::from(c)),
        c => c.to_string(),
    }
}

/// `raw` as a `|` literal block: the header (with its chomping indicator)
/// and the lines, unindented. `None` when `raw` is one line, holds a
/// control character other than a line break, has no line with text, or
/// its first text line starts with a space (the block's indentation would
/// then be read from it).
pub(crate) fn literal(raw: &str) -> Option<(&'static str, Vec<&str>)> {
    let body = raw.strip_suffix('\n').unwrap_or(raw);
    let first = body.split('\n').find(|l| !l.is_empty())?;
    let blockable = raw.contains('\n')
        && !first.starts_with(' ')
        && !raw.chars().any(|c| c.is_control() && c != '\n');
    blockable.then(|| (chomp(raw), body.split('\n').collect()))
}

fn chomp(raw: &str) -> &'static str {
    if raw.ends_with("\n\n") {
        "|+"
    } else if raw.ends_with('\n') {
        "|"
    } else {
        "|-"
    }
}

/// `raw` as the lines of a `>-` folded block indented by `indent`: a break
/// after every `&&` and `||`, and before a word that would pass
/// [`WIDTH`]. `None` when folding would change the string: a line break,
/// a control character, two spaces in a row, a space at either end, or no
/// space at all.
pub(crate) fn fold(raw: &str, indent: usize) -> Option<Vec<String>> {
    let foldable = raw.contains(' ')
        && !raw.contains("  ")
        && raw.trim() == raw
        && !raw.chars().any(char::is_control);
    foldable.then(|| wrap(raw, indent))
}

fn wrap(raw: &str, indent: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in raw.split(' ') {
        if !line.is_empty() && overflows(indent, &line, word) {
            lines.push(std::mem::take(&mut line));
        }
        append(&mut line, word);
        if word == "&&" || word == "||" {
            lines.push(std::mem::take(&mut line));
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    lines
}

fn overflows(indent: usize, line: &str, word: &str) -> bool {
    indent
        .saturating_add(line.len())
        .saturating_add(1)
        .saturating_add(word.len())
        > WIDTH
}

fn append(line: &mut String, word: &str) {
    if !line.is_empty() {
        line.push(' ');
    }
    line.push_str(word);
}

#[cfg(test)]
mod tests;
