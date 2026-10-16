//! The row's cachix cache: its name and public key, from the built-in
//! table only (V53). cachix serves every forge, so this is neutral.

use super::error::KindError;

/// Public keys of the caches pipelith knows, `<name>.cachix.org-1:<key>`.
/// A row naming any other cache is refused: a key is never guessed.
const KEYS: [&str; 1] =
    ["pr0d1r2.cachix.org-1:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM="];

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
        let _ = (line, name, KEYS);
        Err(KindError::default())
    }

    /// The cache name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The public key, `<name>.cachix.org-1:<key>`.
    #[must_use]
    pub fn key(&self) -> &'static str {
        let _ = self;
        ""
    }

    /// The nix config lines that trust the cache.
    #[must_use]
    pub fn nix_config(&self) -> String {
        let _ = (&self.name, self.key);
        String::new()
    }
}

#[cfg(test)]
mod tests;
