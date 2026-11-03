//! One `## caches` cell value (V83): a literal, or `secret:NAME` naming
//! a forge secret, and the shape checks a literal url or key passes.

#[cfg(test)]
mod tests;

/// The prefix of a cell that names a secret instead of a literal.
pub const SECRET_PREFIX: &str = "secret:";

/// A cell value: in the config file as is, or held by a secret.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CacheValue {
    /// Written in the config file as is.
    Literal(String),
    /// The name of the forge secret (or CI/CD variable) holding the
    /// value; the value itself is never in the config file.
    Secret(String),
}

impl CacheValue {
    /// The cell `text`: `secret:NAME` names a secret, anything else is a
    /// literal. The name is not checked here ([`secret_name`]).
    #[must_use]
    #[allow(dead_code)] // RED: wired in by the GREEN commit
    pub(crate) fn read(text: &str) -> Self {
        Self::Literal(text.to_owned())
    }

    /// The literal, if the value is one.
    #[must_use]
    pub fn literal(&self) -> Option<&str> {
        match self {
            Self::Literal(text) => Some(text),
            Self::Secret(_) => None,
        }
    }

    /// The secret's name, if the value is held by one.
    #[must_use]
    pub fn secret(&self) -> Option<&str> {
        match self {
            Self::Secret(name) => Some(name),
            Self::Literal(_) => None,
        }
    }
}

/// `name` is a secret name. RED: accepts anything.
#[allow(dead_code)] // RED: wired in by the GREEN commit
#[must_use]
pub(crate) fn secret_name(_name: &str) -> bool {
    true
}

/// `url` is an https url. RED: accepts anything.
#[allow(dead_code)] // RED: wired in by the GREEN commit
#[must_use]
pub(crate) fn https_url(_url: &str) -> bool {
    true
}

/// `key` is a nix public key. RED: accepts anything.
#[allow(dead_code)] // RED: wired in by the GREEN commit
#[must_use]
pub(crate) fn public_key(_key: &str) -> bool {
    true
}
