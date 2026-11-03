//! One nix binary cache a row may trust (V82, V84, V85): its name, kind,
//! url, public key and push credential, and the built-in cachix table a
//! name falls back to when no `## caches` row names it.

use super::value::CacheValue;

#[cfg(test)]
mod tests;

/// Public keys of the cachix caches pipelith knows,
/// `<name>.cachix.org-1:<key>`. A key is never guessed.
const KEYS: [&str; 1] =
    ["pr0d1r2.cachix.org-1:NfWjbhgAj41byXhCKiaE+av3Vnphm1fTezHXEGsiQIM="];

/// How a cache is read and pushed to. The column set of `## caches` is
/// the same for every kind; a later kind gives the cells its own meaning.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub enum CacheKind {
    /// A cachix cache: `https://<name>.cachix.org`, pushed with a
    /// cachix auth token.
    Cachix,
    /// Any standard nix binary cache over https, pushed with
    /// `nix copy` and a netrc file.
    Https,
}

impl CacheKind {
    /// Every kind, in `docs/KINDS.md` order.
    pub const ALL: [Self; 2] = [Self::Cachix, Self::Https];

    /// The name a `## caches` row writes in its `kind` cell.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Cachix => "cachix",
            Self::Https => "https",
        }
    }

    /// The kind a `kind` cell names, if any.
    #[must_use]
    pub(crate) fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|k| k.name() == name)
    }
}

/// Whether, and with what, a pipeline pushes to the cache.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Push {
    /// Read only: no push step and no push secret.
    ReadOnly,
    /// The forge's own cache-push secret, as before `## caches` existed
    /// (a built-in cache with no row).
    Forge,
    /// The secret holding the credential: a cachix auth token
    /// ([`CacheKind::Cachix`]) or a netrc file ([`CacheKind::Https`]).
    Secret(String),
}

/// A nix binary cache a checked row trusts: a `## caches` row or a
/// built-in entry. `kind::Cache`, the 0.2.0 type, stays the built-in
/// cachix cache only; this one can hold secrets, so it is a separate
/// type and an old caller never meets one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BinaryCache {
    name: String,
    kind: CacheKind,
    url: CacheValue,
    key: CacheValue,
    push: Push,
}

impl BinaryCache {
    /// A read-only cache from its checked parts.
    #[must_use]
    pub(crate) fn new(
        name: &str,
        kind: CacheKind,
        url: CacheValue,
        key: CacheValue,
    ) -> Self {
        Self {
            name: name.to_owned(),
            kind,
            url,
            key,
            push: Push::ReadOnly,
        }
    }

    /// The same cache, pushed with `push`.
    #[must_use]
    pub(crate) fn pushed(self, push: Push) -> Self {
        Self { push, ..self }
    }

    /// The name a `cache` cell writes: the cachix cache name for
    /// [`CacheKind::Cachix`].
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The cache's kind.
    #[must_use]
    pub fn kind(&self) -> CacheKind {
        self.kind
    }

    /// The url nix substitutes from.
    #[must_use]
    pub fn url(&self) -> &CacheValue {
        &self.url
    }

    /// The public key nix trusts, `<keyname>:<base64>`.
    #[must_use]
    pub fn key(&self) -> &CacheValue {
        &self.key
    }

    /// How the pipeline pushes.
    #[must_use]
    pub fn push(&self) -> &Push {
        &self.push
    }

    /// Whether a pipeline may push here (V85).
    #[must_use]
    pub fn can_push(&self) -> bool {
        self.push != Push::ReadOnly
    }

    /// The names of the secrets reading needs, url's then key's (V84);
    /// empty when both are literals. `kind::Purpose` has no variant for
    /// them, so a forge lists these beside `KindSpec::purposes`.
    #[must_use]
    pub fn read_secrets(&self) -> Vec<&str> {
        [&self.url, &self.key]
            .into_iter()
            .filter_map(CacheValue::secret)
            .collect()
    }

    /// The nix settings that trust the cache, in order, each with its
    /// value: the forge writes a literal as is and a secret its own way.
    #[must_use]
    pub fn nix_settings(&self) -> [(&'static str, &CacheValue); 2] {
        [
            ("extra-substituters", &self.url),
            ("extra-trusted-public-keys", &self.key),
        ]
    }

    /// The nix config lines that trust the cache, when the url and key
    /// are both literals; `None` when either is held by a secret.
    #[must_use]
    pub fn nix_config(&self) -> Option<String> {
        let line = |(setting, value): (&str, &CacheValue)| {
            value.literal().map(|v| format!("{setting} = {v}\n"))
        };
        self.nix_settings().into_iter().map(line).collect()
    }
}

/// The built-in cachix cache `name`: its url and key literal, pushed
/// with the forge's own secret (V82).
#[must_use]
pub fn builtin(name: &str) -> Option<BinaryCache> {
    builtin_key(name).map(|key| {
        let url = CacheValue::Literal(cachix_url(name));
        let key = CacheValue::Literal(key.to_owned());
        BinaryCache::new(name, CacheKind::Cachix, url, key).pushed(Push::Forge)
    })
}

/// The names of the built-in cachix caches, in table order.
#[must_use]
pub fn builtin_names() -> Vec<&'static str> {
    KEYS.iter().filter_map(|k| k.split('.').next()).collect()
}

/// The public key of the built-in cachix cache `name`.
#[must_use]
pub(crate) fn builtin_key(name: &str) -> Option<&'static str> {
    let prefix = format!("{name}.cachix.org-1:");
    KEYS.into_iter().find(|k| k.starts_with(&prefix))
}

/// The url of the cachix cache `name`.
#[must_use]
pub(crate) fn cachix_url(name: &str) -> String {
    format!("https://{name}.cachix.org")
}
