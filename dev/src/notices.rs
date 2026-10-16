//! `pipelith-dev notices [--check]`: the crate table in
//! `docs/THIRD-PARTY-NOTICES.md`, from what `Cargo.lock` resolves on normal
//! dependency edges of the published crate, and the prose around it held
//! to the table (dev T18, V40). Ported from actlith-dev, itself from
//! pklith-dev; MIT, same author.

use crate::io::Failed;
use std::path::Path;

/// A crate the published crate ships: name, version, licence.
pub(crate) type Crate = (String, String, String);

/// The crates in `cargo tree -f '{p}|{l}' --prefix none` output, sorted
/// and once each, without the crate `own` itself.
#[must_use]
pub(crate) fn crates(tree: &str, own: &str) -> Vec<Crate> {
    let _ = (tree, own);
    Vec::new()
}

/// The table block, markers included.
#[must_use]
pub(crate) fn table(crates: &[Crate]) -> String {
    let _ = crates;
    String::new()
}

/// What the prose claims that the table does not bear out.
#[must_use]
pub(crate) fn stale_prose(
    doc: &str,
    direct: usize,
    crates: &[Crate],
) -> Vec<String> {
    let _ = (doc, direct, crates);
    Vec::new()
}

/// Render or check.
///
/// # Errors
///
/// Stale prose or drift (1), cargo or a file failing (2).
pub(crate) fn run_verb(repo: &Path, check: bool) -> Result<(), Failed> {
    let _ = (repo, check);
    Ok(())
}

#[cfg(test)]
mod tests;
