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
        let prefix = format!("{name}.cachix.org-1:");
        KEYS.into_iter()
            .find(|k| k.starts_with(&prefix))
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

/// An unknown cache, with the known names as the fix.
fn unknown(line: usize, name: &str) -> KindError {
    let names: Vec<&str> =
        KEYS.iter().filter_map(|k| k.split('.').next()).collect();
    KindError::new(
        line,
        format!("cache `{name}` has no known public key"),
        format!("use one of: {}, or `-` for no cache", names.join(", ")),
    )
}

#[cfg(test)]
mod tests;
