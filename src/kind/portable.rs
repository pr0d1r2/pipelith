//! What a forge adds beyond the catalogue, and the portability check
//! (V33, `.:V4`, `.:V5`): a row using a forge-only kind or arg key is
//! valid on that forge only, and `portable` reports each such use.

use std::fmt;

use super::catalogue::{ArgSpec, CATALOGUE, KindSpec, lookup};
use super::error::KindError;
use super::raw::RawRow;

/// The kinds and arg keys a forge crate declares on top of the
/// catalogue. A forge-only key is accepted on every kind of that forge.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Extras<'a> {
    /// Forge-only kinds.
    pub kinds: &'a [KindSpec],
    /// Forge-only arg keys.
    pub keys: &'a [ArgSpec],
}

/// One reason a row renders on its own forge only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Finding {
    /// The row's kind is forge-only.
    Kind {
        /// The 1-based line of the row.
        line: usize,
        /// The kind name.
        kind: String,
    },
    /// The row gives a forge-only arg key.
    Key {
        /// The 1-based line of the row.
        line: usize,
        /// The key.
        key: String,
    },
}

impl<'a> Extras<'a> {
    /// The forge-only kind `name`, if the forge declares one.
    #[must_use]
    pub fn kind(&self, name: &str) -> Option<&'a KindSpec> {
        self.kinds.iter().find(|k| k.name == name)
    }

    /// The kinds a row may name: the catalogue's, then the forge's.
    fn names(&self) -> String {
        let all = CATALOGUE.iter().chain(self.kinds);
        let quoted: Vec<String> =
            all.map(|k| format!("`{}`", k.name)).collect();
        quoted.join(", ")
    }
}

/// The kind `row` names: a catalogue kind, else a forge-only one.
///
/// # Errors
///
/// A kind neither knows, listing the kinds a row may name.
pub fn resolve<'a>(
    row: &RawRow,
    extras: &Extras<'a>,
) -> Result<&'a KindSpec, KindError> {
    let kind = lookup(&row.kind).or_else(|| extras.kind(&row.kind));
    kind.ok_or_else(|| {
        KindError::new(
            row.line,
            format!("unknown kind `{}`", row.kind.escape_debug()),
            format!("use one of {}", extras.names()),
        )
    })
}

/// One finding per forge-only kind and per forge-only arg key of `row`;
/// empty when the row renders on every forge (V33). A key the row's
/// kind takes itself is never forge-only; an unknown kind or key is
/// not a finding but a [`Row::check`](super::Row::check) error.
#[must_use]
pub fn portable(row: &RawRow, extras: &Extras<'_>) -> Vec<Finding> {
    let neutral = lookup(&row.kind);
    let forge = neutral.is_none().then(|| extras.kind(&row.kind)).flatten();
    let kind = forge.map(|_| Finding::Kind {
        line: row.line,
        kind: row.kind.clone(),
    });
    let schema = neutral.or(forge).map_or(&[][..], |k| k.schema);
    kind.into_iter()
        .chain(forge_keys(row, schema, extras))
        .collect()
}

/// The row's arg keys the forge declares and the row's kind lacks.
fn forge_keys(
    row: &RawRow,
    schema: &[ArgSpec],
    extras: &Extras<'_>,
) -> Vec<Finding> {
    let only = |key: &str| {
        !schema.iter().any(|s| s.key == key)
            && extras.keys.iter().any(|s| s.key == key)
    };
    let keys = row.args.iter().filter(|a| only(&a.key));
    keys.map(|a| Finding::Key {
        line: row.line,
        key: a.key.clone(),
    })
    .collect()
}

impl fmt::Display for Finding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Kind { line, kind } => write!(
                f,
                "line {line}: kind `{kind}` is forge-only; the other forge \
                 refuses this row"
            ),
            Self::Key { line, key } => write!(
                f,
                "line {line}: arg `{key}` is forge-only; the other forge \
                 refuses this row"
            ),
        }
    }
}

#[cfg(test)]
mod tests;
