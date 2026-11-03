//! The `## caches` section (V82, V83): `name|kind|url|key|push`, one row
//! per cache a `## workflows` row may name, every cell checked here so a
//! forge never refuses the row later.

use super::entry::{BinaryCache, builtin, builtin_names};
use crate::registry::{Document, RegistryError, SectionSpec};

#[cfg(test)]
mod tests;

/// The section's name: `## caches` on every forge.
pub const SECTION: &str = "caches";

/// The section's header row, in order.
pub const COLUMNS: [&str; 5] = ["name", "kind", "url", "key", "push"];

/// The section as a forge crate puts it in its registry schema: keyed
/// on `name`, so the registry refuses a name given twice; optional, so a
/// file without it has only the built-in caches.
pub const CACHES: SectionSpec<'static> = SectionSpec {
    name: SECTION,
    columns: &COLUMNS,
    required: false,
    key: Some(0),
};

/// The `## caches` rows of a registry file, checked.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Caches {
    /// One cache per row, in file order. RED: never filled.
    #[allow(dead_code)]
    rows: Vec<BinaryCache>,
}

impl Caches {
    /// The checked `## caches` rows of `doc`, parsed from `file`; none
    /// when the file has no such section.
    ///
    /// # Errors
    ///
    /// A bad name, an unknown kind, a bad secret name, a literal url
    /// that is not https, a malformed literal key, a literal push, or a
    /// cell the kind does not allow, located (V83).
    pub fn from_document(
        _file: &str,
        _doc: &Document,
    ) -> Result<Self, RegistryError> {
        Ok(Self::default())
    }

    /// The cache a `cache` cell names: the row of that name, otherwise
    /// the built-in cachix cache of that name (V82).
    #[must_use]
    pub fn find(&self, name: &str) -> Option<BinaryCache> {
        builtin(name)
    }

    /// Whether the section has no rows: only the built-in caches.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        true
    }

    /// Whether a row of the section is named `name`, shadowing any
    /// built-in cache of that name.
    #[must_use]
    pub fn has_row(&self, _name: &str) -> bool {
        false
    }

    /// Every name a `cache` cell may write: the rows', then each
    /// built-in name no row shadows.
    #[must_use]
    pub fn names(&self) -> Vec<&str> {
        builtin_names()
    }
}
