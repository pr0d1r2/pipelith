//! The row's cachix cache as 0.2.0 has it: its name and public key, from
//! the built-in table only (V53). cachix serves every forge, so this is
//! neutral. A `## caches` row is a `cache::BinaryCache`, never this type
//! (`src/cache:V82`): its key may be a secret, which [`Cache::key`] and
//! [`Cache::nix_config`] could not answer.

use super::error::KindError;
use crate::cache::{BinaryCache, Caches, builtin, builtin_key, builtin_names};

/// A cachix cache with a known public key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cache {
    name: String,
    key: &'static str,
}

impl Cache {
    /// The cache `name` of the row at `line`.
    ///
    /// # Errors
    ///
    /// A name not in the built-in table, naming it and the known ones.
    pub fn parse(line: usize, name: &str) -> Result<Self, KindError> {
        builtin_key(name)
            .map(|key| Self {
                name: name.to_owned(),
                key,
            })
            .ok_or_else(|| unknown(line, name))
    }

    /// The cache name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The public key, `<name>.cachix.org-1:<key>`.
    #[must_use]
    pub fn key(&self) -> &'static str {
        self.key
    }

    /// The nix config lines that trust the cache.
    #[must_use]
    pub fn nix_config(&self) -> String {
        format!(
            "extra-substituters = https://{}.cachix.org\n\
             extra-trusted-public-keys = {}\n",
            self.name, self.key
        )
    }
}

/// The caches a row's cache cell `name` gives, as
/// `(row.cache, checked.cache)`: a `## caches` row is only the second;
/// a built-in cache is both, so [`Row::check`](super::Row::check) sees
/// what 0.2.0 saw (`src/cache:V82`).
pub(super) fn resolve(
    line: usize,
    name: &str,
    caches: &Caches,
) -> Result<(Option<Cache>, Option<BinaryCache>), KindError> {
    if caches.has_row(name) {
        return Ok((None, caches.find(name)));
    }
    let old = Cache::parse(line, name).map_err(|e| {
        if caches.is_empty() {
            e
        } else {
            missing(line, name, caches)
        }
    })?;
    Ok((Some(old), builtin(name)))
}

/// An unknown cache, with the known names as the fix.
fn unknown(line: usize, name: &str) -> KindError {
    KindError::new(
        line,
        format!("cache `{name}` has no known public key"),
        format!(
            "use one of: {}, or `-` for no cache",
            builtin_names().join(", ")
        ),
    )
}

/// A cache in neither the file's `## caches` nor the built-in table.
fn missing(line: usize, name: &str, caches: &Caches) -> KindError {
    KindError::new(
        line,
        format!(
            "cache `{name}` is in neither `## caches` nor the built-in table"
        ),
        format!(
            "add a `## caches` row named `{name}`, use one of: {}, or `-` \
             for no cache",
            caches.names().join(", ")
        ),
    )
}

#[cfg(test)]
mod tests;
