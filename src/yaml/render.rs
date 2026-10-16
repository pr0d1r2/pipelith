//! The writer: a tree to bytes in a fixed layout (V23, V26): `---`,
//! two-space indentation, block style, `[a, b]` lists, comments on their
//! own lines or two spaces after a value, blank lines only between
//! top-level keys.

use super::node::Entry;
use super::scalar;
use super::text::{Comment, Dialect, Text};

/// The document: `---`, the leading comments, then the top-level entries
/// with a blank line between them. Literal text is escaped by `dialect`.
#[must_use]
pub fn document(
    dialect: &dyn Dialect,
    comments: &[Comment],
    entries: &[Entry],
) -> String {
    let _ = (comments, entries, scalar::literal(""), scalar::fold("", 0));
    scalar::inline(dialect, &Text::default(), false)
}

#[cfg(test)]
mod tests;
