//! The `## caches` section (V82, V83): `name|kind|url|key|push`, one row
//! per cache a `## workflows` row may name, every cell checked here so a
//! forge never refuses the row later.

use super::entry::{
    BinaryCache, CacheKind, Push, builtin, builtin_key, builtin_names,
    cachix_url,
};
use super::value::{CacheValue, https_url, public_key, secret_name, userinfo};
use crate::registry::{
    Document, RegistryError, Row as Cells, SectionSpec, optional, required,
};

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
    /// One cache per row, in file order.
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
        file: &str,
        doc: &Document,
    ) -> Result<Self, RegistryError> {
        let rows = doc.section(SECTION).iter().map(|row| cache(file, row));
        Ok(Self {
            rows: rows.collect::<Result<_, _>>()?,
        })
    }

    /// The cache a `cache` cell names: the row of that name, otherwise
    /// the built-in cachix cache of that name (V82).
    #[must_use]
    pub fn find(&self, name: &str) -> Option<BinaryCache> {
        let row = self.rows.iter().find(|c| c.name() == name).cloned();
        row.or_else(|| builtin(name))
    }

    /// Whether the section has no rows: only the built-in caches.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// Whether a row of the section is named `name`, shadowing any
    /// built-in cache of that name.
    #[must_use]
    pub fn has_row(&self, name: &str) -> bool {
        self.rows.iter().any(|c| c.name() == name)
    }

    /// Every name a `cache` cell may write: the rows', then each
    /// built-in name no row shadows.
    #[must_use]
    pub fn names(&self) -> Vec<&str> {
        let rows = self.rows.iter().map(BinaryCache::name);
        let builtins = builtin_names().into_iter().filter(|n| !self.has_row(n));
        rows.chain(builtins).collect()
    }
}

/// One row, checked for its kind.
fn cache(file: &str, row: &Cells) -> Result<BinaryCache, RegistryError> {
    let name = required(file, row, 0, "name")?;
    let kind = kind(file, row)?;
    let at = Cell { file, row };
    named(at, kind, &name)?;
    let (url, key) = match kind {
        CacheKind::Cachix => {
            (cachix_url_cell(at, &name)?, cachix_key(at, &name)?)
        }
        CacheKind::Https => {
            let key = key_cell(at)?.ok_or_else(|| at.empty(3, "key"))?;
            (https_cell(at)?, key)
        }
    };
    Ok(BinaryCache::new(&name, kind, url, key).pushed(push(at)?))
}

/// The `name` cell's shape for `kind` (V83).
fn named(
    at: Cell<'_>,
    kind: CacheKind,
    name: &str,
) -> Result<(), RegistryError> {
    let (ok, fix) = match kind {
        CacheKind::Cachix => (cachix_name(name), CACHIX_NAME),
        CacheKind::Https => (plain_name(name), PLAIN_NAME),
    };
    if ok {
        return Ok(());
    }
    let what = format!(
        "cache name `{}` does not fit kind `{}`",
        name.escape_debug(),
        kind.name()
    );
    Err(at.error(0, what, fix))
}

/// The fix for a bad cachix name.
const CACHIX_NAME: &str = "a cachix name is lower-case letters, digits and `-`, not starting with `-`";

/// The fix for a bad name of any other kind.
const PLAIN_NAME: &str = "use letters, digits, `.`, `_`, `-`; no leading `-`";

/// `name` is lower-case letters, digits and `-`, not starting with `-`:
/// a cachix cache name, and so one label of its host.
fn cachix_name(name: &str) -> bool {
    let ok = |c: char| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-';
    name.chars().all(ok) && !name.starts_with('-')
}

/// `name` is letters, digits and `._-`, not starting with `-`.
fn plain_name(name: &str) -> bool {
    let ok = |c: char| c.is_ascii_alphanumeric() || "._-".contains(c);
    name.chars().all(ok) && !name.starts_with('-')
}

/// The `kind` cell: a known cache kind.
fn kind(file: &str, row: &Cells) -> Result<CacheKind, RegistryError> {
    let name = required(file, row, 1, "kind")?;
    CacheKind::parse(&name).ok_or_else(|| {
        let names: Vec<&str> =
            CacheKind::ALL.iter().map(|k| k.name()).collect();
        Cell { file, row }.error(
            1,
            format!("cache kind `{}` is unknown", name.escape_debug()),
            format!("use one of: {}", names.join(", ")),
        )
    })
}

