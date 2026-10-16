//! The `## workflows` section and its rows as written (kind C): columns
//! `id|kind|systems|cache|args`, cells typed by the registry readers,
//! nothing checked against a kind yet.

use crate::registry::{
    Arg, RegistryError, Row as Cells, SectionSpec, args, list, optional,
    required,
};

/// The section's name: `## workflows` on every forge.
pub const SECTION: &str = "workflows";

/// The section's header row, in order.
pub const COLUMNS: [&str; 5] = ["id", "kind", "systems", "cache", "args"];

/// The section as a forge crate puts it in its registry schema: keyed
/// on `id`, so the registry refuses a repeated id; optional, so a file
/// without it has no rows.
pub const WORKFLOWS: SectionSpec<'static> = SectionSpec {
    name: SECTION,
    columns: &COLUMNS,
    required: false,
    key: Some(0),
};

/// A `## workflows` row as written: `id|kind|systems|cache|args`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RawRow {
    /// The 1-based line the row is on, for errors downstream.
    pub line: usize,
    /// The pipeline's name and file stem.
    pub id: String,
    /// The kind name, checked by [`resolve`](super::resolve()).
    pub kind: String,
    /// Nix system names; empty = the kind's default (`-`).
    pub systems: Vec<String>,
    /// Cachix cache name; `None` = no cache steps (`-`).
    pub cache: Option<String>,
    /// `key=value` pairs, in cell order.
    pub args: Vec<Arg>,
}

impl RawRow {
    /// The cells of a `## workflows` row of `file`, typed.
    ///
    /// # Errors
    ///
    /// An empty `id` or `kind`, an id that is not a file name, a
    /// malformed `systems` list or `args` cell.
    pub fn from_cells(file: &str, row: &Cells) -> Result<Self, RegistryError> {
        let id = required(file, row, 0, "id")?;
        Ok(Self {
            line: row.line,
            id: file_name(file, row, id)?,
            kind: required(file, row, 1, "kind")?,
            systems: list(file, row, 2, "systems")?,
            cache: optional(row, 3),
            args: args(file, row, 4, "args")?,
        })
    }
}

/// An id is a plain file stem: `[A-Za-z0-9._-]`, no leading dot.
fn file_name(
    file: &str,
    row: &Cells,
    id: String,
) -> Result<String, RegistryError> {
    let plain = |c: char| c.is_ascii_alphanumeric() || "._-".contains(c);
    if id.chars().all(plain) && !id.starts_with('.') {
        return Ok(id);
    }
    Err(RegistryError {
        file: file.to_owned(),
        line: row.line,
        column: row.cells.first().map_or(1, |c| c.column),
        what: format!("id `{id}` is not a file name"),
        fix: "use letters, digits, `.`, `_`, `-`; no leading `.`".to_owned(),
    })
}

#[cfg(test)]
mod tests;
