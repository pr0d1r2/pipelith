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
    pub(crate) fn read(text: &str) -> Self {
        text.strip_prefix(SECRET_PREFIX).map_or_else(
            || Self::Literal(text.to_owned()),
            |name| Self::Secret(name.to_owned()),
        )
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

/// `name` is letters, digits and `_`, not starting with a digit: a
/// secret name on every forge.
#[must_use]
pub(crate) fn secret_name(name: &str) -> bool {
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let first = name.chars().next();
    first.is_some_and(|c| !c.is_ascii_digit()) && name.chars().all(word)
}

/// `url` is `https://` and a host, with no blank or control character:
/// one item of nix's `substituters`.
#[must_use]
pub(crate) fn https_url(url: &str) -> bool {
    let clean = !url.chars().any(|c| c.is_whitespace() || c.is_control());
    let host = url.strip_prefix("https://").and_then(|rest| {
        rest.split(['/', '?', '#']).next().filter(|h| !h.is_empty())
    });
    clean && host.is_some()
}

/// `key` is `<keyname>:<base64>`, the base64 being 32 bytes in canonical
/// padding: an ed25519 public key as nix's `trusted-public-keys` takes it.
#[must_use]
pub(crate) fn public_key(key: &str) -> bool {
    let Some((name, data)) = key.split_once(':') else {
        return false;
    };
    let bad = |c: char| c.is_whitespace() || c.is_control();
    !name.is_empty() && !name.chars().any(bad) && ed25519(data)
}

/// `data` is 32 bytes of standard base64: 43 symbols, the last with its
/// two low bits clear, then one `=`.
fn ed25519(data: &str) -> bool {
    const LAST: &str = "AEIMQUYcgkosw048";
    let symbol = |c: char| c.is_ascii_alphanumeric() || matches!(c, '+' | '/');
    let Some(body) = data.strip_suffix('=') else {
        return false;
    };
    body.len() == 43
        && body.chars().all(symbol)
        && body.chars().last().is_some_and(|c| LAST.contains(c))
}