/// A cachix `url`: `-` or the url its name gives; never a secret, since
/// the name is the url.
fn cachix_url_cell(
    at: Cell<'_>,
    name: &str,
) -> Result<CacheValue, RegistryError> {
    let want = cachix_url(name);
    match optional(at.row, 2) {
        None => Ok(CacheValue::Literal(want)),
        Some(url) if url == want => Ok(CacheValue::Literal(want)),
        Some(_) => Err(at.error(
            2,
            format!("a cachix cache's url is `{want}`"),
            "write `-` or that url",
        )),
    }
}

/// A cachix `key`: `-` only when the built-in table knows the name.
fn cachix_key(at: Cell<'_>, name: &str) -> Result<CacheValue, RegistryError> {
    match key_cell(at)? {
        Some(key) => Ok(key),
        None => builtin_key(name)
            .map(|k| CacheValue::Literal(k.to_owned()))
            .ok_or_else(|| {
                at.error(
                    3,
                    format!("cachix cache `{name}` has no built-in public key"),
                    "write its key, `<name>.cachix.org-1:<base64>`, or `secret:NAME`",
                )
            }),
    }
}

/// An https `url`: required; a literal is `https://<host>...`.
fn https_cell(at: Cell<'_>) -> Result<CacheValue, RegistryError> {
    let url = value(at, 2)?.ok_or_else(|| at.empty(2, "url"))?;
    match url.literal() {
        Some(text) if userinfo(text) => Err(credential(at)),
        Some(text) if !https_url(text) => Err(not_https(at)),
        _ => Ok(url),
    }
}

/// A literal url with userinfo: refused without echoing it, since the
/// credential would land in the log as well as the file.
fn credential(at: Cell<'_>) -> RegistryError {
    at.error(
        2,
        "url holds a credential before `@`, so it is refused and not echoed",
        "put the credential in the push netrc secret, or write the url as \
         `secret:NAME`",
    )
}

/// A literal url that is not `https://<host>...`: refused without
/// echoing it, since any url may hold a credential (`user:pw@`) whatever
/// its scheme.
fn not_https(at: Cell<'_>) -> RegistryError {
    at.error(
        2,
        "url is not `https://` and a host, so it is refused and not echoed",
        "write `https://<host>...`, or `secret:NAME` to keep it out of the file",
    )
}

/// The `key` cell: `-`, a secret, or a literal public key.
fn key_cell(at: Cell<'_>) -> Result<Option<CacheValue>, RegistryError> {
    let key = value(at, 3)?;
    match key.as_ref().and_then(CacheValue::literal) {
        Some(text) if !public_key(text) => Err(at.error(
            3,
            format!("key `{}` is not a nix public key", text.escape_debug()),
            "write `<keyname>:<base64 of 32 bytes>`, or `secret:NAME`",
        )),
        _ => Ok(key),
    }
}

/// The `push` cell: `-` is read-only; a credential is only a secret.
fn push(at: Cell<'_>) -> Result<Push, RegistryError> {
    match value(at, 4)? {
        None => Ok(Push::ReadOnly),
        Some(CacheValue::Secret(name)) => Ok(Push::Secret(name)),
        Some(CacheValue::Literal(_)) => Err(at.error(
            4,
            "`push` is a literal: a push credential is never written in the config file",
            "write `secret:NAME`, or `-` for read-only",
        )),
    }
}

/// Cell `index` as a value, `None` for `-`; a secret's name is checked.
fn value(
    at: Cell<'_>,
    index: usize,
) -> Result<Option<CacheValue>, RegistryError> {
    let Some(text) = optional(at.row, index) else {
        return Ok(None);
    };
    let value = CacheValue::read(&text);
    match value.secret() {
        Some(name) if !secret_name(name) => Err(at.error(
            index,
            format!("secret `{}` is not a secret name", name.escape_debug()),
            "write `secret:NAME`: letters, digits and `_`, not starting with a digit",
        )),
        _ => Ok(Some(value)),
    }
}

/// A row of `file`, for errors pointing at one of its cells.
#[derive(Clone, Copy)]
struct Cell<'a> {
    file: &'a str,
    row: &'a Cells,
}

impl Cell<'_> {
    /// An error at cell `index`.
    fn error(
        self,
        index: usize,
        what: impl Into<String>,
        fix: impl Into<String>,
    ) -> RegistryError {
        RegistryError {
            file: self.file.to_owned(),
            line: self.row.line,
            column: self.row.cells.get(index).map_or(1, |c| c.column),
            what: what.into(),
            fix: fix.into(),
        }
    }

    /// Cell `index` (column `name`) is `-` where the kind needs a value.
    fn empty(self, index: usize, name: &str) -> RegistryError {
        self.error(
            index,
            format!("an https cache needs its `{name}`"),
            format!("write the {name}, or `secret:NAME`"),
        )
    }
}
